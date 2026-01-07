// Import rayon for parallel iteration capabilities
use rayon::prelude::*;
// Import env for accessing command line arguments
use std::env;
// Import Path for handling file system paths
use std::path::Path;
// Import Instant for timing the scan operation
use std::time::Instant;
// Import WalkDir for recursively walking directory trees
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy)]
enum SizeUnit {
    MB,
    GB,
}

// Struct to hold file path and size information
#[derive(Debug)]
struct FileInfo {
    path: String,
    size_bytes: u64,
}

// Parse size string with optional unit suffix (g, gb, m, mb) and return size in MB and display unit
fn parse_size(size_str: &str) -> (f64, SizeUnit) {
    let size_str = size_str.to_lowercase();
    let (num, multiplier, unit) = if size_str.ends_with("gb") {
        (&size_str[..size_str.len() - 2], 1024.0, SizeUnit::GB)
    } else if size_str.ends_with("g") {
        (&size_str[..size_str.len() - 1], 1024.0, SizeUnit::GB)
    } else if size_str.ends_with("mb") {
        (&size_str[..size_str.len() - 2], 1.0, SizeUnit::MB)
    } else if size_str.ends_with("m") {
        (&size_str[..size_str.len() - 1], 1.0, SizeUnit::MB)
    } else {
        (size_str.as_str(), 1.0, SizeUnit::MB)
    };

    (num.parse::<f64>().unwrap_or(100.0) * multiplier, unit)
}

// Display help information with usage examples
fn print_help() {
    println!("list-big-files - Find large files in a directory");
    println!();
    println!("USAGE:");
    println!("    list-big-files [DIRECTORY] [SIZE]");
    println!("    list-big-files --help");
    println!("    list-big-files help");
    println!();
    println!("ARGUMENTS:");
    println!("    DIRECTORY    Path to directory to scan (default: current directory)");
    println!("    SIZE         Minimum file size with optional unit");
    println!("                 - Without unit: interpreted as MB (e.g., 100 = 100MB)");
    println!("                 - With unit: MB or GB (e.g., 50MB, 1GB, 2G, 500M)");
    println!("                 Default: 100MB");
    println!();
    println!("EXAMPLES:");
    println!("    list-big-files /home/user/documents");
    println!("        Scan documents for files >= 100MB (default)");
    println!();
    println!("    list-big-files . 50MB");
    println!("        Scan current directory for files >= 50MB");
    println!();
    println!("    list-big-files /path 1GB");
    println!("        Scan /path for files >= 1GB");
    println!();
    println!("    list-big-files ~/Downloads 200M");
    println!("        Scan Downloads for files >= 200MB");
    println!();
    println!("OUTPUT:");
    println!("    Files are sorted by size (largest first) with scan timing information");
}

fn list_big_files(directory: &Path, min_size_bytes: u64) -> (Vec<FileInfo>, usize) {
    let start = Instant::now();

    let all_files: Vec<_> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .collect();

    let scanned_count = all_files.len();

    let files: Vec<FileInfo> = all_files
        .into_par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            let metadata = path.metadata().ok()?;
            let size_bytes = metadata.len();

            if size_bytes >= min_size_bytes {
                Some(FileInfo {
                    path: path.display().to_string(),
                    size_bytes,
                })
            } else {
                None
            }
        })
        .collect();

    let duration = start.elapsed();
    println!("Scanned in: {:.2}s", duration.as_secs_f64());

    (files, scanned_count)
}

fn format_size(size_bytes: u64, unit: SizeUnit) -> f64 {
    match unit {
        SizeUnit::MB => size_bytes as f64 / (1024.0 * 1024.0),
        SizeUnit::GB => size_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
    }
}

fn get_unit_label(unit: SizeUnit) -> &'static str {
    match unit {
        SizeUnit::MB => "MB",
        SizeUnit::GB => "GB",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for help arguments
    if args.len() > 1 && (args[1] == "--help" || args[1] == "help") {
        print_help();
        return;
    }

    // Parse directory argument, default to current directory if not provided
    let directory = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    // Parse minimum size argument, default to 100MB if not provided
    let (min_size_mb, display_unit) = if args.len() > 2 {
        parse_size(&args[2])
    } else {
        (100.0, SizeUnit::MB)
    };

    let min_size_bytes = (min_size_mb * 1024.0 * 1024.0) as u64;

    // Display scan progress information
    println!(
        "Scanning {:?} for files >= {} {}...\n",
        directory,
        format_size(min_size_bytes, display_unit),
        get_unit_label(display_unit)
    );

    // Scan for large files and sort results by size (largest first)
    let (mut files, scanned_count) = list_big_files(directory, min_size_bytes);
    files.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

    // Print table header for results
    println!(
        "{:<15} Path",
        format!("Size ({})", get_unit_label(display_unit))
    );
    println!("{}", "-".repeat(80));

    // Iterate and display each file with formatted output
    for file in &files {
        println!(
            "{:>14.2}  {}",
            format_size(file.size_bytes, display_unit),
            file.path
        );
    }

    // Display total count of large files found and total files scanned
    println!(
        "\nTotal: {} files (scanned {} files)",
        files.len(),
        scanned_count
    );
}
