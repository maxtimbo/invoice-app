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
