# sshTUI

A terminal-based SSH connection manager that reads hosts from your SSH config file and allows quick connections via a convenient TUI interface.

## Features

- Elegant terminal UI using Ratatui
- Parses hosts from your SSH config file
- Navigable list with keyboard controls
- Opens connections in Windows Terminal tabs
- Visual selection with highlighting and scrollbar

## Installation

### Prerequisites

- Rust and Cargo installed
- Windows Terminal (`wt`)
- OpenSSH

### Building from Source

```bash
git clone https://github.com/Vagrantin/sshTUI.git
cd sshTUI
cargo build --release
```

The compiled binary will be available at `target/release/sshTUI`.

## Usage

1. Make sure your SSH config file contains Host entries in the standard format:
   ```
   Host myserver
       HostName 192.168.1.100
       User myuser
   ```

2. Run the application in Powershell or CMD in Windows Terminal:
   ```
   ./sshTUI
   ```

3. Navigate the list using the following controls:
   - `Up` or `t`: Move selection up
   - `Down` or `s`: Move selection down
   - `Enter`: Connect to the selected host
   - `q` or `Q`: Quit the application

## Configuration

By default, the application looks for SSH config at `c:/users/username/.ssh/config`.

To use a different file, modify the path in `app.rs`.

## Platform-Specific Behavior

Currently, sshTUI only supports Windows and uses Windows Terminal to create new tabs with SSH connections.

## Troubleshooting

If you encounter issues with key navigation or unexpected behavior:
- The application includes code to ignore initial key events that might be generated during startup
- Make sure your SSH config file uses the correct format

## Roadmap

- Split server names in several columns for better organization
- Add search functionality to quickly find servers in the list
- Add WSL2/Linux support

## License

GNU General Public License (GPL)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
