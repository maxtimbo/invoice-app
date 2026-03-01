use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use invoice_core::models::invoice::Invoice;
use anyhow::{Error, Result};
use tera::{Context, Tera};
use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::types::PrintToPdfOptions;

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new(template_path: &PathBuf) -> Result<Self> {
        let template_str = &template_path.to_str().unwrap();
        let glob_pattern = format!("{}/**/*", template_str);
        let tera = Tera::new(&glob_pattern)
            .map_err(|e| Error::msg(format!("Failed to initalize Tera: {}", e)))?;
        Ok(TemplateEngine { tera })
    }
    pub fn render(&self, invoice: &Invoice) -> Result<String> {
        let context = Context::from_serialize(invoice)
            .map_err(|e| Error::msg(format!("Context error: {}", e)))?;

        let rendered = self.tera.render("default.html", &context).map_err(|e| {
            eprintln!("Detailed error: {:?}", e);
            Error::msg(format!("Template rendering error: {}", e))
        })?;
        Ok(rendered)
    }
    pub fn to_file(&self, rendered: &String, output_file: &PathBuf) -> Result<()> {
        let mut file = File::create(output_file)
            .map_err(|e| Error::msg(format!("Failed to create output file: {}", e)))?;

        file.write_all(rendered.as_bytes())
            .map_err(|e| Error::msg(format!("Failed to write output file: {}", e)))?;

        Ok(())
    }
    pub fn to_pdf(&self, input_file: &PathBuf) -> Result<PathBuf, anyhow::Error> {
        let input = format!("file://{}", input_file.canonicalize()?.display());

        let mut output_file = input_file.clone();
        output_file.set_extension("pdf");

        let browser = Browser::new(LaunchOptions::default_builder().build().unwrap())?;
        let tab = browser.new_tab()?;
        tab.navigate_to(&input)?.wait_until_navigated()?;

        let pdf_options = PrintToPdfOptions {
            landscape: None,
            display_header_footer: None,
            print_background: Some(true),
            scale: None,
            paper_width: None,
            paper_height: None,
            margin_top: None,
            margin_bottom: None,
            margin_left: None,
            margin_right: None,
            page_ranges: None,
            ignore_invalid_page_ranges: None,
            header_template: None,
            footer_template: None,
            prefer_css_page_size: None,
            transfer_mode: None,
        };

        let bytes = tab.print_to_pdf(Some(pdf_options))?;

        std::fs::write(&output_file, bytes)?;

        Ok(output_file)
    }
}
