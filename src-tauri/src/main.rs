#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod request;
use request::{get_request, post_request};


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_request, post_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
