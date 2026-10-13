mod state;
mod routes;

use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use axum::{Router, routing::get, response::Redirect};
use invoice_storage::sqlite::SqliteStorage;
use invoice_app::commands::paths::Paths;
use tera::Tera;
use state::AppState;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    let paths = Paths::init()?;
    let db = SqliteStorage::connect(paths.db.to_str().unwrap()).await?;
    db.migrate().await?;

    // Directory holding `templates/` and `static/`; defaults to the crate dir for local dev
    let web_dir = std::env::var_os("INVOICE_WEB_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    let bind = std::env::var("INVOICE_BIND").unwrap_or_else(|_| "127.0.0.1:3000".into());

    let mut tera = Tera::new(web_dir.join("templates/**/*.html").to_str().unwrap())?;
    tera.autoescape_on(vec!["html"]);

    let state = Arc::new(AppState { db, tera, paths });

    let app = Router::new()
        .route("/",                                     get(root))

        .route("/items",                                get(routes::items::list))
        .route("/items/new",                            get(routes::items::new_form).post(routes::items::create))
        .route("/items/quick",                          axum::routing::post(routes::items::quick_create))
        .route("/items/{id}",                           get(routes::items::edit_form).post(routes::items::update))
        .route("/items/{id}/delete",                    axum::routing::post(routes::items::delete))

        .route("/companies",                            get(routes::companies::list))
        .route("/companies/new",                        get(routes::companies::new_form).post(routes::companies::create))
        .route("/companies/{id}",                       get(routes::companies::edit_form).post(routes::companies::update))
        .route("/companies/{id}/delete",                axum::routing::post(routes::companies::delete))

        .route("/clients",                              get(routes::clients::list))
        .route("/clients/new",                          get(routes::clients::new_form).post(routes::clients::create))
        .route("/clients/{id}",                         get(routes::clients::edit_form).post(routes::clients::update))
        .route("/clients/{id}/delete",                  axum::routing::post(routes::clients::delete))

        .route("/terms",                                get(routes::terms::list))
        .route("/terms/new",                            get(routes::terms::new_form).post(routes::terms::create))
        .route("/terms/{id}",                           get(routes::terms::edit_form).post(routes::terms::update))
        .route("/terms/{id}/delete",                    axum::routing::post(routes::terms::delete))

        .route("/methods",                              get(routes::methods::list))
        .route("/methods/new",                          get(routes::methods::new_form).post(routes::methods::create))
        .route("/methods/{id}",                         get(routes::methods::edit_form).post(routes::methods::update))
        .route("/methods/{id}/delete",                  axum::routing::post(routes::methods::delete))

        .route("/templates",                            get(routes::templates::list))
        .route("/templates/new",                        get(routes::templates::new_form).post(routes::templates::create))
        .route("/templates/{id}",                       get(routes::templates::edit_form).post(routes::templates::update))
        .route("/templates/{id}/delete",                axum::routing::post(routes::templates::delete))

        .route("/invoices",                             get(routes::invoices::list))
        .route("/invoices/new",                         get(routes::invoices::new_form).post(routes::invoices::create))
        .route("/invoices/{id}",                        get(routes::invoices::edit_form).post(routes::invoices::update))
        .route("/invoices/{id}/delete",                 axum::routing::post(routes::invoices::delete))
        .route("/invoices/{id}/view",                   get(routes::invoices::view))
        .route("/invoices/{id}/print",                  get(routes::invoices::print))
        .route("/invoices/{id}/items/add",              axum::routing::post(routes::invoices::add_item))
        .route("/invoices/{id}/items/{item_id}/remove", axum::routing::post(routes::invoices::remove_item))
        .route("/invoices/{id}/email",                  axum::routing::post(routes::invoices::send_email))
        .route("/invoices/{id}/archive",                axum::routing::post(routes::invoices::archive))
        .route("/invoices/{id}/unarchive",              axum::routing::post(routes::invoices::unarchive))

        .route("/settings/email",                       get(routes::settings::email_form).post(routes::settings::save_email_config))
        .route("/settings/email/test",                  axum::routing::post(routes::settings::test_email_config))

        .nest_service("/static",                        ServeDir::new(web_dir.join("static")))
        .with_state(state);

    println!("Listening on {bind}");
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> Redirect {
    Redirect::to("/invoices")
}
