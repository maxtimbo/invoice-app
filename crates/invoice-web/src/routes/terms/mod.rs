mod views;
mod actions;

pub use views::*;
pub use actions::*;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct TermsForm {
    pub name: String,
    pub due: i64,
}
