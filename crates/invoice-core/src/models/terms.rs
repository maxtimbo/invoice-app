use crate::models::ids::TermsId;

#[derive(Debug, Clone)]
pub struct Terms {
    pub id: TermsId,
    pub name: String,
    pub due: i64,
}
