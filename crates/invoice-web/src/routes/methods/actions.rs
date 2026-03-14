use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};
use axum::extract::Multipart;
use invoice_app::ports::repos::method_repo::{MethodRepo, CreateMethod, UpdateMethod};
use invoice_core::models::ids::MethodId;
use crate::state::AppState;
use super::MethodForm;

type S = Arc<AppState>;

async fn parse_multipart(mut multipart: Multipart) -> MethodForm {
    let mut name = String::new();
    let mut link = None;
    let mut qr = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap_or("") {
            "name" => name = field.text().await.unwrap(),
            "link" => link = some_if_not_empty(field.text().await.unwrap()),
            "qr"   => {
                let bytes = field.bytes().await.unwrap();
                if !bytes.is_empty() {
                    qr = Some(bytes.to_vec());
                }
            }
            _ => {}
        }
    }

    MethodForm { name, link, qr }
}

fn some_if_not_empty(s: String) -> Option<String> {
    if s.trim().is_empty() { None } else { Some(s) }
}

pub async fn create(State(s): State<S>, multipart: Multipart) -> impl IntoResponse {
    let form = parse_multipart(multipart).await;
    s.db.create_method(CreateMethod {
        name: form.name,
        link: form.link,
        qr: form.qr,
    }).await.unwrap();
    Redirect::to("/methods")
}

pub async fn update(
    State(s): State<S>,
    Path(id): Path<i64>,
    multipart: Multipart,
) -> impl IntoResponse {
    let form = parse_multipart(multipart).await;

    // Keep existing QR if no new one uploaded
    let qr = if form.qr.is_some() {
        form.qr
    } else {
        s.db.get_method(MethodId(id)).await.unwrap()
            .and_then(|m| m.qr)
    };

    s.db.update_method(MethodId(id), UpdateMethod {
        name: Some(form.name),
        link: form.link,
        qr,
    }).await.unwrap();
    Redirect::to("/methods")
}

pub async fn delete(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    s.db.delete_method(MethodId(id)).await.unwrap();
    Redirect::to("/methods")
}
