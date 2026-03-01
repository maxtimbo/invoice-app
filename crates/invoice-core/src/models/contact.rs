
#[derive(Debug, Clone)]
pub struct Contact {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub addr1: Option<String>,
    pub addr2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
