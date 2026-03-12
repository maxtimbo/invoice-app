use crate::models::ids::CompanyId;
use crate::models::contact::Contact;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: CompanyId,
    pub name: String,
    pub logo: Option<Vec<u8>>,
    pub contact: Contact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyList {
    pub id: CompanyId,
    pub name: String,
}

