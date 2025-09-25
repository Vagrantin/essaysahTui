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

#[derive(PartialEq, Clone)]
pub enum AppMode {
    FileSelection,
    HostSelection,
    Search,
}

pub struct App {
    pub files: Vec<parser::FileEntry>,
    pub filtered_files: Vec<(usize, parser::FileEntry)>,
    pub hosts: Vec<Line<'static>>,
    pub filtered_hosts: Vec<(usize, Line<'static>)>,
    pub selected: usize,
    pub current_file: Option<parser::FileEntry>,
    pub status_message: String,
    pub vertical_scroll_state: ScrollbarState,
    pub state: ListState,
    pub mode: AppMode,
    pub search_query: String,
    #[allow(dead_code)]
    pub config_folder: String,
}

impl App {
    pub fn new() -> App {
        let config_folder = "c:/users/xpvm5843/.ssh/config.d".to_string(); // Default folder
        let files = parser::get_files_in_folder(&config_folder);
        let number_of_files = files.len();
        
        let filtered_files: Vec<(usize, parser::FileEntry)> = files
            .iter()
            .enumerate()
            .map(|(i, item)| (i, item.clone()))
            .collect();

        App {
            files,
            filtered_files,
            hosts: Vec::new(),
            filtered_hosts: Vec::new(),
            selected: 0,
            current_file: None,
            status_message: format!("Loaded {} files from {}", number_of_files, config_folder),
            vertical_scroll_state: ScrollbarState::new(number_of_files),
            state: ListState::default().with_selected(Some(0)),
            mode: AppMode::FileSelection,
            search_query: String::new(),
            config_folder,
        }
    }

    pub fn load_hosts_from_selected_file(&mut self) {
        if self.filtered_files.is_empty() {
            self.status_message = "No file selected".to_string();
            return;
        }

        let selected_file = &self.filtered_files[self.selected].1;
        self.current_file = Some(selected_file.clone());
        
        self.hosts = parser::parse_ssh_hosts(&selected_file.path);
        self.filtered_hosts = self.hosts
            .iter()
            .enumerate()
            .map(|(i, item)| (i, item.clone()))
            .collect();

        let number_of_hosts = self.hosts.len();
        self.vertical_scroll_state = ScrollbarState::new(number_of_hosts);
        self.selected = 0;
        self.state.select(Some(0));
        self.mode = AppMode::HostSelection;
        
        self.status_message = format!(
            "Loaded {} hosts from {}",
            number_of_hosts,
            selected_file.name
        );
    }

