use letterlanders_core::{GameEngine, GameStatus, GameVariant, SoundEvent, SessionState};
use std::sync::Mutex;
use tauri::State;
use serde::Serialize;

struct AppState {
    engine: Mutex<GameEngine>,
}

#[derive(Serialize)]
struct FrontendState {
    status: GameStatus,
    session: Option<SessionState>,
}

#[tauri::command]
fn get_game_state(state: State<AppState>) -> FrontendState {
    let engine = state.engine.lock().unwrap();
    FrontendState {
        status: engine.status.clone(),
        session: engine.session.clone(),
    }
}

#[tauri::command]
fn start_new_game(state: State<AppState>, variant_str: String) -> FrontendState {
    let mut engine = state.engine.lock().unwrap();
    let variant = match variant_str.as_str() {
        "Letters" => GameVariant::Letters,
        _ => GameVariant::Numbers,
    };
    engine.start_game(variant);
    FrontendState {
        status: engine.status.clone(),
        session: engine.session.clone(),
    }
}

#[tauri::command]
fn submit_answer(state: State<AppState>, answer: char) -> FrontendState {
    let mut engine = state.engine.lock().unwrap();
    engine.submit_answer(answer);
    FrontendState {
        status: engine.status.clone(),
        session: engine.session.clone(),
    }
}

#[tauri::command]
fn next_level(state: State<AppState>) -> FrontendState {
    let mut engine = state.engine.lock().unwrap();
    match engine.status {
        GameStatus::LevelComplete { .. } => engine.advance_to_next_level_or_retry(),
        GameStatus::Feedback { .. } => engine.next_level(),
        _ => {}
    }
    FrontendState {
        status: engine.status.clone(),
        session: engine.session.clone(),
    }
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
    FrontendState {
        status: engine.status.clone(),
        session: engine.session.clone(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { engine: Mutex::new(GameEngine::new()) })
        .invoke_handler(tauri::generate_handler![
            get_game_state, 
            start_new_game, 
            submit_answer, 
            next_level, 
            consume_sound,
            reset_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
