use crate::models::ids::TemplateId;
use crate::models::client::Client;
use crate::models::company::Company;
use crate::models::terms::Terms;
use crate::models::method::Method;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: TemplateId,
    pub name: String,
    pub company: Company,
    pub client: Client,
    pub terms: Terms,
    pub method: Vec<Method>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSummary {
    pub id: TemplateId,
    pub name: String,
}
