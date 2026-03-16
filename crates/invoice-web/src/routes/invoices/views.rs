use std::sync::Arc;
use std::collections::HashMap;
use axum::{
    extract::{Path, State, Query},
    response::{Html, IntoResponse, Response},
    body::Body,
    http::{header, StatusCode},
};
use tera::Context;
use invoice_app::ports::repos::invoice_repo::InvoiceRepo;
use invoice_app::ports::repos::template_repo::TemplateRepo;
use invoice_app::ports::repos::item_repo::ItemRepo;
use invoice_app::render::TemplateEngine;
use invoice_app::render::view::InvoiceView;
use invoice_app::commands::paths::Paths;
use invoice_core::models::ids::InvoiceId;
use invoice_core::models::status::PaidStatus;
use invoice_core::models::stage::InvoiceStage;
use crate::state::AppState;
use super::{InvoiceSummaryView, InvoiceEditView, InvoiceItemView};

type S = Arc<AppState>;

pub async fn view(
    State(s): State<S>,
    Path(id): Path<i64>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let invoice = s.db.get_invoice(InvoiceId(id)).await.unwrap().unwrap();
    let view = InvoiceView::from(&invoice);
    let mut ctx = Context::new();
    ctx.insert("inv", &view);
    ctx.insert("sent", &params.get("sent").map(|s| s.as_str()).unwrap_or(""));
    ctx.insert("sent_msg", &params.get("msg").map(|s| s.replace('+', " ")));
    Html(s.tera.render("invoices/view.html", &ctx).unwrap())
}

pub async fn print(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    let invoice = s.db.get_invoice(InvoiceId(id)).await.unwrap().unwrap();
    let paths = Paths::init().unwrap();
    let engine = TemplateEngine::new(&paths.templates).unwrap();
    let html = engine.render(&invoice).unwrap();
    let pdf = engine.to_pdf(&html).unwrap();
    let filename = format!("invoice_{:04}.pdf", id);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, format!("inline; filename=\"{filename}\""))
        .body(Body::from(pdf))
        .unwrap()
}

pub async fn list(
    State(s): State<S>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let show_archived = params.get("archived").map(|v| v == "1").unwrap_or(false);
    let summaries = s.db.list_invoice_summary(show_archived).await.unwrap();
    let views: Vec<InvoiceSummaryView> = summaries.into_iter().map(|s| {
        let (status, status_date, _) = flatten_status(&s.status);
        let status_str = match status_date {
            Some(d) => format!("{status} ({d})"),
            None    => status.to_string(),
        };
        InvoiceSummaryView {
            id:           s.id.0,
            client_name:  s.client_name,
            issued:       s.issued.format("%Y-%m-%d").to_string(),
            due:          s.due.format("%Y-%m-%d").to_string(),
            status:       status_str,
            total:        format!("{:.2}", s.total.inner()),
            message_sent: s.message_sent.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default(),
            archived:     s.archived,
        }
    }).collect();
    let mut ctx = Context::new();
    ctx.insert("invoices", &views);
    ctx.insert("show_archived", &show_archived);
    Html(s.tera.render("invoices/list.html", &ctx).unwrap())
}

pub async fn new_form(State(s): State<S>) -> impl IntoResponse {
    use serde::Serialize;
    #[derive(Serialize)]
    struct ItemOption { id: i64, name: String, rate: String }

    let templates = s.db.list_template().await.unwrap();
    let items: Vec<ItemOption> = s.db.list_item().await.unwrap()
        .into_iter()
        .map(|item| ItemOption {
            id: item.id.0,
            name: item.name,
            rate: format!("{:.2}", item.rate.inner()),
        })
        .collect();

    let mut ctx = Context::new();
    ctx.insert("templates", &templates);
    ctx.insert("items", &items);
    Html(s.tera.render("invoices/form.html", &ctx).unwrap())
}

pub async fn edit_form(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    let invoice = s.db.get_invoice(InvoiceId(id)).await.unwrap().unwrap();
    let all_items = s.db.list_item().await.unwrap();
    let subtotals = invoice.calculate_subtotals();
    let current_item_ids: Vec<i64> = subtotals.iter().map(|d| d.id.0).collect();
    let total = format!("{:.2}", invoice.calculate_total().inner());
    let (status, status_date, status_check) = flatten_status(&invoice.attributes.status);

    let view = InvoiceEditView {
        id: invoice.id.0,
        date: invoice.date.format("%Y-%m-%d").to_string(),
        template_id: invoice.template.id.0,
        client_name: invoice.template.client.name.clone(),
        company_name: invoice.template.company.name.clone(),
        show_methods: invoice.attributes.show_methods,
        show_notes: invoice.attributes.show_notes,
        stage: match invoice.attributes.stage {
            InvoiceStage::Quote => "Quote".into(),
            InvoiceStage::Invoice => "Invoice".into(),
        },
        status: status.to_string(),
        status_date,
        status_check,
        notes: invoice.notes.clone(),
        items: subtotals.iter().map(|d| InvoiceItemView {
            id: d.id.0,
            name: d.name.clone(),
            rate: format!("{:.2}", d.rate.inner()),
            quantity: d.quantity.inner().to_string(),
            subtotal: format!("{:.2}", d.subtotal.inner()),
        }).collect(),
        total,
        message_sent: invoice.message_sent.map(|d| d.format("%Y-%m-%d").to_string()),
    };

    let mut ctx = Context::new();
    ctx.insert("invoice", &view);
    ctx.insert("all_items", &all_items);
    ctx.insert("current_item_ids", &current_item_ids);
    Html(s.tera.render("invoices/edit.html", &ctx).unwrap())
}

pub fn flatten_status(status: &PaidStatus) -> (&'static str, Option<String>, Option<String>) {
    match status {
        PaidStatus::Waiting            => ("Waiting", None, None),
        PaidStatus::PastDue            => ("PastDue", None, None),
        PaidStatus::Paid { date, check } => (
            "Paid",
            Some(date.format("%Y-%m-%d").to_string()),
            check.clone(),
        ),
        PaidStatus::Failed { date }    => ("Failed",   Some(date.format("%Y-%m-%d").to_string()), None),
        PaidStatus::Refunded { date }  => ("Refunded", Some(date.format("%Y-%m-%d").to_string()), None),
    }
}
