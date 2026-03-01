use crate::models::ids::CompanyId;
use crate::models::contact::Contact;

#[derive(Debug, Clone)]
pub struct Company {
    pub id: CompanyId,
    pub name: String,
    pub logo: Option<Vec<u8>>,
    pub contact: Contact,
}
