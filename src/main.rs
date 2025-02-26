use std::io::{self,Result};
use ratatui::{
    prelude::*,
    crossterm::event::{self, KeyCode, KeyEventKind},
    widgets::*,
    DefaultTerminal,
};

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
                Constraint::Min(1),
                Constraint::Length(9),
            ]);
            let [text, debug] = chunks.areas(frame.area());
            let items: Vec<ListItem> = app
                .items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    if i == app.selected {
                        ListItem::new::<Line<'_>>(item.clone()).style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new::<Line<'_>>(item.clone())
                    }
                })
                .collect();
            let list = List::new(items)
                .block(Block::default().title("List").borders(Borders::ALL));
            app.vertical_scroll_state = app.vertical_scroll_state.content_length(list.len());
            frame.render_stateful_widget(list, text,&mut app.state);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                text,
                &mut app.vertical_scroll_state,
            );
            let status = format!("{}", &app.status_message);

            let debug_message = Paragraph::new(status)
                .block(Block::bordered().title("debug"));
            frame.render_widget(debug_message, debug);

        })?;

        if let event::Event::Key(key) = event::read()? {
            if startup_phase {
                initial_event_ignored += 1;
                if initial_event_ignored >= required_events_to_ignore {
                    startup_phase = false;
                }
            } else if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                terminal.clear()?;
                return Ok(());
            } else {
             match key.code {
                 KeyCode::Char('t') | KeyCode::Up => app.move_up(),
                 KeyCode::Char('s') | KeyCode::Down =>{
                     app.move_down();
                 },
                 KeyCode::Enter => {
                    app.tmux_session()?;
                    terminal.clear()?;
             },
                 _ => {}
             }
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
