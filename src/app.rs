use ratatui::{
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};
use std::{
    io::{stdout, Result},
    process::{Command, Stdio},
};

mod fuzzy;
mod parser;
//mod tmux;

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Search,
}

pub struct App {
    pub items: Vec<Line<'static>>,
    pub filtered_items: Vec<(usize, Line<'static>)>,
    pub selected: usize,
    pub server: Option<Line<'static>>,
    pub status_message: String,
    pub vertical_scroll_state: ScrollbarState,
    pub state: ListState,
    pub mode: AppMode,
    pub search_query: String,
}

impl App {
    pub fn new() -> App {
        let filename = "c:/users/xpvm5843/.ssh/config";
        let servers = parser::parse_ssh_hosts(filename);
        let number_of_items = servers.len();
        let filtered_items: Vec<(usize, Line<'static>)> = servers
            .iter()
            .enumerate()
            .map(|(i, item)| (i, item.clone()))
            .collect();

        App {
            items: servers,
            filtered_items,
            selected: 0,
            server: None,
            status_message: "".to_owned(),
            vertical_scroll_state: ScrollbarState::new(number_of_items),
            state: ListState::default().with_selected(Some(0)),
            mode: AppMode::Normal,
            search_query: String::new(),
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

    pub fn enter_search_mode(&mut self) {
        self.mode = AppMode::Search;
        self.search_query.clear();
        self.selected = 0;
        self.state.select(Some(0));
    }

    pub fn exit_search_mode(&mut self) {
        self.mode = AppMode::Normal;
        self.search_query.clear();
        self.update_filtered_items();
        self.selected = 0;
        self.state.select(Some(0));
    }

    pub fn add_char_to_search(&mut self, c: char) {
        self.search_query.push(c);
        self.update_filtered_items();
        self.selected = 0;
        self.state.select(Some(0));
    }

    pub fn remove_char_from_search(&mut self) {
        self.search_query.pop();
        self.update_filtered_items();
        self.selected = 0;
        self.state.select(Some(0));
    }

    fn update_filtered_items(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_items = self
                .items
                .iter()
                .enumerate()
                .map(|(i, item)| (i, item.clone()))
                .collect();
        } else {
            self.filtered_items = fuzzy::fuzzy_search(&self.items, &self.search_query);
        }

        let content_length = self.filtered_items.len();
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(content_length);
    }

    //tmux::tmux_session();
    pub fn tmux_session(&mut self) -> Result<()> {
        disable_raw_mode()?;
        stdout().execute(Clear(ClearType::All))?;

        if self.filtered_items.is_empty() {
            self.status_message = "No server to connect to".to_string();
            enable_raw_mode()?;
            return Ok(());
        }

        self.server = Some(self.filtered_items[self.selected].1.clone());
        let selected_server = match &self.server {
            Some(server_name) => server_name
                .spans
                .iter()
                .map(|span| span.content.clone())
                .collect::<String>()
                .trim()
                .to_string(),
            None => "".to_string(),
        };
        let ssh_command = format!("ssh {}", &selected_server);

        match Command::new("wt")
            .arg("-w")
            .arg("0")
            .arg("new-tab")
            .arg("cmd")
            .arg("/k")
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
                    self.status_message = format!(
                        "Executed the tmux session : {}\nStdout{}\nStderr{}",
                        &selected_server, stdout_msg, stderr_msg
                    );
                } else {
                    self.status_message = format!(
                        "Didn’ work on : {}\nStdout{}\nStderr{}",
                        &selected_server, stdout_msg, stderr_msg
                    );
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
