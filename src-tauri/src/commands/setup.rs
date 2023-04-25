use crate::AppState;
use std::fs::File;
use std::io::prelude::*;
use tauri::Runtime;

#[tauri::command]
pub async fn setup_local_db<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    println!("setup_local_db");

    let data_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("failed to get app local data dir");
    let local_db_dir = data_dir.join("local_metadata");
    let local_db_file = local_db_dir.join("metadata.sqlite");

    if !local_db_dir.exists() {
        std::fs::create_dir_all(&local_db_dir).expect("failed to create local_db dir");
    }

    if !local_db_file.exists() {
        File::create(&local_db_file).expect("failed to create local_db file");
    }

    let db_url = format!(
        "sqlite://{}",
        local_db_file
            .to_str()
            .expect("failed to convert local_db_file to str")
    );
    state.db_connect(&db_url).await.unwrap();

    // set up env file for dev
    let env_file = std::env::current_dir()
        .expect("failed to get current dir")
        .join(".env");
    if !env_file.exists() {
        let mut file = File::create(&env_file).expect("failed to create .env file");
        file.write_all(format!("DATABASE_URL={}", db_url).as_bytes())
            .expect("failed to write to .env file");
    }

    Ok(())
}
