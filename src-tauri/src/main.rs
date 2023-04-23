#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use commands::*;

fn main() {
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
        .invoke_handler(tauri::generate_handler![
            setup_local_db,
            move_file_to_local_storage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
