use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use letterlanders_core::settings::InputMethod;
use letterlanders_core::{GameEngine, GameSettings, GameStatus, GameVariant, SoundEvent};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};

mod ui;

fn main() -> Result<()> {
    // Load Settings
    let settings = GameSettings::load_from_file("settings.json").unwrap_or_default();

    // Audio Setup
    // We use a result here because on some headless CI/servers audio might fail,
    // but we don't want to crash the game logic.
    let (_stream, stream_handle) = match rodio::OutputStream::try_default() {
        Ok(s) => (Some(s.0), Some(s.1)),
        Err(_) => (None, None),
    };

    let sink = if let Some(handle) = &stream_handle {
        rodio::Sink::try_new(handle).ok()
    } else {
        None
    };

    // Terminal Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_result = run_app(&mut terminal, sink.as_ref(), settings);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    app_result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    sink: Option<&rodio::Sink>,
    settings: GameSettings,
) -> Result<()> {
    let mut engine = GameEngine::new_with_settings(settings);
    let mut feedback_start: Option<Instant> = None;
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &engine))?;

        // Game Timer Tick (for Boss Level)
        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs();
        if dt >= 1 {
            engine.tick(dt);
            last_tick = now;
        }

        // Handle Audio
        let sound = engine.consume_sound();
        if let Some(s) = sink {
            match sound {
                SoundEvent::None => {}
                SoundEvent::PlaySuccess => play_file(s, "success.wav"),
                SoundEvent::PlayFailure => play_file(s, "failure.wav"),
                SoundEvent::SayPrompt(c) => play_file(s, &format!("prompts/{}.wav", c)),
                SoundEvent::LevelComplete => play_file(s, "complete.wav"),
                _ => {}
            }
        }

        // Auto-advance Feedback Timer
        if let GameStatus::Feedback { .. } = engine.status {
            if feedback_start.is_none() {
                feedback_start = Some(Instant::now());
            } else if let Some(start) = feedback_start {
                if start.elapsed() >= Duration::from_secs(engine.settings.feedback_duration_seconds)
                {
                    engine.next_level();
                    feedback_start = None;
                }
            }
        } else {
            feedback_start = None;
        }

        // Input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match engine.status {
                        GameStatus::Menu => match key.code {
                            KeyCode::Char('n') | KeyCode::Char('N') => {
                                engine.start_game(GameVariant::Numbers)
                            }
                            KeyCode::Char('l') | KeyCode::Char('L') => {
                                engine.start_game(GameVariant::Letters)
                            }
                            KeyCode::Char('s') | KeyCode::Char('S') => {
                                engine.status = GameStatus::Settings { message: None }
                            }
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            _ => {}
                        },
                        GameStatus::Settings { ref mut message } => {
                            // ... existing settings input ...
                            match key.code {
                                KeyCode::Char('1') => {
                                    engine.settings.show_target_visual =
                                        !engine.settings.show_target_visual;
                                    *message = None;
                                }
                                KeyCode::Char('2') => {
                                    engine.settings.feedback_duration_seconds =
                                        match engine.settings.feedback_duration_seconds {
                                            1 => 2,
                                            2 => 3,
                                            3 => 5,
                                            _ => 1,
                                        };
                                    *message = None;
                                }
                                KeyCode::Char('3') => {
                                    engine.settings.input_method = match engine
                                        .settings
                                        .input_method
                                    {
                                        InputMethod::DirectKeyboard => InputMethod::ArrowSelection,
                                        InputMethod::ArrowSelection => InputMethod::Hybrid,
                                        InputMethod::Hybrid => InputMethod::DirectKeyboard,
                                    };
                                    *message = None;
                                }
                                KeyCode::Char('4') => {
                                    engine.settings.start_level =
                                        if engine.settings.start_level >= 4 {
                                            1
                                        } else {
                                            engine.settings.start_level + 1
                                        };
                                    *message = None;
                                }
                                KeyCode::Char('5') => {
                                    // Export
                                    match engine
                                        .settings
                                        .save_to_file("letterlanders_settings_export.json")
                                    {
                                        Ok(_) => {
                                            *message = Some(
                                                "Exported to letterlanders_settings_export.json"
                                                    .to_string(),
                                            )
                                        }
                                        Err(e) => *message = Some(format!("Export failed: {}", e)),
                                    }
                                }
                                KeyCode::Char('6') => {
                                    // Import
                                    match GameSettings::load_from_file(
                                        "letterlanders_settings_export.json",
                                    ) {
                                        Ok(s) => {
                                            engine.settings = s;
                                            *message =
                                                Some("Settings imported successfully!".to_string());
                                        }
                                        Err(_) => {
                                            *message = Some(
                                                "Import failed: file not found or invalid"
                                                    .to_string(),
                                            )
                                        }
                                    }
                                }
                                KeyCode::Esc => {
                                    // Save settings when exiting menu
                                    let _ = engine.settings.save_to_file("settings.json");
                                    engine.status = GameStatus::Menu;
                                }
                                _ => {}
                            }
                        }
                        GameStatus::Playing => match engine.settings.input_method {
                            InputMethod::DirectKeyboard => match key.code {
                                KeyCode::Char(c) => engine.submit_answer(c),
                                KeyCode::Esc => engine.status = GameStatus::Menu,
                                _ => {}
                            },
                            InputMethod::ArrowSelection => match key.code {
                                KeyCode::Left => engine.move_selection(-1),
                                KeyCode::Right => engine.move_selection(1),
                                KeyCode::Enter => engine.submit_current_selection(),
                                KeyCode::Esc => engine.status = GameStatus::Menu,
                                _ => {}
                            },
                            InputMethod::Hybrid => match key.code {
                                KeyCode::Left => engine.move_selection(-1),
                                KeyCode::Right => engine.move_selection(1),
                                KeyCode::Enter => engine.submit_current_selection(),
                                KeyCode::Char(c) => engine.submit_answer(c),
                                KeyCode::Esc => engine.status = GameStatus::Menu,
                                _ => {}
                            },
                        },
                        GameStatus::LevelComplete { .. } => match key.code {
                            KeyCode::Enter => engine.advance_to_next_level_or_retry(),
                            KeyCode::Esc => engine.status = GameStatus::Menu,
                            _ => {}
                        },
                        GameStatus::Feedback { .. } => {
                            if key.code == KeyCode::Esc {
                                engine.status = GameStatus::Menu
                            }
                        }
                        GameStatus::SessionComplete { .. } => match key.code {
                            KeyCode::Char('m') | KeyCode::Char('M') | KeyCode::Esc => {
                                engine.status = GameStatus::Menu
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
    }
}

fn play_file(sink: &rodio::Sink, filename: &str) {
    // Path relative to where binary is run (root of repo usually)
    let path = format!("assets/sounds/{}", filename);
    if let Ok(file) = std::fs::File::open(&path) {
        let source = rodio::Decoder::new(std::io::BufReader::new(file));
        if let Ok(s) = source {
            sink.append(s);
        }
    }
}
