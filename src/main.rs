use std:: {
    error::Error,
    fs::File,
    io::{self,BufRead,BufReader},
    path::Path
};
use ratatui::{
    prelude::*,
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::*,
    DefaultTerminal,
};

fn servers_from_file(filename: impl AsRef<Path>) -> Vec<Line<'static>> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    let data: Vec<Line> = buf.lines()
        .map(|l| {Line::from(l.unwrap())})
        .collect::<Vec<_>>();
    data
}

fn run_render(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let filename = "servers.conf";
            let servers = servers_from_file(filename);
            let textfilecontent = Text::from(servers);
            let serverslist = Paragraph::new(textfilecontent)
                .white()
                .on_blue();
            frame.render_widget(serverslist, frame.area());
            frame.set_cursor_position(Position::new(0,0));
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                return Ok(());
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
