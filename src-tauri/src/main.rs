#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use std::sync::{Arc, Mutex};

use commands::*;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct AppState {
    pub db: Arc<Mutex<Option<DatabaseConnection>>>,
}

impl AppState {
    async fn db_connect(&self, db_url: &str) -> Result<(), DbErr> {
        let db_connection = Database::connect(db_url).await?;
        let mut connection = self.db.lock().unwrap();
        *connection = Some(db_connection);
        Ok(())
    }
}

fn main() {
    let app_state = AppState {
        db: Arc::new(Mutex::new(None)),
    };

    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to get app local data dir");
            let local_files_dir = data_dir.join("local_files");
            let local_metadata_dir = data_dir.join("local_metadata");

            if !local_files_dir.exists() {
                std::fs::create_dir_all(&local_files_dir)
                    .expect("failed to create local_files dir");
            }
            if !local_metadata_dir.exists() {
                std::fs::create_dir_all(&local_metadata_dir)
                    .expect("failed to create local_metadata dir");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![setup_local_db,])
        .manage(app_state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
