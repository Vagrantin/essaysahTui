use std:: {
    io::{stdout,stderr,Result,Write},
    process::{Stdio,Command},
    str
};
use ratatui::{
    prelude::*,
    widgets::*,
    crossterm::{
        terminal::{Clear,ClearType,disable_raw_mode, enable_raw_mode},
        ExecutableCommand,
    },
};

mod parser;
//mod tmux;

pub struct App {
   pub items: Vec<Line<'static>>,
   pub selected: usize,
   pub server: Option<Line<'static>>,
   pub status_message: String,
   pub vertical_scroll_state: ScrollbarState,
   pub state: ListState,
}

impl App {
   pub fn new() -> App {
            let filename = ".ssh/config";
            let servers = parser::parse_ssh_hosts(filename);
            let number_of_items = servers.len();
        App {
            items: servers,
            selected: 0,
            server: None, 
            status_message: "".to_owned(),
            vertical_scroll_state: ScrollbarState::new(number_of_items),
            state: ListState::default().with_selected(Some(0))

        }
    }

    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        self.state.select(Some(self.selected));
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.selected);
    }

    pub fn move_down(&mut self) {
        if self.selected < self.items.len() - 1 {
            self.selected += 1;
        }
        self.state.select(Some(self.selected));
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.selected);
    }
    //tmux::tmux_session();
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
        let ssh_command = format!("ssh root@{}", &selected_server);
        //let ssh_command = format!("ssh xpvm5843@{}", &selected_server);
        //let ssh_command = format!("ping google.com");

        match Command::new("tmux")
            .arg("new")
            .arg("-s")
            .arg(&selected_server)
            //.arg(format!("root@{}",&selected_server))
            .arg(ssh_command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            { 
                
                Ok(child) => {
                    let output = child.wait_with_output()?;

                    let stderr_msg = String::from_utf8_lossy(&output.stderr);
                    let stdout_msg = String::from_utf8_lossy(&output.stdout);

                    if output.status.success() {
                    self.status_message = format!("Executed the tmux session : {}\nStdout{}\nStderr{}",
                                                  &selected_server,
                                                  stdout_msg,
                                                  stderr_msg
                                                  );
                } else {
                    self.status_message = format!("Didn't work on server {}", &selected_server);
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

