use std:: {
    fs::File,
    io::{self,BufRead,BufReader, stdout},
    path::Path,
    process::Command
};
use ratatui::{
    prelude::*,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{Clear,ClearType,disable_raw_mode, enable_raw_mode},
        ExecutableCommand,
    },
    widgets::*,
    DefaultTerminal,
};

struct App {
    items: Vec<Line<'static>>,
    selected: usize,
    server: Option<Line<'static>>,
    status_message: String
}

impl App {
    fn new() -> App {
            let filename = "servers.conf";
            let servers = servers_from_file(filename);
        App {
            items: servers,
            selected: 0,
            server: None, 
            status_message: "".to_owned()
        }
    }

    fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    fn move_down(&mut self) {
        if self.selected < self.items.len() - 1 {
            self.selected += 1;
        }
    }
    fn print_selected(&mut self) {
        
        disable_raw_mode().unwrap();
        // Clears entire screen and moves cursor to (0,0)
        stdout().execute(Clear(ClearType::All)).unwrap();
        
        self.server = Some(self.items[self.selected].clone());
        let selected_server = match &self.server {
               Some(server_name) => format!("{}", server_name), //the issue is probably here
               None => "".to_string(),
           };
        let command = "tmux new -s";
        match Command::new("sh")
            .arg("-c")
            .arg(&command)
            .arg(&selected_server) //we are not getting this value.
            .output()
            { 
                Ok(_) => {
                    self.status_message = format!("Executed: {} {}", &command, &selected_server);
                }
                Err(e) => {
                    self.status_message = format!("Failed: {}", e);
                }
            }
        enable_raw_mode().unwrap();
    }
}

fn servers_from_file(filename: impl AsRef<Path>) -> Vec<Line<'static>> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    let data: Vec<Line> = buf.lines()
        .map(|l| {Line::from(l.unwrap())})
        .collect::<Vec<_>>();
    data
}

fn run_render(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();
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
            
            let selected_server = match &app.server {
                Some(server_name) => format!("Selected!: {}", server_name),
                None => "".to_string(),
            };
            let status = format!("{}", &app.status_message);

            let debug_message = Paragraph::new(status)
                .block(Block::bordered().title("debug"));
            frame.render_widget(debug_message, debug);

        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                return Ok(());
            } else {
             match key.code {
                 KeyCode::Up => app.move_up(),
                 KeyCode::Char('t') => app.move_up(),
                 KeyCode::Down => app.move_down(),
                 KeyCode::Char('s') => app.move_down(),
                 KeyCode::Enter => {
                     app.print_selected();
                     terminal.clear()?;
                 },
                 _ => {}
             }
            }
        }
    }
}

//this is probably here that you have an issue to trigger Tmux
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run_render(terminal);
    ratatui::restore();
    app_result
}
