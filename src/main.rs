use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    prelude::*,
    widgets::*,
    DefaultTerminal,
};
use std::io::{self, Result};

mod app;

fn run_render(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = app::App::new();

    //For MS windows weird behaviour we want to ignore
    //the first couple of event, otherwise it triggers
    //the connection to the first element in the list
    let mut startup_phase = true;
    let mut initial_event_ignored = 0;
    let required_events_to_ignore = 1;

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::vertical([
                Constraint::Min(1),    // Server list
                Constraint::Length(3), // Search
                Constraint::Length(9), // Debug
            ]);
            let [list_area, search_area, debug_area] = chunks.areas(frame.area());

            let items: Vec<ListItem> = app
                .filtered_items
                .iter()
                .enumerate()
                .map(|(i, (_, item))| {
                    if i == app.selected {
                        ListItem::new::<Line<'_>>(item.clone())
                            .style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new::<Line<'_>>(item.clone())
                    }
                })
                .collect();

            let list_title = if app.mode == app::AppMode::Search {
                format!("SSH Hosts (Filtered: {})", app.filtered_items.len())
            } else {
                "SSH Hosts".to_string()
            };

            let list =
                List::new(items).block(Block::default().title(list_title).borders(Borders::ALL));

            app.vertical_scroll_state = app
                .vertical_scroll_state
                .content_length(app.filtered_items.len());
            frame.render_stateful_widget(list, list_area, &mut app.state);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                list_area,
                &mut app.vertical_scroll_state,
            );

            let search_style = if app.mode == app::AppMode::Search {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };

            let search_text = if app.mode == app::AppMode::Search {
                format!("Search: {}_", app.search_query)
            } else {
                "Press '/' to search".to_string()
            };

            let search_widget = Paragraph::new(search_text)
                .style(search_style)
                .block(Block::bordered().title("Search"));
            frame.render_widget(search_widget, search_area);

            let status = if app.mode == app::AppMode::Search {
                format!("Search mode - ESC to exit\n{}", &app.status_message)
            } else {
                format!(
                    "Normal mode - q: quit, /: search, Enter: connect\n{}",
                    &app.status_message
                )
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
                    app::AppMode::Normal => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            terminal.clear()?;
                            return Ok(());
                        }
                        KeyCode::Char('/') => {
                            app.enter_search_mode();
                        }
                        KeyCode::Char('t') | KeyCode::Up => app.move_up(),
                        KeyCode::Char('s') | KeyCode::Down => app.move_down(),
                        KeyCode::Enter => {
                            app.tmux_session()?;
                            terminal.clear()?;
                        }
                        _ => {}
                    },
                    app::AppMode::Search => match key.code {
                        KeyCode::Esc => {
                            app.exit_search_mode();
                        }
                        KeyCode::Enter => {
                            if !app.filtered_items.is_empty() {
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
