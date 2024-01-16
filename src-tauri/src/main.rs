use std::sync::Mutex;

use rug::Integer;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

struct AppState {
    value: Integer,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(state_mutex: tauri::State<'_, Mutex<AppState>>, name: &str) -> String {
    let mut state = state_mutex.lock().unwrap();
    let length = name.len();
    state.value += length;
    format!("({:016b})", state.value)
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState {
            value: Integer::from(0)
        }))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
