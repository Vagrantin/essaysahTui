use ratatui::text::Line;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
}

impl FileEntry {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }
}

pub fn get_files_in_folder<P: AsRef<Path>>(folder_path: P) -> Vec<FileEntry> {
    let mut files = Vec::new();
    
    match fs::read_dir(folder_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(file_name) = path.file_name() {
                            if let Some(name_str) = file_name.to_str() {
                                files.push(FileEntry::new(
                                    name_str.to_string(),
                                    path.clone()
                                ));
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
        }
    }
    
    // Sort files alphabetically
    files.sort_by(|a, b| a.name.cmp(&b.name));
    files
}

pub fn parse_ssh_hosts<P: AsRef<Path>>(filepath: P) -> Vec<Line<'static>> {
    let file = match File::open(&filepath) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };
    
    let reader = BufReader::new(file);
    let mut hosts = Vec::<Line>::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed = line.trim();

            // Check if line starts with "Host " (case-insensitive)
            if trimmed.to_lowercase().starts_with("host ") {
                let host = trimmed
                    .splitn(2, ' ')
                    .nth(1)
                    .unwrap_or("")
                    .trim()
                    .to_string();

                if !host.is_empty() && !host.contains('*') && !host.contains('?') {
                    let host_line = Line::from(host);
                    hosts.push(host_line);
                }
            }
        }
    }
    hosts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_get_files_in_folder() {
        // Create a temporary directory for testing
        let temp_dir = std::env::temp_dir().join("test_ssh_configs");
        fs::create_dir_all(&temp_dir).unwrap();

        // Create some test files
        let test_files = vec!["config1", "config2", "config3"];
        for file_name in &test_files {
            let file_path = temp_dir.join(file_name);
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "Host test").unwrap();
        }

        let files = get_files_in_folder(&temp_dir);
        assert_eq!(files.len(), test_files.len());
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_parse_ssh_hosts() {
        let temp_dir = std::env::temp_dir().join("test_ssh_parse");
        fs::create_dir_all(&temp_dir).unwrap();
        
        let config_path = temp_dir.join("test_config");
        let mut file = File::create(&config_path).unwrap();
        writeln!(file, "Host server1").unwrap();
        writeln!(file, "    HostName 192.168.1.1").unwrap();
        writeln!(file, "Host server2").unwrap();
        writeln!(file, "    HostName 192.168.1.2").unwrap();

        let hosts = parse_ssh_hosts(&config_path);
        assert_eq!(hosts.len(), 2);
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
