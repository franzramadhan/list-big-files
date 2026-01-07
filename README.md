# List Big Files

A fast, parallel file scanning utility that helps you find large files in a directory tree. Built with Rust for performance and efficiency.

## Features

- **Fast scanning**: Uses parallel processing with Rayon for optimal performance
- **Flexible size specifications**: Support for both MB and GB units
- **Recursive search**: Scans entire directory trees
- **Sorted output**: Results are sorted by file size (largest first)
- **Performance timing**: Shows scan duration
- **Help built-in**: Easy-to-use help system

## Installation

### Install from Source

```bash
# Clone the repository
git clone https://github.com/franzramadhan/list-big-files.git
cd list-big-files

# Build and install
cargo install --path .
```

### Build Manually

```bash
# Build in release mode
./build.sh

# Or manually:
cargo build --release
```

The binary will be available at `./target/release/list-big-files`.

## Usage

### Basic Usage

```bash
# Scan current directory for files >= 100MB (default)
list-big-files

# Scan specific directory
list-big-files /path/to/directory

# Scan with custom size (in MB)
list-big-files /path/to/directory 50MB

# Scan with GB specification
list-big-files /path/to/directory 1GB

# Show help
list-big-files --help
# or
list-big-files help
```

### Size Format Options

You can specify file size in multiple formats:

- **Plain number**: Interpreted as MB (e.g., `100` = 100MB)
- **MB suffix**: Explicit megabytes (e.g., `50MB`, `200M`)
- **GB suffix**: Gigabytes (e.g., `1GB`, `2G`, `0.5GB`)

### Examples

```bash
# Find files larger than 500MB in Downloads
list-big-files ~/Downloads 500MB

# Find files larger than 1GB in the entire home directory
list-big-files ~ 1GB

# Find files larger than 10MB in current directory
list-big-files . 10MB

# Scan a specific project directory
list-big-files ~/projects/myapp 200MB
```

## Output

The tool displays:

1. Scan progress information
2. Scanning duration
3. Formatted table with file sizes and paths
4. Total count of files found

Example output:

```
Scanning "~/Downloads" for files >= 100 MB...

Scanned in: 2.34s
Size (MB)       Path
--------------------------------------------------------------------------------
     1048.50  ~/Downloads/video.mp4
      512.75  ~/Downloads/backup.zip
      256.25  ~/Downloads/project.iso

Total: 3 files
```

## Requirements

- Rust 1.70 or later
- Cargo (comes with Rust)

## Building from Source

### Prerequisites

Install Rust using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build Steps

```bash
# Clone the repository
git clone https://github.com/franzramadhan/list-big-files.git
cd list-big-files

# Build in debug mode (faster compilation, slower binary)
cargo build

# Build in release mode (slower compilation, faster binary)
cargo build --release

# Run tests
cargo test

# Run the program
./target/debug/list-big-files .
# or for release build:
./target/release/list-big-files .
```

## Performance

The tool uses:
- **Rayon**: Data parallelism for concurrent file scanning
- **WalkDir**: Efficient directory traversal
- **Optimized filtering**: Early filtering to reduce metadata operations

On a typical SSD drive, you can expect to scan:
- ~10,000 files in 1-2 seconds
- ~100,000 files in 5-10 seconds

Performance varies based on:
- Drive speed (SSD vs HDD)
- File system type
- Directory depth and structure
- Number of files

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Rayon](https://github.com/rayon-rs/rayon) - Data parallelism library
- [WalkDir](https://github.com/BurntSushi/walkdir) - Recursive directory traversal

## Roadmap

Potential future features:
- [ ] Size filtering with multiple units simultaneously
- [ ] Exclude patterns (directories, file extensions)
- [ ] Output to JSON/CSV format
- [ ] Interactive mode for selective deletion
- [ ] Progress bar for large scans
- [ ] Configuration file support
- [ ] Watch mode for continuous monitoring

## Troubleshooting

### Permission Denied

If you get permission errors, ensure you have read access to all directories and files:

```bash
# Fix permissions on macOS/Linux
chmod +r -R /path/to/directory
```

### Slow Performance

For slow performance:
- Ensure you're using an SSD
- Try a more restrictive minimum size
- Consider excluding large directories with known large files (node_modules, etc.)

### Out of Memory

For very large directories (millions of files), the tool may use significant memory. Consider:
- Scanning subdirectories separately
- Increasing your system's swap space
