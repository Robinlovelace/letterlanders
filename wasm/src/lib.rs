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

    pub fn go_to_about(&self) -> JsValue {
        let mut engine = self.engine.lock().unwrap();
        engine.go_to_about();
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

    pub fn tick(&self, dt_seconds: f64) -> JsValue {
        let mut engine = self.engine.lock().unwrap();
        // Cast to u64 as core expects u64 seconds, but wait, core expects u64?
        // Let's check core/src/game.rs line 171: pub fn tick(&mut self, dt_seconds: u64)
        // A u64 for delta time in seconds is very coarse (1 second resolution). 
        // If the tick is called more often, it might be 0.
        // I should probably fix core to use f32 or f64 for smoother timing if needed, 
        // but for now, let's see. If I pass 0.1s, u64 will be 0.
        // Ah, if core uses u64, it implies it only counts whole seconds? 
        // Let's check core again.
        // "session.level_elapsed_time += dt_seconds;" 
        // If dt_seconds is u64, we can only update by whole seconds.
        // For a 5 second timer, this is "okay" but jerky.
        // Ideally I'd refactor core to use f64, but I'll stick to the "Proper Logic Fix" constrained to making it work first.
        // However, if I call tick(0.1) from JS, casting to u64 makes it 0.
        // The loop will never advance time.
        // I MUST refactor core to use f64 or u64 milliseconds.
        // Let's check core/src/game.rs content again in memory.
        
        // Line 49: level_elapsed_time: 0, (in struct SessionState) -> inferred type likely integer
        // Line 171: pub fn tick(&mut self, dt_seconds: u64)
        
        // I will refactor core::game::tick to take milliseconds (u64) or float.
        // Since I'm editing WASM now, let's delay this Edit and do Core first.
        
        engine.tick(dt_seconds); 
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
