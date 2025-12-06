use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameVariant {
    Numbers,
    Letters,
    LetterTeams,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SoundEvent {
    #[default]
    None,
    PlaySuccess,
    PlayFailure,
    SayPrompt(String),
    GameStart,
    LevelComplete,
}

impl GameVariant {
    /// Returns the item pool (characters or strings) for this game variant
    pub fn item_pool(&self) -> Vec<String> {
        match self {
            GameVariant::Numbers => ('1'..='9').map(|c| c.to_string()).collect(),
            GameVariant::Letters => ('A'..='Z').map(|c| c.to_string()).collect(),
            GameVariant::LetterTeams => vec![
                "th", "sh", "ch", "wh", "ph", "ng", "ck", "qu"
            ].into_iter().map(String::from).collect(),
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
    pub target: String,
    pub options: Vec<String>, // Visual hint options
    #[serde(default)]
    pub selected_index: usize, // For ArrowSelection mode
    pub level_time_limit: Option<u64>, // Seconds, None if no limit
    pub level_elapsed_time: u64, // Seconds elapsed in current level
}
