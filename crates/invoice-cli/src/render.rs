use anyhow::Result;
use invoice_core::models::invoice::Invoice;

pub fn render_html(_invoice: &Invoice) -> Result<String> {
    Ok(String::from("<html><body><p>placeholder</p></body></html>"))
}

pub fn render_pdf(_invoice: &Invoice) -> Result<Vec<u8>> {
    Ok(Vec::new())
}
