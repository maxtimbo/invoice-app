use crate::models::ids::{
    CompanyId,
    ClientId,
    TermsId,
    MethodId,
    TemplateId
};

#[derive(Debug, Clone)]
pub struct TemplateSkel {
    pub id: TemplateId,
    pub name: String,
    pub company_id: CompanyId,
    pub client_id: ClientId,
    pub terms_id: TermsId,
    pub method_ids: Vec<MethodId>,
}
