
use inquire::{Text, InquireError, Editor};

pub trait EntityUpdater<T> {
    type Output;
    fn update(&self) -> Result<Self::Output, InquireError>;
}

pub trait EntityDeleter<T> {
    type Output;
    fn delete(&self) -> Result<Self::Output, anyhow::Error>;
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
