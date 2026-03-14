mod views;
mod actions;

pub use views::*;
pub use actions::*;

use serde::Serialize;
use base64::{engine::general_purpose, Engine as _};
use invoice_core::models::method::Method;

#[derive(Serialize)]
pub struct MethodView {
    pub id: i64,
    pub name: String,
    pub link: Option<String>,
    pub qr: Option<String>, // data URI
}

impl From<Method> for MethodView {
    fn from(m: Method) -> Self {
        Self {
            id: m.id.0,
            name: m.name,
            link: m.link,
            qr: m.qr.as_deref().map(|bytes| {
                let mime = detect_mime(bytes);
                format!("data:{mime};base64,{}", general_purpose::STANDARD.encode(bytes))
            }),
        }
    }
}

fn detect_mime(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") { "image/png" }
    else if bytes.starts_with(b"\xFF\xD8\xFF") { "image/jpeg" }
    else if bytes.len() >= 12 && &bytes[8..12] == b"WEBP" { "image/webp" }
    else { "image/png" }
}

pub struct MethodForm {
    pub name: String,
    pub link: Option<String>,
    pub qr: Option<Vec<u8>>,
}
