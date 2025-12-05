use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Modifier},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use letterlanders_core::{GameEngine, GameStatus, GameVariant};

pub fn draw(f: &mut Frame, engine: &GameEngine) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(5),    // Content
                Constraint::Length(3), // Footer
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("LetterLanders TUI")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    match &engine.status {
        GameStatus::Menu => {
            let menu_text = vec![
                Line::from("Welcome to LetterLanders!"),
                Line::from(""),
                Line::from("Press 'N' for Numbers"),
                Line::from("Press 'L' for Letters"),
                Line::from("Press 'Q' to Quit"),
            ];
            let p = Paragraph::new(menu_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Menu"));
            f.render_widget(p, chunks[1]);
        }
        GameStatus::Playing => {
            if let Some(session) = &engine.session {
                let content = vec![
                    Line::from(format!("Target: {}", session.target)), // Debug aid, usually hidden in audio-only
                    Line::from(""),
                    Line::from("Listen to the sound..."),
                    Line::from("Press the matching key on your keyboard!"),
                    Line::from(""),
                    Line::from(format!("Options: {:?}", session.options)),
                ];
                let p = Paragraph::new(content)
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).title("Challenge"));
                f.render_widget(p, chunks[1]);
            }
        }
        GameStatus::Feedback { success, message } => {
            let color = if *success { Color::Green } else { Color::Red };
            let text = vec![
                Line::from(Span::styled(
                    if *success { "CORRECT!" } else { "WRONG!" },
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(message.as_str()),
                Line::from(""),
                Line::from("Press Enter to Continue"),
            ];
            let p = Paragraph::new(text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Feedback"));
            f.render_widget(p, chunks[1]);
        }
        GameStatus::SessionComplete => {
             if let Some(session) = &engine.session {
                let text = vec![
                    Line::from("Session Complete!"),
                    Line::from(format!("Final Score: {}/{}", session.score, session.total_questions)),
                    Line::from(""),
                    Line::from("Press 'M' for Menu"),
                ];
                 let p = Paragraph::new(text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Game Over"));
                f.render_widget(p, chunks[1]);
             }
        }
    }

    let footer_text = match engine.status {
        GameStatus::Menu => "Use Keyboard to Select",
        _ => "Press 'Esc' to Quit",
    };
    f.render_widget(Paragraph::new(footer_text).alignment(Alignment::Center), chunks[2]);
}
