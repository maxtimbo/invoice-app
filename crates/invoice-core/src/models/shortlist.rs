
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ShortList {
    pub id: i64,
    pub name: String,
}
