use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Form,
};
use invoice_app::ports::repos::template_repo::{TemplateRepo, CreateTemplate, UpdateTemplate};
use invoice_core::models::ids::{TemplateId, CompanyId, ClientId, TermsId, MethodId};
use crate::state::AppState;
use super::TemplateForm;

type S = Arc<AppState>;

pub async fn create(State(s): State<S>, Form(input): Form<TemplateForm>) -> impl IntoResponse {
    s.db.create_template(CreateTemplate {
        name:    input.name,
        company: CompanyId(input.company_id),
        client:  ClientId(input.client_id),
        terms:   TermsId(input.terms_id),
        method:  input.method_ids.into_iter().map(MethodId).collect(),
    }).await.unwrap();
    Redirect::to("/templates")
}

pub async fn update(
    State(s): State<S>,
    Path(id): Path<i64>,
    Form(input): Form<TemplateForm>,
) -> impl IntoResponse {
    s.db.update_template(TemplateId(id), UpdateTemplate {
        name:    Some(input.name),
        company: Some(CompanyId(input.company_id)),
        client:  Some(ClientId(input.client_id)),
        terms:   Some(TermsId(input.terms_id)),
        method:  Some(input.method_ids.into_iter().map(MethodId).collect()),
    }).await.unwrap();
    Redirect::to("/templates")
}

pub async fn delete(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    s.db.delete_template(TemplateId(id)).await.unwrap();
    Redirect::to("/templates")
}
