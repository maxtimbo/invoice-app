use std::path::PathBuf;
use inquire::{Text, InquireError, Editor};
use anyhow::{anyhow, Result};

use invoice_core::models::contact::Contact;
use invoice_app::services::image::validate_image;

pub mod render;

#[macro_export]
macro_rules! resolve_id {
    (
        $id:expr,
        $db:expr,
        $list_method:ident,
        $model_type:ty,
        $id_type:ident,
        $empty_msg:expr,
        $prompt:expr
    ) => {{
        match $id {
            Some(id) => $id_type(id),
            None => {
                let all = $db.$list_method().await?;
                if all.is_empty() {
                    return Err(::anyhow::anyhow!($empty_msg));
                }
                let choice: $model_type =
                    ::inquire::Select::new($prompt, all).prompt()?;
                choice.id
            }
        }
    }};
}

pub fn is_cancelled(e: &anyhow::Error) -> bool {
    e.downcast_ref::<InquireError>()
        .map(|ie| matches!(ie, InquireError::OperationCanceled | InquireError::OperationInterrupted))
        .unwrap_or(false)
}

pub fn prompt_contact(existing: Option<&Contact>) -> Result<Contact> {
    match existing {
        None => prompt_contact_blank(),
        Some(c) => Ok(Contact {
            phone:  prompt_optional("Phone:",        c.phone.as_deref().unwrap_or(""))?,
            email:  prompt_optional("Email:",        c.email.as_deref().unwrap_or(""))?,
            addr1:  prompt_optional("Address 1:",    c.addr1.as_deref().unwrap_or(""))?,
            addr2:  prompt_optional("Address 2:",    c.addr2.as_deref().unwrap_or(""))?,
            city:   prompt_optional("City:",         c.city.as_deref().unwrap_or(""))?,
            state:  prompt_optional("State:",        c.state.as_deref().unwrap_or(""))?,
            zip:    prompt_optional("Zip:",          c.zip.as_deref().unwrap_or(""))?,
        }),
    }
}

pub fn prompt_image(prompt: &str) -> Result<Option<Vec<u8>>> {
    let input = prompt_optional(prompt, "")?;
    match input {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => validate_image(&PathBuf::from(s)).map(Some),
    }
}

pub fn prompt_contact_blank() -> Result<Contact> {
    Ok(Contact {
        phone: prompt_optional("Phone:",     "")?,
        email: prompt_optional("Email:",     "")?,
        addr1: prompt_optional("Address 1:", "")?,
        addr2: prompt_optional("Address 2:", "")?,
        city:  prompt_optional("City:",      "")?,
        state: prompt_optional("State:",     "")?,
        zip:   prompt_optional("Zip:",       "")?,
    })
}

pub fn prompt_optional(prompt: &str, default: &str) -> Result<Option<String>, InquireError> {
    let input = Text::new(prompt)
        .with_default(default)
        .prompt()?;

    if input.trim().eq_ignore_ascii_case("None") {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}

pub fn editor_optional(prompt: &str, default: &str) -> Result<Option<String>, InquireError> {
    let input = Editor::new(prompt)
        .with_help_message("Use standard markdown syntax")
        .with_file_extension("md")
        .with_predefined_text(default)
        .prompt()?;

    if input.trim().eq_ignore_ascii_case("None") {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}

pub fn prompt_id(label: &str) -> Result<i64> {
    let s = Text::new(label).prompt()?;
    s.trim()
        .parse::<i64>()
        .map_err(|_| anyhow!("'{}' is not a valid ID.", s))
}
