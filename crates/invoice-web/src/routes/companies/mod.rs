mod views;
mod actions;

pub use views::*;
pub use actions::*;

use serde::{Serialize, Deserialize};
use base64::{engine::general_purpose, Engine as _};
use invoice_core::models::company::Company;
use invoice_core::models::contact::Contact;

#[derive(Serialize, Deserialize)]
pub struct CompanyView {
    pub id: i64,
    pub name: String,
    pub logo: Option<String>,
    pub contact: Contact,
}

impl From<Company> for CompanyView {
    fn from(c: Company) -> Self {
        Self {
            id: c.id.0,
            name: c.name,
            logo: c.logo.as_deref().map(|bytes| {
                let mime = detect_mime(bytes);
                format!("data:{mime};base64,{}", general_purpose::STANDARD.encode(bytes))
            }),
            contact: c.contact,
        }
    }
}

fn detect_mime(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") { "image/png" }
    else if bytes.starts_with(b"\xFF\xD8\xFF") { "image/jpeg" }
    else if bytes.len() >= 12 && &bytes[8..12] == b"WEBP" { "image/webp" }
    else { "image/png" }
}

// Parsed form data from multipart
pub struct CompanyForm {
    pub name: String,
    pub logo: Option<Vec<u8>>,
    pub contact: Contact,
}
