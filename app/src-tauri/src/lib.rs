use letterlanders_core::{GameEngine, GameVariant};
use std::sync::Mutex;
use tauri::State;

struct AppState {
    engine: Mutex<GameEngine>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_new_game(state: State<AppState>) -> String {
    let mut engine = state.engine.lock().unwrap();
    engine.start_game(GameVariant::Numbers);
    // Return a debug string for now
    format!("Game Started! Status: {:?}, Target: {:?}", engine.status, engine.session.as_ref().map(|s| s.target))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { engine: Mutex::new(GameEngine::new()) })
        .invoke_handler(tauri::generate_handler![greet, start_new_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
