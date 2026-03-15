use std::sync::Arc;
use std::collections::HashMap;
use std::str::FromStr;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Form,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use invoice_app::ports::repos::invoice_repo::{InvoiceRepo, CreateInvoice, UpdateInvoice};
use invoice_app::ports::repos::config_repo::ConfigRepo;
use invoice_core::models::ids::{InvoiceId, ItemId, TemplateId};
use invoice_core::models::attributes::InvoiceAttrs;
use invoice_core::models::stage::InvoiceStage;
use invoice_core::models::status::PaidStatus;
use invoice_core::models::quantity::Quantity;
use crate::state::AppState;
use super::UpdateInvoiceForm;

type S = Arc<AppState>;

pub async fn create(
    State(s): State<S>,
    Form(input): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let template_id = input["template_id"].parse::<i64>().unwrap();
    let date = NaiveDate::parse_from_str(&input["date"], "%Y-%m-%d").unwrap();
    let show_methods = input.contains_key("show_methods");
    let show_notes   = input.contains_key("show_notes");
    let stage = match input.get("stage").map(|s| s.as_str()) {
        Some("Quote") => InvoiceStage::Quote,
        _             => InvoiceStage::Invoice,
    };
    let notes = input.get("notes")
        .filter(|s| !s.trim().is_empty())
        .cloned();

    let mut items = Vec::new();
    for (key, val) in &input {
        if let Some(id_str) = key.strip_prefix("qty_") {
            if val.trim().is_empty() { continue; }
            if let (Ok(id), Ok(dec)) = (id_str.parse::<i64>(), Decimal::from_str(val)) {
                if let Ok(qty) = Quantity::new(dec) {
                    items.push((ItemId(id), qty));
                }
            }
        }
    }

    s.db.create_invoice(CreateInvoice {
        template: TemplateId(template_id),
        date,
        attributes: InvoiceAttrs {
            show_methods,
            show_notes,
            stage,
            status: PaidStatus::Waiting,
        },
        notes,
        items,
    }).await.unwrap();
    Redirect::to("/invoices")
}

pub async fn update(
    State(s): State<S>,
    Path(id): Path<i64>,
    Form(input): Form<UpdateInvoiceForm>,
) -> impl IntoResponse {
    let stage = match input.stage.as_str() {
        "Quote" => InvoiceStage::Quote,
        _       => InvoiceStage::Invoice,
    };

    let status = parse_status_form(
        &input.status,
        input.status_date.as_deref(),
        input.status_check.as_deref(),
    );

    let notes = input.notes.filter(|s| !s.trim().is_empty());

    s.db.update_invoice(InvoiceId(id), UpdateInvoice {
        show_methods: Some(input.show_methods.is_some()),
        show_notes:   Some(input.show_notes.is_some()),
        stage:        Some(stage),
        status:       Some(status),
        notes,
        ..Default::default()
    }).await.unwrap();
    Redirect::to("/invoices")
}

pub async fn delete(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    s.db.delete_invoice(InvoiceId(id)).await.unwrap();
    Redirect::to("/invoices")
}

fn parse_status_form(
    status: &str,
    date: Option<&str>,
    check: Option<&str>,
) -> PaidStatus {
    let parse_date = |d: Option<&str>| {
        d.and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
         .unwrap_or_else(|| chrono::Local::now().date_naive())
    };

    match status {
        "PastDue"  => PaidStatus::PastDue,
        "Paid"     => PaidStatus::Paid {
            date: parse_date(date),
            check: check.filter(|s| !s.trim().is_empty()).map(str::to_string),
        },
        "Failed"   => PaidStatus::Failed   { date: parse_date(date) },
        "Refunded" => PaidStatus::Refunded  { date: parse_date(date) },
        _          => PaidStatus::Waiting,
    }
}

pub async fn add_item(
    State(s): State<S>,
    Path(id): Path<i64>,
    Form(input): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let item_id = input["item_id"].parse::<i64>().unwrap();
    let qty_str = input.get("quantity").map(|s| s.as_str()).unwrap_or("1");
    let dec = Decimal::from_str(qty_str).unwrap_or(Decimal::ONE);
    let qty = Quantity::new(dec).unwrap_or(Quantity::new(Decimal::ONE).unwrap());

    // fetch existing items and append
    let invoice = s.db.get_invoice(InvoiceId(id)).await.unwrap().unwrap();
    let mut items: Vec<(ItemId, Quantity)> = invoice.items
        .into_iter()
        .map(|(item, qty)| (item.id, qty))
        .collect();

    // replace if already present, otherwise push
    if let Some(existing) = items.iter_mut().find(|(i, _)| i.0 == item_id) {
        existing.1 = qty;
    } else {
        items.push((ItemId(item_id), qty));
    }

    s.db.update_invoice(InvoiceId(id), UpdateInvoice {
        items: Some(items),
        ..Default::default()
    }).await.unwrap();

    Redirect::to(&format!("/invoices/{id}"))
}

pub async fn remove_item(
    State(s): State<S>,
    Path((id, item_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let invoice = s.db.get_invoice(InvoiceId(id)).await.unwrap().unwrap();
    let items: Vec<(ItemId, Quantity)> = invoice.items
        .into_iter()
        .filter(|(item, _)| item.id.0 != item_id)
        .map(|(item, qty)| (item.id, qty))
        .collect();

    s.db.update_invoice(InvoiceId(id), UpdateInvoice {
        items: Some(items),
        ..Default::default()
    }).await.unwrap();

    Redirect::to(&format!("/invoices/{id}"))
}

pub async fn send_email(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    use invoice_app::render::TemplateEngine;
    use invoice_app::services::email_service::EmailService;
    use invoice_app::commands::paths::Paths;
    use chrono::Local;

    let invoice = s.db.get_invoice(InvoiceId(id)).await.unwrap().unwrap();
    let config  = s.db.get_config().await.unwrap().unwrap();
    let paths   = Paths::init().unwrap();
    let engine  = TemplateEngine::new(&paths.templates).unwrap();
    let html    = engine.render(&invoice).unwrap();
    let pdf     = engine.to_pdf(&html).unwrap();
    let filename = format!("invoice_{:04}.pdf", id);

    match EmailService::send(&config, &invoice, html, pdf, filename).await {
        Ok(_)  => {
            s.db.update_invoice(InvoiceId(id), UpdateInvoice {
                message_sent: Some(Local::now().date_naive()),
                ..Default::default()
            }).await.unwrap();
        },
        Err(e) => {
            eprintln!("Email error: {e}");
        }
    }
    Redirect::to(&format!("/invoices/{id}/view"))
}
