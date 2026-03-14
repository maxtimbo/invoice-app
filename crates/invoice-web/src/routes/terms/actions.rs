use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Form,
};
use invoice_app::ports::repos::terms_repo::{TermsRepo, CreateTerms, UpdateTerms};
use invoice_core::models::ids::TermsId;
use crate::state::AppState;
use super::TermsForm;

type S = Arc<AppState>;

pub async fn create(State(s): State<S>, Form(input): Form<TermsForm>) -> impl IntoResponse {
    s.db.create_terms(CreateTerms {
        name: input.name,
        due: input.due,
    }).await.unwrap();
    Redirect::to("/terms")
}

pub async fn update(
    State(s): State<S>,
    Path(id): Path<i64>,
    Form(input): Form<TermsForm>,
) -> impl IntoResponse {
    s.db.update_terms(TermsId(id), UpdateTerms {
        name: Some(input.name),
        due: Some(input.due),
    }).await.unwrap();
    Redirect::to("/terms")
}

pub async fn delete(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    s.db.delete_terms(TermsId(id)).await.unwrap();
    Redirect::to("/terms")
}
