use crate::models::ids::TermsId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terms {
    pub id: TermsId,
    pub name: String,
    pub due: i64,
}
