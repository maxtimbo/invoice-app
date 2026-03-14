use crate::state::AppState;
use invoice_app::ports::repos::item_repo::ItemRepo;
use invoice_core::models::item::Item;
use tauri::State;

#[tauri::command]
pub async fn list_items(state: State<'_, AppState>) -> Result<Vec<Item>, String> {
    state.db.lock().await.list_item().await.map_err(|e| e.to_string())
}
