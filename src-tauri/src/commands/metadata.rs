use crate::AppState;
use pdf::file::File;

#[tauri::command]
async fn import_metadata(
    state: tauri::State<'_, AppState>,
    file_path: String,
) -> Result<(), String> {
    let db = state
        .inner()
        .db
        .lock()
        .unwrap()
        .as_ref()
        .ok_or("Database not connected")?;

    let pdf = File::from_data(&file_path).unwrap();

    Ok(())
}
