use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameVariant {
    Numbers,
    Letters,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SoundEvent {
    None,
    PlaySuccess,
    PlayFailure,
    SayPrompt(char),
    GameStart,
    LevelComplete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameStatus {
    Menu,
    Playing,
    Feedback {
        success: bool,
        message: String,
    },
    SessionComplete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub variant: GameVariant,
    pub current_question_index: u32,
    pub total_questions: u32,
    pub score: u32,
    pub target: char,
    pub options: Vec<char>, // Visual hint options
}
