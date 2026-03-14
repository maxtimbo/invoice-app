use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use tera::Context;
use invoice_app::ports::repos::method_repo::MethodRepo;
use invoice_core::models::ids::MethodId;
use crate::state::AppState;
use super::MethodView;

type S = Arc<AppState>;

pub async fn list(State(s): State<S>) -> impl IntoResponse {
    let methods = s.db.list_method().await.unwrap();
    let views: Vec<MethodView> = methods.into_iter().map(MethodView::from).collect();
    let mut ctx = Context::new();
    ctx.insert("methods", &views);
    Html(s.tera.render("methods/list.html", &ctx).unwrap())
}

pub async fn new_form(State(s): State<S>) -> impl IntoResponse {
    Html(s.tera.render("methods/form.html", &Context::new()).unwrap())
}

pub async fn edit_form(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    let method = s.db.get_method(MethodId(id)).await.unwrap().map(MethodView::from);
    let mut ctx = Context::new();
    ctx.insert("method", &method);
    Html(s.tera.render("methods/form.html", &ctx).unwrap())
}
