use anyhow::{Context, Result};
use std::path::PathBuf;

/// Environment variable naming the directory that holds the database and
/// invoice templates. In Docker this points at the mounted volume.
pub const DATA_DIR_ENV: &str = "INVOICE_DATA_DIR";
const DEFAULT_DATA_DIR: &str = "data";

#[derive(Debug, Clone)]
pub struct Paths {
    pub data: PathBuf,
    pub db: PathBuf,
    pub templates: PathBuf,
}

impl Paths {
    pub fn init() -> Result<Paths> {
        let data: PathBuf = std::env::var_os(DATA_DIR_ENV)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_DATA_DIR));
        std::fs::create_dir_all(&data)
            .with_context(|| format!("Unable to create data dir: {:?}", &data))?;

        let db = data.join("invoice.db");

        let templates = data.join("templates");
        std::fs::create_dir_all(&templates)
            .with_context(|| format!("Unable to create templates dir: {:?}", &templates))?;

        // Seed the default template so users can customise it in the volume
        let default = templates.join("default.html");
        if !default.is_file() {
            std::fs::write(&default, include_str!("../templates/default.html"))
                .with_context(|| format!("Unable to create {:?}", &default))?;
        }

        Ok(Paths { data, db, templates })
    }
}
