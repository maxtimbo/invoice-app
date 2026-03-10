use std::path::PathBuf;
use inquire::{Text, InquireError, Editor};
use anyhow::{anyhow, Result};

use invoice_core::models::contact::Contact;

pub mod render;
mod interactive;
mod cli;
mod validators;

pub trait EntityUpdater<T> {
    type Output;
    fn update(&self) -> Result<Self::Output, InquireError>;
}

pub trait EntityDeleter<T> {
    type Output;
    fn delete(&self) -> Result<Self::Output, anyhow::Error>;
}

pub fn is_cancelled(e: &anyhow::Error) -> bool {
    e.downcast_ref::<InquireError>()
        .map(|ie| matches!(ie, InquireError::OperationCanceled | InquireError::OperationInterrupted))
        .unwrap_or(false)
}

pub fn prompt_contact(existing: Option<&Contact>) -> Result<Contact> {
    let def = |opt: &Option<String>| -> &str { opt.as_deref().unwrap_or("") };

    let (ep, ee, ea1, ea2, ec, es, ez) = match existing {
        Some(c) => (
            &c.phone, &c.email, &c.addr1, &c.addr2,
            &c.city, &c.state, &c.zip,
        ),
        None => {
            let _ : Option<String> = None;
            return prompt_contact_blank();
        }
    };

    Ok(Contact {
        phone:  prompt_optional("Phone:",        def(ep))?,
        email:  prompt_optional("Email:",        def(ee))?,
        addr1:  prompt_optional("Address 1:",    def(ea1))?,
        addr2:  prompt_optional("Address 2:",    def(ea2))?,
        city:   prompt_optional("City:",         def(ec))?,
        state:  prompt_optional("State:",        def(es))?,
        zip:    prompt_optional("Zip:",          def(ez))?,
    })
}

fn prompt_image(prompt: &str) -> Result<Option<Vec<u8>>> {
    let input = prompt_optional(prompt, "")?;
    let path = match input {
        None => return Ok(None),
        Some(s) => PathBuf::from(s),
    };

    // Validate MIME type
    // This should be in app?
    let mime = mime_guess::from_path(&path)
        .first()
        .ok_or_else(|| anyhow!("Could not determine file type for {:?}", path))?;
    if mime.type_() != "image" {
        return Err(anyhow!("File must be an image (got {})", mime));
    }
    let accepted = ["jpeg", "jpg", "png", "webp"];
    if !accepted.contains(&mime.subtype().as_str()) {
        return Err(anyhow!(
            "Unsupported image format '{}'. Use jpeg, png, or webp.",
            mime.subtype()
        ));
    }

    // Validate size (1 MB limit)
    // This should be in app?
    const MAX_BYTES: u64 = 1_000_000;
    let size = std::fs::metadata(&path)?.len();
    if size > MAX_BYTES {
        return Err(anyhow!(
            "Image is {:.1} KB, maximum is 1000 KB.",
            size as f64 / 1024.0
        ));
    }

    Ok(Some(std::fs::read(&path)?))
}

fn prompt_contact_blank() -> Result<Contact> {
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
