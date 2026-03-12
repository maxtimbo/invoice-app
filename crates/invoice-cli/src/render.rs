use anyhow::Result;

use invoice_app::commands::paths::Paths;
use invoice_app::render::TemplateEngine;
use invoice_core::models::invoice::Invoice;

pub fn render_html(invoice: &Invoice) -> Result<String> {
    let paths = Paths::init()?;
    TemplateEngine::new(&paths.templates)?.render(invoice)
}

pub fn render_pdf(invoice: &Invoice) -> Result<Vec<u8>> {
    let paths = Paths::init()?;
    let engine = TemplateEngine::new(&paths.templates)?;
    let html = engine.render(invoice)?;
    engine.to_pdf(&html)
}
