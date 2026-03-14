use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};
use axum::extract::Multipart;
use invoice_app::ports::repos::company_repo::{CompanyRepo, CreateCompany, UpdateCompany};
use invoice_core::models::ids::CompanyId;
use invoice_core::models::contact::Contact;
use crate::state::AppState;
use super::CompanyForm;

type S = Arc<AppState>;

async fn parse_multipart(mut multipart: Multipart) -> CompanyForm {
    let mut name = String::new();
    let mut logo: Option<Vec<u8>> = None;
    let mut phone = None;
    let mut email = None;
    let mut addr1 = None;
    let mut addr2 = None;
    let mut city = None;
    let mut state = None;
    let mut zip = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap_or("") {
            "name"  => name  = field.text().await.unwrap(),
            "phone" => phone = some_if_not_empty(field.text().await.unwrap()),
            "email" => email = some_if_not_empty(field.text().await.unwrap()),
            "addr1" => addr1 = some_if_not_empty(field.text().await.unwrap()),
            "addr2" => addr2 = some_if_not_empty(field.text().await.unwrap()),
            "city"  => city  = some_if_not_empty(field.text().await.unwrap()),
            "state" => state = some_if_not_empty(field.text().await.unwrap()),
            "zip"   => zip   = some_if_not_empty(field.text().await.unwrap()),
            "logo"  => {
                let bytes = field.bytes().await.unwrap();
                if !bytes.is_empty() {
                    logo = Some(bytes.to_vec());
                }
            }
            _ => {}
        }
    }

    CompanyForm {
        name,
        logo,
        contact: Contact { phone, email, addr1, addr2, city, state, zip },
    }
}

fn some_if_not_empty(s: String) -> Option<String> {
    if s.trim().is_empty() { None } else { Some(s) }
}

pub async fn create(State(s): State<S>, multipart: Multipart) -> impl IntoResponse {
    let form = parse_multipart(multipart).await;
    s.db.create_company(CreateCompany {
        name: form.name,
        logo: form.logo,
        contact: form.contact,
    }).await.unwrap();
    Redirect::to("/companies")
}

pub async fn update(
    State(s): State<S>,
    Path(id): Path<i64>,
    multipart: Multipart,
) -> impl IntoResponse {
    let form = parse_multipart(multipart).await;

    // Fetch existing logo if no new one was uploaded
    let logo = if form.logo.is_some() {
        form.logo
    } else {
        s.db.get_company(CompanyId(id)).await.unwrap()
            .and_then(|c| c.logo)
    };

    s.db.update_company(CompanyId(id), UpdateCompany {
        name: Some(form.name),
        logo,
        contact: Some(form.contact),
    }).await.unwrap();
    Redirect::to("/companies")
}

pub async fn delete(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    s.db.delete_company(CompanyId(id)).await.unwrap();
    Redirect::to("/companies")
}
