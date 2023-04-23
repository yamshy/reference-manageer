#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{App, Manager};

fn main() {
    tauri::Builder::default()
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let main_window = app.get_window("main").expect("failed to get main window");
    setup_local_dirs(app);

    // Listen for file drop events
    main_window.listen("file_drop", |event| {
        handle_file_drop(event.payload().expect("failed to get file_drop payload"));
    });

    Ok(())
}

fn setup_local_dirs(app: &mut App) {
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

fn handle_file_drop(file_path: &str) {
    let clean_path = file_path.replace("\"", "");
    // opener::open(clean_path).unwrap();
    println!("file_path: {:?}", clean_path);
}
