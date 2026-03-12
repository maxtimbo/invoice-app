use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;

use invoice_core::models::{
    invoice::Invoice,
    stage::InvoiceStage,
    status::PaidStatus,
};

#[derive(Debug, Serialize)]
pub struct InvoiceView {
    pub id:             i64,
    pub invoice_stage:  &'static str,
    pub date:           String,
    pub due_date:       String,
    pub total:          String,

    pub status:         &'static str,
    pub status_date:    Option<String>,
    pub status_check:   Option<String>,

    pub show_methods:   bool,
    pub show_notes:     bool,
    pub notes:          Option<String>,

    pub items:          Vec<ItemDetailView>,
    pub template:       TemplateView,
}

#[derive(Debug, Serialize)]
pub struct ItemDetailView {
    pub name:     String,
    pub quantity: String,
    pub rate:     String,
    pub subtotal: String,
}

#[derive(Debug, Serialize)]
pub struct TemplateView {
    pub company: CompanyView,
    pub client:  ClientView,
    pub terms:   TermsView,
    pub methods: Vec<MethodView>,
}

#[derive(Debug, Serialize)]
pub struct CompanyView {
    pub name:    String,
    pub logo:    Option<String>,
    pub contact: ContactView,
}

#[derive(Debug, Serialize)]
pub struct ClientView {
    pub name:    String,
    pub contact: ContactView,
}

#[derive(Debug, Serialize)]
pub struct ContactView {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub addr1: Option<String>,
    pub addr2: Option<String>,
    pub city:  Option<String>,
    pub state: Option<String>,
    pub zip:   Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TermsView {
    pub name: String,
    pub due:  i64,
}

#[derive(Debug, Serialize)]
pub struct MethodView {
    pub name: String,
    pub link: Option<String>,
    pub qr:   Option<String>,
}

impl From<&Invoice> for InvoiceView {
    fn from(inv: &Invoice) -> Self {
        let (status, status_date, status_check) = flatten_status(&inv.attributes.status);

        let items = inv
            .calculate_subtotals()
            .iter()
            .map(|d| ItemDetailView {
                name:     d.name.clone(),
                quantity: d.quantity.0.to_string(),
                rate:     d.rate.0.to_string(),
                subtotal: d.subtotal.0.to_string(),
            })
            .collect();

        let company = &inv.template.company;
        let client  = &inv.template.client;
        let terms   = &inv.template.terms;

        let template = TemplateView {
            company: CompanyView {
                name: company.name.clone(),
                logo: company.logo.as_deref().map(image_to_data_uri),
                contact: ContactView::from_contact(&company.contact),
            },
            client: ClientView {
                name: client.name.clone(),
                contact: ContactView::from_contact(&client.contact),
            },
            terms: TermsView {
                name: terms.name.clone(),
                due:  terms.due,
            },
            methods: inv
                .template
                .method
                .iter()
                .map(|m| MethodView {
                    name: m.name.clone(),
                    link: m.link.clone(),
                    qr:   m.qr.as_deref().map(image_to_data_uri),
                })
                .collect(),
        };

        InvoiceView {
            id:            inv.id.0,
            invoice_stage: match inv.attributes.stage {
                InvoiceStage::Invoice => "Invoice",
                InvoiceStage::Quote   => "Quote",
            },
            date:          inv.date.format("%B %d, %Y").to_string(),
            due_date:      inv.due_date().format("%B %d, %Y").to_string(),
            total:         inv.calculate_total().0.to_string(),
            status,
            status_date,
            status_check,
            show_methods:  inv.attributes.show_methods,
            show_notes:    inv.attributes.show_notes,
            notes:         inv.notes.clone(),
            items,
            template,
        }
    }
}

impl ContactView {
    fn from_contact(c: &invoice_core::models::contact::Contact) -> Self {
        Self {
            phone: c.phone.clone(),
            email: c.email.clone(),
            addr1: c.addr1.clone(),
            addr2: c.addr2.clone(),
            city:  c.city.clone(),
            state: c.state.clone(),
            zip:   c.zip.clone(),
        }
    }
}

fn flatten_status(
    status: &PaidStatus,
) -> (&'static str, Option<String>, Option<String>) {
    match status {
        PaidStatus::Waiting  => ("", None, None),
        PaidStatus::PastDue  => ("PAST DUE", None, None),
        PaidStatus::Paid { date, check } => (
            "PAID",
            Some(date.format("%B %d, %Y").to_string()),
            check.clone(),
        ),
        PaidStatus::Failed { date } => (
            "FAILED",
            Some(date.format("%B %d, %Y").to_string()),
            None,
        ),
        PaidStatus::Refunded { date } => (
            "REFUNDED",
            Some(date.format("%B %d, %Y").to_string()),
            None,
        ),
    }
}

fn image_to_data_uri(bytes: &[u8]) -> String {
    let mime = detect_mime(bytes);
    let b64  = general_purpose::STANDARD.encode(bytes);
    format!("data:{mime};base64,{b64}")
}

fn detect_mime(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        "image/png"
    } else if bytes.starts_with(b"\xFF\xD8\xFF") {
        "image/jpeg"
    } else if bytes.len() >= 12 && &bytes[8..12] == b"WEBP" {
        "image/webp"
    } else {
        "image/png"
    }
}
