use letterlanders_core::{
    GameEngine, GameSettings, GameStatus, GameVariant, SessionState, SoundEvent,
};
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    engine: Mutex<GameEngine>,
}

#[derive(Serialize)]
struct FrontendState {
    status: GameStatus,
    session: Option<SessionState>,
}

impl FrontendState {
    fn from_engine(engine: &GameEngine) -> Self {
        Self {
            status: engine.status.clone(),
            session: engine.session.clone(),
        }
    }
}

#[tauri::command]
fn get_game_state(state: State<AppState>) -> FrontendState {
    let engine = state.engine.lock().unwrap();
    FrontendState::from_engine(&engine)
}

#[tauri::command]
fn get_settings(state: State<AppState>) -> GameSettings {
    let engine = state.engine.lock().unwrap();
    engine.settings.clone()
}

#[tauri::command]
fn update_settings(state: State<AppState>, settings: GameSettings) -> FrontendState {
    let mut engine = state.engine.lock().unwrap();
    engine.settings = settings;
    let _ = engine.settings.save_to_file("settings.json");
    engine.status = GameStatus::Menu; // Return to menu after saving
    FrontendState::from_engine(&engine)
}

#[tauri::command]
fn go_to_settings(state: State<AppState>) -> FrontendState {
    let mut engine = state.engine.lock().unwrap();
    engine.status = GameStatus::Settings { message: None };
    FrontendState::from_engine(&engine)
}

#[tauri::command]
fn start_new_game(state: State<AppState>, variant_str: String) -> FrontendState {
    let mut engine = state.engine.lock().unwrap();
    let variant = match variant_str.as_str() {
        "Letters" => GameVariant::Letters,
        _ => GameVariant::Numbers,
    };
    engine.start_game(variant);
    FrontendState::from_engine(&engine)
}

#[tauri::command]
fn submit_answer(state: State<AppState>, answer: char) -> FrontendState {
    let mut engine = state.engine.lock().unwrap();
    engine.submit_answer(answer);
    FrontendState::from_engine(&engine)
}

#[tauri::command]
fn next_level(state: State<AppState>) -> FrontendState {
    let mut engine = state.engine.lock().unwrap();
    match engine.status {
        GameStatus::LevelComplete { .. } => engine.advance_to_next_level_or_retry(),
        GameStatus::Feedback { .. } => engine.next_level(),
        _ => {}
    }
    FrontendState::from_engine(&engine)
}

#[tauri::command]
fn consume_sound(state: State<AppState>) -> Option<SoundEvent> {
    let mut engine = state.engine.lock().unwrap();
    let sound = engine.consume_sound();
    if sound == SoundEvent::None {
        None
    } else {
        Some(sound)
    }
}

// Reset to menu
#[tauri::command]
fn reset_game(state: State<AppState>) -> FrontendState {
    let mut engine = state.engine.lock().unwrap();
    engine.status = GameStatus::Menu;
    engine.session = None;
    FrontendState::from_engine(&engine)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            engine: Mutex::new(GameEngine::new_with_settings(
                GameSettings::load_from_file("settings.json").unwrap_or_default(),
            )),
        })
        .invoke_handler(tauri::generate_handler![
            get_game_state,
            get_settings,
            update_settings,
            go_to_settings,
            start_new_game,
            submit_answer,
            next_level,
            consume_sound,
            reset_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
