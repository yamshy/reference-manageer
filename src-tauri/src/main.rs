#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use sea_orm::{Database, DbErr};
use tauri::{App, Manager};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let (async_process_input_tx, async_process_input_rx) = mpsc::channel(1);
            let (async_process_output_tx, async_process_output_rx) = mpsc::channel(1);
            let main_window = app.get_window("main").expect("failed to get main window");
            setup_local_dirs(&app);
    
            tokio::spawn(async move {
                setup_local_db(&app).await.expect("failed to setup local db");
            });

            // Listen for file drop events
            main_window.listen("file_drop", |event| {
                handle_file_drop(event.payload().expect("failed to get file_drop payload"));
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_local_dirs(app: &App) {
    let data_dir = app
        .path_resolver()
        .app_local_data_dir()
        .expect("failed to get app local data dir");
    let local_files_dir = data_dir.join("local_files");
    let local_metadata_dir = data_dir.join("local_metadata");

    if !local_files_dir.exists() {
        std::fs::create_dir_all(&local_files_dir).expect("failed to create local_files dir");
    }
    if !local_metadata_dir.exists() {
        std::fs::create_dir_all(&local_metadata_dir).expect("failed to create local_metadata dir");
    }
}

async fn setup_local_db(app: &App) -> Result<(), DbErr> {
    let data_dir = app
        .path_resolver()
        .app_local_data_dir()
        .expect("failed to get app local data dir");
    let local_db_path = data_dir.join("local_metadata").join("metadata.db");
    let db_url = format!(
        "sqlite://{}",
        local_db_path.to_str().expect("failed to get db path")
    );
    let mut env_file = File::create(".env").expect("failed to create .env file");
    env_file.write_all(format!("DATABASE_URL={}", db_url).as_bytes()).expect("failed to write to .env file");
    let db = Database::connect(&db_url).await?;

    Ok(())
}

fn handle_file_drop(file_path: &str) {
    let clean_path = file_path.replace("\"", "");
    // opener::open(clean_path).unwrap();
    println!("file_path: {:?}", clean_path);
}
