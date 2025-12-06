use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputMethod {
    DirectKeyboard,
    ArrowSelection,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    #[serde(default = "default_feedback_duration")]
    pub feedback_duration_seconds: u64,

    #[serde(default = "default_show_target")]
    pub show_target_visual: bool,

    #[serde(default = "default_input_method")]
    pub input_method: InputMethod,

    #[serde(default = "default_start_level")]
    pub start_level: u32,
}

fn default_feedback_duration() -> u64 {
    2
}

fn default_show_target() -> bool {
    false
}

fn default_input_method() -> InputMethod {
    InputMethod::Hybrid
}

fn default_start_level() -> u32 {
    1
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            feedback_duration_seconds: default_feedback_duration(),
            show_target_visual: default_show_target(),
            input_method: default_input_method(),
            start_level: default_start_level(),
        }
    }
}

impl GameSettings {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        if !path.as_ref().exists() {
            let defaults = Self::default();
            defaults.save_to_file(&path)?;
            return Ok(defaults);
        }
        let content = fs::read_to_string(path)?;
        let settings = serde_json::from_str(&content)?;
        Ok(settings)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
