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
