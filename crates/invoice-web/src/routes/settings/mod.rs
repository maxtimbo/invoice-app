mod views;
mod actions;

pub use views::*;
pub use actions::*;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmailConfigForm {
    pub smtp_server: String,
    pub port: String,
    pub tls: Option<String>,
    pub username: String,
    pub password: Option<String>,
    pub fromname: String,
    pub test_recipient: Option<String>,
}
