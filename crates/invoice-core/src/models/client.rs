use crate::models::ids::ClientId;
use crate::models::contact::Contact;

#[derive(Debug, Clone)]
pub struct Client {
    pub id: ClientId,
    pub name: String,
    pub contact: Contact,
}
