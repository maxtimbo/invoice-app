pub mod view;

use std::path::PathBuf;

use anyhow::{Error, Result};
use tera::{Context, Tera};
use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::types::PrintToPdfOptions;

use invoice_core::models::invoice::Invoice;
use view::InvoiceView;

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new(template_path: &PathBuf) -> Result<Self> {
        let glob = format!("{}/**/*", template_path.to_str().unwrap());
        let tera = Tera::new(&glob)
            .map_err(|e| Error::msg(format!("Failed to initalize Tera: {}", e)))?;
        Ok(TemplateEngine { tera })
    }
    pub fn render(&self, invoice: &Invoice) -> Result<String> {
        let view = InvoiceView::from(invoice);
        let ctx = Context::from_serialize(&view)
            .map_err(|e| Error::msg(format!("Context error: {e}")))?;
        self.tera.render("default.html", &ctx).map_err(|e| {
            eprintln!("Detailed error: {e:?}");
            Error::msg(format!("Template rendering error: {e}"))
        })
    }
    //pub fn to_file(&self, rendered: &String, output_file: &PathBuf) -> Result<()> {
    //    let mut file = File::create(output_file)
    //        .map_err(|e| Error::msg(format!("Failed to create output file: {}", e)))?;

    //    file.write_all(rendered.as_bytes())
    //        .map_err(|e| Error::msg(format!("Failed to write output file: {}", e)))?;

    //    Ok(())
    //}
    pub fn to_pdf(&self, html: &str) -> Result<Vec<u8>> {
        let tmp = std::env::temp_dir()
            .join(format!("invoice_{}.html", std::process::id()));

        std::fs::write(&tmp, html)
            .map_err(|e| Error::msg(format!("Failed to write tmp html: {e}")))?;

        let url = format!("file://{}", tmp.canonicalize()?.display());

        let browser = Browser::new(LaunchOptions::default_builder().build().unwrap())?;
        let tab = browser.new_tab()?;
        tab.navigate_to(&url)?.wait_until_navigated()?;

        let pdf_options = PrintToPdfOptions {
            generate_document_outline:  None,
            generate_tagged_pdf:        None,
            landscape:                  None,
            display_header_footer:      None,
            print_background:           Some(true),
            scale:                      None,
            paper_width:                None,
            paper_height:               None,
            margin_top:                 None,
            margin_bottom:              None,
            margin_left:                None,
            margin_right:               None,
            page_ranges:                None,
            ignore_invalid_page_ranges: None,
            header_template:            None,
            footer_template:            None,
            prefer_css_page_size:       None,
            transfer_mode:              None,
        };

        let bytes = tab.print_to_pdf(Some(pdf_options))?;

        let _ = std::fs::remove_file(&tmp);


        Ok(bytes)
    }
}
