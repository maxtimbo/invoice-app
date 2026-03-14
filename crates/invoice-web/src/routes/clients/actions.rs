use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Form,
};
use invoice_app::ports::repos::client_repo::{ClientRepo, CreateClient, UpdateClient};
use invoice_core::models::ids::ClientId;
use crate::state::AppState;
use super::ClientForm;

type S = Arc<AppState>;

pub async fn create(State(s): State<S>, Form(input): Form<ClientForm>) -> impl IntoResponse {
    let (name, contact) = input.into_contact();
    s.db.create_client(CreateClient { name, contact }).await.unwrap();
    Redirect::to("/clients")
}

pub async fn update(
    State(s): State<S>,
    Path(id): Path<i64>,
    Form(input): Form<ClientForm>,
) -> impl IntoResponse {
    let (name, contact) = input.into_contact();
    s.db.update_client(ClientId(id), UpdateClient {
        name: Some(name),
        contact: Some(contact),
    }).await.unwrap();
    Redirect::to("/clients")
}

pub async fn delete(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    s.db.delete_client(ClientId(id)).await.unwrap();
    Redirect::to("/clients")
}
