use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Modifier},
    text::{Span, Line},
        widgets::{Block, Borders, Paragraph},
        Frame,
    };
    use letterlanders_core::{GameEngine, GameStatus};
    use letterlanders_core::settings::InputMethod;
    

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
        .split(f.area());

    let title = Paragraph::new("LetterLanders TUI")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    match &engine.status {
        GameStatus::Menu => {
            // ... existing menu code ...
            let menu_text = vec![
                Line::from("Welcome to LetterLanders!"),
                Line::from(""),
                Line::from("Press 'N' for Numbers"),
                Line::from("Press 'L' for Letters"),
                Line::from("Press 'S' for Settings"),
                Line::from("Press 'Q' to Quit"),
            ];
            let p = Paragraph::new(menu_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Menu"));
            f.render_widget(p, chunks[1]);
        }
        GameStatus::Settings { message } => {
            let s = &engine.settings;
            let input_method_str = match s.input_method {
                InputMethod::DirectKeyboard => "Direct Keyboard",
                InputMethod::ArrowSelection => "Arrow Selection",
                InputMethod::Hybrid => "Hybrid (Arrows + Type)",
            };
            
            let mut settings_text = vec![
                Line::from("SETTINGS"),
                Line::from(""),
                Line::from(format!("1. Show Target Visual: {} (Press '1' to toggle)", s.show_target_visual)),
                Line::from(format!("2. Feedback Duration: {}s (Press '2' to cycle)", s.feedback_duration_seconds)),
                Line::from(format!("3. Input Method: {} (Press '3' to toggle)", input_method_str)),
                Line::from("4. Export Settings (Press '4' - shares config)"),
                Line::from("5. Import Settings (Press '5' - note: not backwards compatible)"),
                Line::from(""),
                Line::from("Press 'Esc' to Save & Back"),
            ];

            if let Some(msg) = message {
                settings_text.push(Line::from(""));
                settings_text.push(Line::from(Span::styled(
                    format!("Msg: {}", msg),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )));
            }

            let p = Paragraph::new(settings_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Settings"));
            f.render_widget(p, chunks[1]);
        }
        GameStatus::Playing => {
            if let Some(session) = &engine.session {
                let mut content = vec![
                    Line::from(format!("Level {}/{}", session.current_level, if session.current_level > 3 { "BOSS" } else { "3" })),
                ];
                
                if let Some(limit) = session.level_time_limit {
                     let remaining = limit.saturating_sub(session.level_elapsed_time);
                     let style = if remaining <= 5 { Style::default().fg(Color::Red).add_modifier(Modifier::BOLD) } else { Style::default() };
                     content.push(Line::from(Span::styled(format!("TIME: {}s", remaining), style)));
                }
                
                content.push(Line::from(""));
                content.push(Line::from("Listen to the sound..."));
                
                match engine.settings.input_method {
                    InputMethod::DirectKeyboard => {
                         content.push(Line::from("Press the matching key on your keyboard!"));
                    }
                    InputMethod::ArrowSelection => {
                         content.push(Line::from("Use Left/Right Arrow keys to select, Enter to confirm!"));
                    }
                    InputMethod::Hybrid => {
                         content.push(Line::from("Type the character OR use Arrows to select!"));
                    }
                }
                content.push(Line::from(""));
                
                if engine.settings.show_target_visual {
                    content.insert(0, Line::from(format!("Target: {}", session.target)));
                }

                // Render Options
                let mut options_spans = vec![];
                for (i, option) in session.options.iter().enumerate() {
                    let is_selected = (engine.settings.input_method == InputMethod::ArrowSelection || engine.settings.input_method == InputMethod::Hybrid) 
                        && i == session.selected_index;
                    
                    let style = if is_selected {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::REVERSED)
                    } else {
                        Style::default()
                    };

                    if i > 0 {
                        options_spans.push(Span::raw("   "));
                    }
                    options_spans.push(Span::styled(format!(" {} ", option), style));
                }
                content.push(Line::from(options_spans));

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
            ];
            let p = Paragraph::new(text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Feedback"));
            f.render_widget(p, chunks[1]);
        }
        GameStatus::LevelComplete { level: _, score, passed } => {
             let title = if *passed { "LEVEL COMPLETE!" } else { "LEVEL FAILED" };
             let color = if *passed { Color::Green } else { Color::Red };
             let mut text = vec![
                Line::from(Span::styled(title, Style::default().fg(color).add_modifier(Modifier::BOLD))),
                Line::from(""),
                Line::from(format!("Score: {}/5", score)),
                Line::from(""),
             ];
             
             if *passed {
                 text.push(Line::from("Press Enter to start next level"));
             } else {
                 text.push(Line::from("Press Enter to retry level"));
             }
             
             let p = Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Summary"));
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
