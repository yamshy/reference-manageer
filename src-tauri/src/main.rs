#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::Read;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            // listen for file_drop
            main_window.listen("file_drop", |event| {
                println!("event: {:?}", event);
                handle_file_drop(event.payload().unwrap());
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_file_drop(file_path: &str) {
    println!("file_drop: {:?}", file_path);

    let clean_path = file_path.replace("\"", "");

    //open file
    let file = std::fs::File::open(clean_path).unwrap();

    println!("file: {:?}", file);
    println!("file: {:?}", file.metadata().unwrap().len());
}