    pub fn back_to_file_selection(&mut self) {
        self.mode = AppMode::FileSelection;
        self.search_query.clear();
        self.hosts.clear();
        self.filtered_hosts.clear();
        self.current_file = None;
        self.selected = 0;
        self.state.select(Some(0));
        self.vertical_scroll_state = ScrollbarState::new(self.files.len());
        self.update_filtered_files();
    }

    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        self.state.select(Some(self.selected));
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.selected);
    }

    pub fn move_down(&mut self) {
        let max_items = match self.mode {
            AppMode::FileSelection => self.filtered_files.len(),
            AppMode::HostSelection | AppMode::Search => self.filtered_hosts.len(),
        };
        
        if self.selected < max_items.saturating_sub(1) {
            self.selected += 1;
        }
        self.state.select(Some(self.selected));
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.selected);
    }

    pub fn enter_search_mode(&mut self) {
        if self.mode == AppMode::HostSelection {
            self.mode = AppMode::Search;
            self.search_query.clear();
            self.selected = 0;
            self.state.select(Some(0));
        }
    }

    pub fn exit_search_mode(&mut self) {
        if self.mode == AppMode::Search {
            self.mode = AppMode::HostSelection;
            self.search_query.clear();
            self.update_filtered_hosts();
            self.selected = 0;
            self.state.select(Some(0));
        }
    }

    pub fn add_char_to_search(&mut self, c: char) {
        match self.mode {
            AppMode::FileSelection => {
                self.search_query.push(c);
                self.update_filtered_files();
            }
            AppMode::Search => {
                self.search_query.push(c);
                self.update_filtered_hosts();
            }
            _ => {}
        }
        self.selected = 0;
        self.state.select(Some(0));
    }

    pub fn remove_char_from_search(&mut self) {
        match self.mode {
            AppMode::FileSelection => {
                self.search_query.pop();
                self.update_filtered_files();
            }
            AppMode::Search => {
                self.search_query.pop();
                self.update_filtered_hosts();
            }
            _ => {}
        }
        self.selected = 0;
        self.state.select(Some(0));
    }

    fn update_filtered_files(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_files = self
                .files
                .iter()
                .enumerate()
                .map(|(i, item)| (i, item.clone()))
                .collect();
        } else {
            // Simple string matching for files
            self.filtered_files = self
                .files
                .iter()
                .enumerate()
                .filter(|(_, file)| {
                    file.name
                        .to_lowercase()
                        .contains(&self.search_query.to_lowercase())
                })
                .map(|(i, file)| (i, file.clone()))
                .collect();
        }

        let content_length = self.filtered_files.len();
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(content_length);
    }

    fn update_filtered_hosts(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_hosts = self
                .hosts
                .iter()
                .enumerate()
                .map(|(i, item)| (i, item.clone()))
                .collect();
        } else {
            self.filtered_hosts = fuzzy::fuzzy_search(&self.hosts, &self.search_query);
        }

        let content_length = self.filtered_hosts.len();
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(content_length);
    }

    // CHANGED: Return owned data instead of borrowed data
    pub fn get_current_items_display(&self) -> (Vec<ListItem<'static>>, String) {
        match self.mode {
            AppMode::FileSelection => {
                let items: Vec<ListItem<'static>> = self
                    .filtered_files
                    .iter()
                    .enumerate()
                    .map(|(i, (_, file))| {
                        let display_text = Line::from(file.name.clone());
                        if i == self.selected {
                            ListItem::new(display_text)
                                .style(Style::default().fg(Color::Yellow))
                        } else {
                            ListItem::new(display_text)
                        }
                    })
                    .collect();
                
                let title = if !self.search_query.is_empty() {
                    format!("Config Files (Filtered: {})", self.filtered_files.len())
                } else {
                    "Config Files".to_string()
                };
                
                (items, title)
            }
            AppMode::HostSelection | AppMode::Search => {
                let items: Vec<ListItem<'static>> = self
                    .filtered_hosts
                    .iter()
                    .enumerate()
                    .map(|(i, (_, item))| {
                        // Clone the Line content to create owned data
                        let owned_line = Line::from(
                            item.spans
                                .iter()
                                .map(|span| span.content.clone().into_owned())
                                .collect::<Vec<String>>()
                                .join("")
                        );
                        
                        if i == self.selected {
                            ListItem::new(owned_line)
                                .style(Style::default().fg(Color::Yellow))
                        } else {
                            ListItem::new(owned_line)
                        }
                    })
                    .collect();

                let title = if self.mode == AppMode::Search {
                    format!("SSH Hosts (Filtered: {})", self.filtered_hosts.len())
                } else {
                    format!("SSH Hosts - {}", 
                        self.current_file
                            .as_ref()
                            .map(|f| f.name.as_str())
                            .unwrap_or("Unknown"))
                };
                
                (items, title)
            }
        }
    }

    pub fn tmux_session(&mut self) -> Result<()> {
        disable_raw_mode()?;
        stdout().execute(Clear(ClearType::All))?;

        if self.filtered_hosts.is_empty() {
            self.status_message = "No host to connect to".to_string();
            enable_raw_mode()?;
            return Ok(());
        }

        let selected_host = self.filtered_hosts[self.selected].1.clone();
        let selected_server = selected_host
            .spans
            .iter()
            .map(|span| span.content.clone())
            .collect::<String>()
            .trim()
            .to_string();

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
                        "Connected to: {}\nStdout: {}\nStderr: {}",
                        &selected_server, stdout_msg, stderr_msg
                    );
                } else {
                    self.status_message = format!(
                        "Failed to connect to: {}\nStdout: {}\nStderr: {}",
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
