use crate::models::ids::MethodId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub id: MethodId,
    pub name: String,
    pub link: Option<String>,
    pub qr: Option<Vec<u8>>,
}
