//! Minimal Tauri Shell
//!
//! This is a thin shell that hosts the web frontend. All game logic
//! runs in WebAssembly in the browser/webview. No IPC handlers needed.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
