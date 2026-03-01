use crate::models::ids::MethodId;

#[derive(Debug, Clone)]
pub struct Methods {
    pub id: MethodId,
    pub name: String,
    pub link: Option<String>,
    pub qr: Option<Vec<u8>>,
}
