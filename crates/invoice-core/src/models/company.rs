use crate::models::contact::Contact;

pub struct Company {
    pub id: i64,
    pub name: String,
    pub logo: Option<Vec<u8>>,
    pub contact: Contact,
}
