use ratatui::text::Line;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn parse_ssh_hosts<P: AsRef<Path>>(filepath: P) -> Vec<Line<'static>> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut hosts = Vec::<Line>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let trimmed = line.trim();

        //Check if line starts with "Host " (case-insensitive)
        let host = trimmed
            .splitn(2, "Host ")
            .nth(1)
            .unwrap_or("")
            .trim()
            .to_string();

        if !host.is_empty() && host.chars().all(char::is_alphanumeric) {
            let host_line = Line::from(host);
            hosts.push(host_line);
        }
    }
    hosts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ssh_hosts() {
        let hosts = parse_ssh_hosts("samples/spd");
        assert!(!hosts.is_empty());
    }

}
