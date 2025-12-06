use letterlanders_core::{GameEngine, GameSettings, GameStatus, GameVariant, SoundEvent};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// Wrapper around the GameEngine for WASM
#[wasm_bindgen]
pub struct WasmGameEngine {
    engine: Mutex<GameEngine>,
}

#[derive(Serialize, Deserialize)]
pub struct FrontendState {
    status: GameStatus,
    session: Option<letterlanders_core::SessionState>,
}

impl FrontendState {
    fn from_engine(engine: &GameEngine) -> Self {
        Self {
            status: engine.status.clone(),
            session: engine.session.clone(),
        }
    }
}

impl Default for WasmGameEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl WasmGameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let settings = Self::load_settings_from_local_storage().unwrap_or_default();
        Self {
            engine: Mutex::new(GameEngine::new_with_settings(settings)),
        }
    }

    pub fn get_game_state(&self) -> JsValue {
        let engine = self.engine.lock().unwrap();
        let state = FrontendState::from_engine(&engine);
        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    pub fn get_settings(&self) -> JsValue {
        let engine = self.engine.lock().unwrap();
        serde_wasm_bindgen::to_value(&engine.settings).unwrap()
    }

    pub fn update_settings(&self, settings_val: JsValue) -> JsValue {
        let mut engine = self.engine.lock().unwrap();
        let settings: GameSettings = serde_wasm_bindgen::from_value(settings_val).unwrap();
        engine.settings = settings;
        let _ = Self::save_settings_to_local_storage(&engine.settings);
        engine.status = GameStatus::Menu;
        let state = FrontendState::from_engine(&engine);
        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    pub fn go_to_settings(&self) -> JsValue {
        let mut engine = self.engine.lock().unwrap();
        engine.status = GameStatus::Settings { message: None };
        let state = FrontendState::from_engine(&engine);
        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    pub fn start_new_game(&self, variant_str: String) -> JsValue {
        let mut engine = self.engine.lock().unwrap();
        let variant = match variant_str.as_str() {
            "Letters" => GameVariant::Letters,
            _ => GameVariant::Numbers,
        };
        engine.start_game(variant);
        let state = FrontendState::from_engine(&engine);
        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    pub fn submit_answer(&self, answer: String) -> JsValue {
        let mut engine = self.engine.lock().unwrap();
        if let Some(c) = answer.chars().next() {
            engine.submit_answer(c);
        }
        let state = FrontendState::from_engine(&engine);
        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    pub fn next_level(&self) -> JsValue {
        let mut engine = self.engine.lock().unwrap();
        match engine.status {
            GameStatus::LevelComplete { .. } => engine.advance_to_next_level_or_retry(),
            GameStatus::Feedback { .. } => engine.next_level(),
            _ => {}
        }
        let state = FrontendState::from_engine(&engine);
        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    pub fn consume_sound(&self) -> JsValue {
        let mut engine = self.engine.lock().unwrap();
        let sound = engine.consume_sound();
        if sound == SoundEvent::None {
            JsValue::NULL
        } else {
            serde_wasm_bindgen::to_value(&sound).unwrap()
        }
    }

    pub fn reset_game(&self) -> JsValue {
        let mut engine = self.engine.lock().unwrap();
        engine.status = GameStatus::Menu;
        engine.session = None;
        let state = FrontendState::from_engine(&engine);
        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    // Internal helpers for LocalStorage
    fn load_settings_from_local_storage() -> Option<GameSettings> {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        let json = storage.get_item("letterlanders_settings").ok()??;
        serde_json::from_str(&json).ok()
    }

    fn save_settings_to_local_storage(settings: &GameSettings) -> Option<()> {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        let json = serde_json::to_string(settings).ok()?;
        storage.set_item("letterlanders_settings", &json).ok()
    }
}
