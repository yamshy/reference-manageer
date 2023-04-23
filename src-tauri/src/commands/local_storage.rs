use tauri::Runtime;

#[tauri::command]
pub async fn move_file_to_local_storage<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    println!("move_file_to_local_storage");

    Ok(())
}
