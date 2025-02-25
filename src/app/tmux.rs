use std:: {
    io::{stdout,Result},
    process::Command,
    str
};
use ratatui::
    crossterm::{
        terminal::{Clear,ClearType,disable_raw_mode, enable_raw_mode},
        ExecutableCommand,
    };

pub fn tmux_session(&mut _crate::app::App) -> Result<()>{
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
    let ssh_command = format!("ls && ssh -t XPVM5843@{}", &selected_server);

    match Command::new("tmux")
        .arg("new")
        .arg("-s")
        .arg(&selected_server)
        .arg(ssh_command)
        .output()
        { 
            Ok(output) => {
                let stderr_msg = str::from_utf8(&output.stderr); 
                let stdout_msg = str::from_utf8(&output.stdout); 
                if output.status.success() {
                self.status_message = format!("Executed the tmux session : {}\nStdout: {:?}\nStderr: {:?}", &selected_server, &stdout_msg, &stderr_msg);
            } else {
                self.status_message = format!("Didn't work on server {} -- It's potentially a duplicate session\nStdout: {:#?}\nStderr: {:?}", &selected_server, &stdout_msg, &stderr_msg);
                }
            }
            Err(e) => {
                self.status_message = format!("Failed: {}", e);
            }
        }
    enable_raw_mode()?;
    Ok(())
}
