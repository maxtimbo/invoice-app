mod views;
mod actions;

pub use views::*;
pub use actions::*;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct TemplateForm {
    pub name: String,
    pub company_id: i64,
    pub client_id: i64,
    pub terms_id: i64,
    #[serde(default)]
    pub method_ids: Vec<i64>,
}
