#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").expect("failed to get main window");
            let data_dir = app
                .path_resolver()
                .app_local_data_dir()
                .expect("failed to get app local data dir");
            if !data_dir.join("local_files").exists() {
                std::fs::create_dir_all(data_dir.join("local_files"))
                    .expect("failed to create local_files dir");
            }
            println!("data_dir: {:?}", data_dir);

            // Listen for file drop events
            main_window.listen("file_drop", |event| {
                handle_file_drop(event.payload().unwrap());
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_file_drop(file_path: &str) {
    let clean_path = file_path.replace("\"", "");
    // opener::open(clean_path).unwrap();
    println!("file_path: {:?}", clean_path);
}

fn startup() {}
