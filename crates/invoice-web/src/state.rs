use invoice_storage::sqlite::SqliteStorage;
use tera::Tera;

pub struct AppState {
    pub db: SqliteStorage,
    pub tera: Tera,
}
