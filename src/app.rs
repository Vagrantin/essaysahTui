use std:: {
    fs::File,
    io::{self,BufRead,BufReader, stdout,Result,Write},
    path::Path,
    process::Command
};
use ratatui::{
    prelude::*,
    crossterm::{
        terminal::{Clear,ClearType,disable_raw_mode, enable_raw_mode},
        ExecutableCommand,
    },
};

pub struct App {
   pub items: Vec<Line<'static>>,
   pub selected: usize,
   pub server: Option<Line<'static>>,
   pub status_message: String
}

impl App {
   pub fn new() -> App {
            let filename = "servers.conf";
            let servers = servers_from_file(filename);
        App {
            items: servers,
            selected: 0,
            server: None, 
            status_message: "".to_owned()
        }
    }

    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        if self.selected < self.items.len() - 1 {
            self.selected += 1;
        }
    }
    pub fn tmux_session(&mut self) -> Result<()>{
        disable_raw_mode()?;
        stdout().execute(Clear(ClearType::All))?;
        
        self.server = Some(self.items[self.selected].clone());
        let selected_server = match &self.server {
               Some(server_name) => format!("{}", server_name),
               None => "".to_string(),
           };
        let command = "tmux new -s";
        match Command::new("sh")
            .arg("-c")
            .arg(&command)
            .arg(&selected_server)
            .output()
            { 
                Ok(output) => {
                    if output.status.success() {
                    self.status_message = format!("Executed: {} {}", &command, &selected_server);
                } else {
                    let stdout = io::stdout().write_all(&output.stdout).unwrap();
                    let stderr = io::stderr().write_all(&output.stderr).unwrap();
                    self.status_message = format!("didn't work: {}\n stdout {:?}\nstderr{:?}", output.status, stdout, stderr);
                    }
                }
                Err(e) => {
                    self.status_message = format!("Failed: {}", e);
                }
            }
        enable_raw_mode()?;
        Ok(())
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
