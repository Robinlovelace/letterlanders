use std::io;
use std::time::Duration;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use letterlanders_core::{GameEngine, GameStatus, GameVariant, SoundEvent};

mod ui;

fn main() -> Result<()> {
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

    let app_result = run_app(&mut terminal, sink.as_ref());

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    app_result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, sink: Option<&rodio::Sink>) -> Result<()> {
    let mut engine = GameEngine::new();

    loop {
        terminal.draw(|f| ui::draw(f, &engine))?;

        // Handle Audio
        let sound = engine.consume_sound();
        if let Some(s) = sink {
            match sound {
                SoundEvent::None => {},
                SoundEvent::PlaySuccess => play_file(s, "success.wav"),
                SoundEvent::PlayFailure => play_file(s, "failure.wav"),
                SoundEvent::SayPrompt(c) => play_file(s, &format!("prompts/{}.wav", c)),
                SoundEvent::LevelComplete => play_file(s, "complete.wav"),
                _ => {}
            }
        }

        // Input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match engine.status {
                        GameStatus::Menu => {
                            match key.code {
                                KeyCode::Char('n') | KeyCode::Char('N') => engine.start_game(GameVariant::Numbers),
                                KeyCode::Char('l') | KeyCode::Char('L') => engine.start_game(GameVariant::Letters),
                                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                                _ => {}
                            }
                        }
                        GameStatus::Playing => {
                             match key.code {
                                KeyCode::Char(c) => engine.submit_answer(c),
                                KeyCode::Esc => engine.status = GameStatus::Menu,
                                _ => {}
                            }
                        }
                        GameStatus::Feedback { .. } => {
                            match key.code {
                                KeyCode::Enter => engine.next_level(),
                                KeyCode::Esc => engine.status = GameStatus::Menu,
                                _ => {}
                            }
                        }
                        GameStatus::SessionComplete => {
                             match key.code {
                                KeyCode::Char('m') | KeyCode::Char('M') | KeyCode::Esc => engine.status = GameStatus::Menu,
                                _ => {}
                            }
                        }
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
