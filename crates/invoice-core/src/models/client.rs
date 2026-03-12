use crate::models::ids::ClientId;
use crate::models::contact::Contact;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: ClientId,
    pub name: String,
    pub contact: Contact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientList {
    pub id: ClientId,
    pub name: String,
}
