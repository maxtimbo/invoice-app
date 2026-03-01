use crate::models::ids::TemplateId;
use crate::models::client::Client;
use crate::models::company::Company;
use crate::models::terms::Terms;
use crate::models::methods::Methods;

#[derive(Debug, Clone)]
pub struct Template {
    pub id: TemplateId,
    pub name: String,
    pub company: Company,
    pub client: Client,
    pub terms: Terms,
    pub methods: Vec<Methods>,
}
