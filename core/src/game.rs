'''use crate::settings::GameSettings;
use crate::state::{GameStatus, GameVariant, SessionState, SoundEvent};
use rand::seq::SliceRandom;

pub struct GameEngine {
    pub status: GameStatus,
    pub session: Option<SessionState>,
    pub last_sound: SoundEvent,
    pub settings: GameSettings,
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            status: GameStatus::Menu,
            session: None,
            last_sound: SoundEvent::None,
            settings: GameSettings::default(),
        }
    }

    pub fn new_with_settings(settings: GameSettings) -> Self {
        Self {
            status: GameStatus::Menu,
            session: None,
            last_sound: SoundEvent::None,
            settings,
        }
    }

    pub fn start_game(&mut self, variant: GameVariant) {
        let mut session = SessionState {
            variant,
            current_level: self.settings.start_level,
            current_question_index: 0,
            total_questions: 5, // Default for non-boss levels
            score: 0,
            total_score: 0,
            target: ' ',
            options: vec![],
            selected_index: 0,
            level_time_limit: None,
            level_elapsed_time: 0,
        };

        Self::setup_level_properties(&mut session); // Call here to setup level 1 total_questions
        Self::generate_level_question(&mut session);
        self.session = Some(session);
        self.status = GameStatus::Playing;

        if let Some(s) = &self.session {
            self.last_sound = SoundEvent::SayPrompt(s.target);
        }
    }

    fn setup_level_properties(session: &mut SessionState) {
        match session.current_level {
            1..=3 => session.total_questions = 5,
            _ => session.total_questions = 1, // Boss level is just 1 question
        }
    }

    fn generate_level_question(session: &mut SessionState) {
        let mut rng = rand::thread_rng();

        // Level Configuration
        let (num_options, time_limit) = match session.current_level {
            1 => (2, None),
            2 => (3, None),
            3 => (5, None),
            _ => (9, Some(5)), // Boss Level: 9 options, 5 seconds!
        };

        session.level_time_limit = time_limit;
        session.level_elapsed_time = 0; // Reset timer for new question/level

        let mut pool = session.variant.char_pool();
        pool.shuffle(&mut rng); // Shuffle the pool to ensure random questions
        let target = *pool.choose(&mut rng).unwrap();

        // Generate distinct options (including target)
        let mut options = vec![target];
        while options.len() < num_options {
            let choice = *pool.choose(&mut rng).unwrap();
            if !options.contains(&choice) {
                options.push(choice);
            }
        }
        options.shuffle(&mut rng);

        session.target = target;
        session.options = options;
        session.selected_index = 0;
    }

    pub fn next_level(&mut self) {
        if let Some(session) = &mut self.session {
            session.current_question_index += 1;

            if session.current_question_index >= session.total_questions {
                // End of Level Check
                // Dynamic passing threshold: 80% of total questions, rounded up.
                // For 5 questions: 0.8 * 5 = 4.0 -> 4
                // For 1 question: 0.8 * 1 = 0.8 -> 1
                let threshold = (session.total_questions as f32 * 0.8).ceil() as u32;
                let passed = session.score >= threshold;

                if passed && session.current_level >= 4 {
                    // Boss Defeated! Skip intermediate screen.
                    session.total_score += session.score;
                    self.status = GameStatus::SessionComplete {
                        score: session.total_score,
                    };
                    self.last_sound = SoundEvent::LevelComplete;
                } else {
                    self.status = GameStatus::LevelComplete {
                        level: session.current_level,
                        score: session.score,
                        passed,
                    };
                    self.last_sound = if passed {
                        SoundEvent::LevelComplete
                    } else {
                        SoundEvent::PlayFailure
                    };
                }
            } else {
                Self::generate_level_question(session);
                self.status = GameStatus::Playing;
                self.last_sound = SoundEvent::SayPrompt(session.target);
            }
        }
    }

    pub fn advance_to_next_level_or_retry(&mut self) {
        if let GameStatus::LevelComplete { passed, level, .. } = self.status {
            if let Some(session) = &mut self.session {
                if passed {
                    // Accumulate score before moving on or finishing
                    session.total_score += session.score;

                    if level >= 4 {
                        // Boss level is 4. Passing level 4 means game over!
                        self.status = GameStatus::SessionComplete {
                            score: session.total_score,
                        };
                        self.last_sound = SoundEvent::LevelComplete;
                        return;
                    }
                    session.current_level += 1;
                }
                // Reset for new level (or retry)
                // Note: We do NOT reset total_score here, only current level score
                session.current_question_index = 0;
                session.score = 0;
                session.level_elapsed_time = 0;
                Self::setup_level_properties(session); // Re-evaluate total_questions

                Self::generate_level_question(session);
                self.status = GameStatus::Playing;
                self.last_sound = SoundEvent::SayPrompt(session.target);
            }
        }
    }

    pub fn tick(&mut self, dt_seconds: u64) {
        if let GameStatus::Playing = self.status {
            if let Some(session) = &mut self.session {
                if let Some(limit) = session.level_time_limit {
                    session.level_elapsed_time += dt_seconds;
                    if session.level_elapsed_time >= limit {
                        // Timeout!
                        self.status = GameStatus::Feedback {
                            success: false,
                            message: "Time's Up!".to_string(),
                        };
                        self.last_sound = SoundEvent::PlayFailure;
                    }
                }
            }
        }
    }

    pub fn move_selection(&mut self, delta: i32) {
        if let Some(session) = &mut self.session {
            let len = session.options.len() as i32;
            if len == 0 {
                return;
            }

            let new_index = (session.selected_index as i32 + delta).rem_euclid(len);
            session.selected_index = new_index as usize;
        }
    }

    pub fn submit_current_selection(&mut self) {
        if let Some(session) = &self.session {
            if let Some(&selected_char) = session.options.get(session.selected_index) {
                // We need to clone the char to avoid borrowing issues since submit_answer uses &mut self
                let choice = selected_char;
                self.submit_answer(choice);
            }
        }
    }

    pub fn submit_answer(&mut self, input: char) {
        if let Some(session) = &mut self.session {
            let is_correct = input.eq_ignore_ascii_case(&session.target);

            if is_correct {
                session.score += 1;
                self.status = GameStatus::Feedback {
                    success: true,
                    message: "Great Job!".to_string(),
                };
                self.last_sound = SoundEvent::PlaySuccess;
            } else {
                self.status = GameStatus::Feedback {
                    success: false,
                    message: format!("Oops! That was {}. Try again!", input),
                };
                self.last_sound = SoundEvent::PlayFailure;
            }
        }
    }

    // Consumes the last sound event (so it doesn't loop)
    pub fn consume_sound(&mut self) -> SoundEvent {
        std::mem::take(&mut self.last_sound)
    }
}
''