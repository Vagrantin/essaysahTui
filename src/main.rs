use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    prelude::*,
    widgets::*,
    DefaultTerminal,
};
use std::io::{self, Result};

mod app;

// In main.rs

fn run_render(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = app::App::new();

    let mut startup_phase = true;
    let mut initial_event_ignored = 0;
    let required_events_to_ignore = 1;

    loop {
        // Update scroll state before drawing
        let content_length = match app.mode {
            app::AppMode::FileSelection => app.filtered_files.len(),
            app::AppMode::HostSelection | app::AppMode::Search => app.filtered_hosts.len(),
        };

        app.vertical_scroll_state = app
            .vertical_scroll_state
            .content_length(content_length);

        // All rendering logic should be inside the closure
        terminal.draw(|frame| {
            let chunks = Layout::vertical([
                Constraint::Min(1),    // Main list (files or hosts)
                Constraint::Length(3), // Search/input area
                Constraint::Length(9), // Status/debug area
            ]);
            let [list_area, search_area, debug_area] = chunks.areas(frame.area());

            // 1. First, get all the immutable data from 'app'.
            let (items, list_title) = app.get_current_items_display();
            let current_mode = app.mode.clone();
            let search_query = app.search_query.clone();
            let status_message = app.status_message.clone();

            let list = List::new(items)
                .block(Block::default().title(list_title).borders(Borders::ALL));

            // 2. Now, perform all the mutable operations on 'app'
            //    in a separate, subsequent action.
            frame.render_stateful_widget(list, list_area, &mut app.state);

            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                list_area,
                &mut app.vertical_scroll_state,
            );

            // Search/Input area (use the cloned variables)
            let (search_style, search_text) = match current_mode {
                app::AppMode::FileSelection => {
                    if !search_query.is_empty() {
                        (
                            Style::default().fg(Color::Yellow),
                            format!("Filter: {}_", search_query)
                        )
                    } else {
                        (
                            Style::default(),
                            "Type to filter files, Enter to select".to_string()
                        )
                    }
                }
                app::AppMode::HostSelection => {
                    (
                        Style::default(),
                        "Press '/' to search hosts, Enter to connect, Esc to go back".to_string()
                    )
                }
                app::AppMode::Search => {
                    (
                        Style::default().fg(Color::Yellow),
                        format!("Search: {}_", search_query)
                    )
                }
            };

            let search_widget = Paragraph::new(search_text)
                .style(search_style)
                .block(Block::bordered().title("Actions"));
            frame.render_widget(search_widget, search_area);

            // Status area (use the cloned variables)
            let status = match current_mode {
                app::AppMode::FileSelection => {
                    format!("File Selection - q: quit, type to filter\n{}", &status_message)
                }
                app::AppMode::HostSelection => {
                    format!(
                        "Host Selection - q: quit, /: search, Enter: connect, Esc: back\n{}",
                        &status_message
                    )
                }
                app::AppMode::Search => {
                    format!(
                        "Search Mode - ESC: exit search, Enter: connect\n{}",
                        &status_message
                    )
                }
            };

            let debug_message = Paragraph::new(status).block(Block::bordered().title("Status"));
            frame.render_widget(debug_message, debug_area);
        })?;

        if let event::Event::Key(key) = event::read()? {
            if startup_phase {
                initial_event_ignored += 1;
                if initial_event_ignored >= required_events_to_ignore {
                    startup_phase = false;
                }
            } else if key.kind == KeyEventKind::Press {
                match app.mode {
                    app::AppMode::FileSelection => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            terminal.clear()?;
                            return Ok(());
                        }
                        KeyCode::Up => app.move_up(),
                        KeyCode::Down => app.move_down(),
                        KeyCode::Enter => {
                            app.load_hosts_from_selected_file();
                        }
                        KeyCode::Backspace => {
                            app.remove_char_from_search();
                        }
                        KeyCode::Char(c) => {
                            app.add_char_to_search(c);
                        }
                        _ => {}
                    },
                    app::AppMode::HostSelection => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            terminal.clear()?;
                            return Ok(());
                        }
                        KeyCode::Char('/') => {
                            app.enter_search_mode();
                        }
                        KeyCode::Up => app.move_up(),
                        KeyCode::Down => app.move_down(),
                        KeyCode::Enter => {
                            app.tmux_session()?;
                            terminal.clear()?;
                        }
                        KeyCode::Esc => {
                            app.back_to_file_selection();
                        }
                        _ => {}
                    },
                    app::AppMode::Search => match key.code {
                        KeyCode::Esc => {
                            app.exit_search_mode();
                        }
                        KeyCode::Enter => {
                            if !app.filtered_hosts.is_empty() {
                                app.tmux_session()?;
                                terminal.clear()?;
                            }
                        }
                        KeyCode::Up => app.move_up(),
                        KeyCode::Down => app.move_down(),
                        KeyCode::Backspace => {
                            app.remove_char_from_search();
                        }
                        KeyCode::Char(c) => {
                            app.add_char_to_search(c);
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run_render(terminal);
    ratatui::restore();
    app_result
}
