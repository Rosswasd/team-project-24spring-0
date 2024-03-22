// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_io;
mod interface;
mod middleware;
mod simulator;

#[tauri::command]
fn great(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![great])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
