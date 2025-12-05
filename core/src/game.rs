use crate::state::{GameStatus, GameVariant, SessionState, SoundEvent};
use rand::seq::SliceRandom;
use rand::Rng;

pub struct GameEngine {
    pub status: GameStatus,
    pub session: Option<SessionState>,
    pub last_sound: SoundEvent,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            status: GameStatus::Menu,
            session: None,
            last_sound: SoundEvent::None,
        }
    }

    pub fn start_game(&mut self, variant: GameVariant) {
        let mut session = SessionState {
            variant,
            current_question_index: 0,
            total_questions: 5, // Short session for testing
            score: 0,
            target: ' ',
            options: vec![],
        };
        
        Self::generate_level(&mut session);
        self.session = Some(session);
        self.status = GameStatus::Playing;
        
        // Sound will be pulled by the UI checking `last_sound` or `get_sound()`
        // Ideally, we set the prompt sound here.
        if let Some(s) = &self.session {
            self.last_sound = SoundEvent::SayPrompt(s.target);
        }
    }

    fn generate_level(session: &mut SessionState) {
        let mut rng = rand::thread_rng();
        
        let (target, pool) = match session.variant {
            GameVariant::Numbers => {
                let pool: Vec<char> = ('1'..='9').collect();
                let target = *pool.choose(&mut rng).unwrap();
                (target, pool)
            },
            GameVariant::Letters => {
                let pool: Vec<char> = ('A'..='Z').collect();
                let target = *pool.choose(&mut rng).unwrap();
                (target, pool)
            },
        };

        // Generate distinct options (including target)
        let mut options = vec![target];
        while options.len() < 3 {
            let choice = *pool.choose(&mut rng).unwrap();
            if !options.contains(&choice) {
                options.push(choice);
            }
        }
        options.shuffle(&mut rng);

        session.target = target;
        session.options = options;
    }

    pub fn submit_answer(&mut self, input: char) {
        if let Some(session) = &mut self.session {
            let is_correct = input.to_ascii_uppercase() == session.target.to_ascii_uppercase();
            
            if is_correct {
                session.score += 1;
                self.status = GameStatus::Feedback { 
                    success: true, 
                    message: "Great Job!".to_string() 
                };
                self.last_sound = SoundEvent::PlaySuccess;
            } else {
                self.status = GameStatus::Feedback { 
                    success: false, 
                    message: format!("Oops! That was {}. Try again!", input).to_string() 
                };
                self.last_sound = SoundEvent::PlayFailure;
            }
        }
    }

    pub fn next_level(&mut self) {
        if let Some(session) = &mut self.session {
            session.current_question_index += 1;
            
            if session.current_question_index >= session.total_questions {
                self.status = GameStatus::SessionComplete;
                self.last_sound = SoundEvent::LevelComplete;
            } else {
                Self::generate_level(session);
                self.status = GameStatus::Playing;
                self.last_sound = SoundEvent::SayPrompt(session.target);
            }
        }
    }
    
    // Consumes the last sound event (so it doesn't loop)
    pub fn consume_sound(&mut self) -> SoundEvent {
        let sound = self.last_sound.clone();
        self.last_sound = SoundEvent::None;
        sound
    }
}
