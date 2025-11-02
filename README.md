# WebDAV Backup

A fast and reliable Rust tool for backing up directories to WebDAV servers with compression.

## Features

- Basic WebDAV authentication (Basic Auth)
- 7z compression with configurable levels
- Configuration via TOML files

### Notice

- NOT SUPPORT very large directory

## Installation

### From Source

```bash
git clone https://github.com/lalala-233/webdav-backup
cd webdav-backup
cargo install --path .
```

## Quick Start

### 1. Create a `config.toml`

```toml
[webdav]
host = "https://your-webdav-server.com/"   # WebDAV server URL
subfolder = "/dav/xxxx"                    # Optional subfolder on server (default: "backup")
username = "your-username"                 # WebDAV username
password = "your-password"                 # WebDAV password

[backup]
compression_level = 6        # Optional 7z compression level (0-9, clamped to this rang, default: 6)
prefix = "backup"            # Optional File name prefix (default: "backup")
password = "encryption-key"  # 7z encryption password
```

### 2. Run the backup

```bash
webdav-backup /path/to/backup
```

Notice:

- Only support one file or one folder.
- If the subfolder in your server is not existed, it will be create automatically.

## Usage

### Basic Backup

```bash
webdav-backup -c custom_config.toml /path/to/backup
# or
webdav-backup --config custom_config.toml /path/to/backup
```

Notice: If you don't pass `-c` or `--config` flag, it will suppose you pass a `-c config.toml`.

### Show Help

```bash
webdav-backup --help
```

Output:

```text
Usage: webdav-backup [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to the directory to compress

Options:
  -c, --config <CONFIG>  Path to the config file [default: config.toml]
  -h, --help             Print help
  -V, --version          Print version
```

## File Naming

Backup files are automatically named using the pattern:

```text
{prefix}-{timestamp}.zip
```

Example: `backup-20250114-122012.zip`

## Tested WebDAV Services

- **Nutstore (坚果云)**

But theoretically any standard WebDAV server is supported.

## Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run directly
cargo run --release -- /path/to/backup
```

## Requirements

- The latest version of Rust
- A WebDAV server with read and write permissions
- Read access permission for the backup directory

## Security Notes

- Passwords are stored in plaintext in config files
- Use app-specific passwords when possible
- Encrypting backup contents with strong password
- Rotate backup passwords regularly

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.
