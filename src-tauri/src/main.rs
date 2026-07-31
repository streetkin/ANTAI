// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn get_antai_version() -> String {
    "ANTAI Sentinel v1.0.0 (Rust Native + Tauri 2.0)".to_string()
}

fn main() {
    // Spawna antai-core.exe in background se presente
    let _ = Command::new("antai-core.exe")
        .spawn();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_antai_version])
        .run(tauri::generate_context!())
        .expect("Errore durante l'esecuzione dell'applicazione Desktop ANTAI");
}
