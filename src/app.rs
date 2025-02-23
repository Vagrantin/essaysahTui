use std:: {
    io::{stdout,Result},
    process::Command
};
use ratatui::{
    prelude::*,
    crossterm::{
        terminal::{Clear,ClearType,disable_raw_mode, enable_raw_mode},
        ExecutableCommand,
    },
};

mod parser;

pub struct App {
   pub items: Vec<Line<'static>>,
   pub selected: usize,
   pub server: Option<Line<'static>>,
   pub status_message: String
}

impl App {
   pub fn new() -> App {
            let filename = "spd";
            let servers = parser::parse_ssh_hosts(filename);
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
               Some(server_name) => {
                   server_name.spans.iter()
                       .map(|span| span.content.clone())
                       .collect::<String>()
                       .trim()
                       .to_string()
                       },
               None => "".to_string(),
           };
        match Command::new("tmux")
            .arg("new")
            .arg("-s")
            .arg(&selected_server)
            .status()
            { 
                Ok(status) => {
                    if status.success() {
                    self.status_message = format!("Executed the tmux session : {}", &selected_server);
                } else {
                    self.status_message = format!("Didn't work on server {}:\nIt's potentially a duplicate session ", &selected_server);
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

