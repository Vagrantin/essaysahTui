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
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::vertical([
                Constraint::Min(1),
                Constraint::Length(5),
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
            frame.render_widget(list, text);
            let status = format!("{}", &app.status_message);

            let debug_message = Paragraph::new(status)
                .block(Block::bordered().title("debug"));
            frame.render_widget(debug_message, debug);

        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                terminal.clear()?;
                return Ok(());
            } else {
             match key.code {
                 KeyCode::Up => app.move_up(),
                 KeyCode::Char('t') => app.move_up(),
                 KeyCode::Down => app.move_down(),
                 KeyCode::Char('s') => app.move_down(),
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

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run_render(terminal);
    ratatui::restore();
    app_result
}
