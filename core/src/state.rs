use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameVariant {
    Numbers,
    Letters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SoundEvent {
    #[default]
    None,
    PlaySuccess,
    PlayFailure,
    SayPrompt(char),
    GameStart,
    LevelComplete,
}

impl GameVariant {
    /// Returns the character pool for this game variant
    pub fn char_pool(&self) -> Vec<char> {
        match self {
            GameVariant::Numbers => ('1'..='9').collect(),
            GameVariant::Letters => ('A'..='Z').collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameStatus {
    Menu,
    Settings {
        message: Option<String>,
    },
    Playing,
    Feedback {
        success: bool,
        message: String,
    },
    LevelComplete {
        level: u32,
        score: u32,
        passed: bool,
    },
    SessionComplete {
        score: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub variant: GameVariant,
    pub current_level: u32,
    pub current_question_index: u32,
    pub total_questions: u32,
    pub score: u32,
    pub total_score: u32, // Cumulative score across levels
    pub target: char,
    pub options: Vec<char>, // Visual hint options
    #[serde(default)]
    pub selected_index: usize, // For ArrowSelection mode
    pub level_time_limit: Option<u64>, // Seconds, None if no limit
    pub level_elapsed_time: u64, // Seconds elapsed in current level
}
