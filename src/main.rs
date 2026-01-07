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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    fn create_test_file(dir: &Path, name: &str, size_bytes: usize) -> std::io::Result<()> {
        let file_path = dir.join(name);
        let mut file = File::create(&file_path)?;
        file.write_all(&vec![0u8; size_bytes])?;
        Ok(())
    }

    #[test]
    fn test_parse_size_mb() {
        let (size, unit) = parse_size("100MB");
        assert_eq!(size, 100.0);
        assert_eq!(unit, SizeUnit::MB);
    }

    #[test]
    fn test_parse_size_m() {
        let (size, unit) = parse_size("50M");
        assert_eq!(size, 50.0);
        assert_eq!(unit, SizeUnit::MB);
    }

    #[test]
    fn test_parse_size_mb_lowercase() {
        let (size, unit) = parse_size("100mb");
        assert_eq!(size, 100.0);
        assert_eq!(unit, SizeUnit::MB);
    }

    #[test]
    fn test_parse_size_m_lowercase() {
        let (size, unit) = parse_size("50m");
        assert_eq!(size, 50.0);
        assert_eq!(unit, SizeUnit::MB);
    }

    #[test]
    fn test_parse_size_gb() {
        let (size, unit) = parse_size("1GB");
        assert_eq!(size, 1024.0);
        assert_eq!(unit, SizeUnit::GB);
    }

    #[test]
    fn test_parse_size_g() {
        let (size, unit) = parse_size("2G");
        assert_eq!(size, 2048.0);
        assert_eq!(unit, SizeUnit::GB);
    }

    #[test]
    fn test_parse_size_gb_lowercase() {
        let (size, unit) = parse_size("1gb");
        assert_eq!(size, 1024.0);
        assert_eq!(unit, SizeUnit::GB);
    }

    #[test]
    fn test_parse_size_g_lowercase() {
        let (size, unit) = parse_size("2g");
        assert_eq!(size, 2048.0);
        assert_eq!(unit, SizeUnit::GB);
    }

    #[test]
    fn test_parse_size_no_unit() {
        let (size, unit) = parse_size("100");
        assert_eq!(size, 100.0);
        assert_eq!(unit, SizeUnit::MB);
    }

    #[test]
    fn test_parse_size_invalid() {
        let (size, unit) = parse_size("invalid");
        assert_eq!(size, 100.0);
        assert_eq!(unit, SizeUnit::MB);
    }

    #[test]
    fn test_parse_size_fractional() {
        let (size, unit) = parse_size("0.5GB");
        assert_eq!(size, 512.0);
        assert_eq!(unit, SizeUnit::GB);
    }

    #[test]
    fn test_parse_size_zero() {
        let (size, unit) = parse_size("0");
        assert_eq!(size, 0.0);
        assert_eq!(unit, SizeUnit::MB);
    }

    #[test]
    fn test_parse_size_large_value() {
        let (size, unit) = parse_size("1000GB");
        assert_eq!(size, 1024000.0);
        assert_eq!(unit, SizeUnit::GB);
    }

    #[test]
    fn test_format_size_mb_1mb() {
        let mb_bytes = 1024 * 1024;
        let size = format_size(mb_bytes, SizeUnit::MB);
        assert!((size - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_format_size_mb_100mb() {
        let mb_bytes = 100 * 1024 * 1024;
        let size = format_size(mb_bytes, SizeUnit::MB);
        assert!((size - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_format_size_mb_fractional() {
        let mb_bytes = (1024 * 1024) / 2;
        let size = format_size(mb_bytes, SizeUnit::MB);
        assert!((size - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_format_size_gb_1gb() {
        let gb_bytes = 1024 * 1024 * 1024;
        let size = format_size(gb_bytes, SizeUnit::GB);
        assert!((size - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_format_size_gb_10gb() {
        let gb_bytes = 10 * 1024 * 1024 * 1024;
        let size = format_size(gb_bytes, SizeUnit::GB);
        assert!((size - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_format_size_gb_fractional() {
        let gb_bytes = (1024 * 1024 * 1024) / 2;
        let size = format_size(gb_bytes, SizeUnit::GB);
        assert!((size - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_format_size_zero() {
        let size = format_size(0, SizeUnit::MB);
        assert_eq!(size, 0.0);
    }

    #[test]
    fn test_format_size_large_value() {
        let large_bytes = 1000 * 1024 * 1024 * 1024;
        let size = format_size(large_bytes, SizeUnit::GB);
        assert!((size - 1000.0).abs() < 0.001);
    }

    #[test]
    fn test_get_unit_label_mb() {
        assert_eq!(get_unit_label(SizeUnit::MB), "MB");
    }

    #[test]
    fn test_get_unit_label_gb() {
        assert_eq!(get_unit_label(SizeUnit::GB), "GB");
    }

    #[test]
    fn test_list_big_files_empty_directory() {
        let dir = tempdir().unwrap();
        let (files, scanned_count) = list_big_files(dir.path(), 100 * 1024 * 1024);
        assert_eq!(files.len(), 0);
        assert_eq!(scanned_count, 0);
    }

    #[test]
    fn test_list_big_files_all_small_files() {
        let dir = tempdir().unwrap();
        create_test_file(dir.path(), "small1.txt", 1024).unwrap();
        create_test_file(dir.path(), "small2.txt", 2048).unwrap();
        create_test_file(dir.path(), "small3.txt", 4096).unwrap();

        let (files, scanned_count) = list_big_files(dir.path(), 100 * 1024 * 1024);
        assert_eq!(files.len(), 0);
        assert_eq!(scanned_count, 3);
    }

    #[test]
    fn test_list_big_files_all_large_files() {
        let dir = tempdir().unwrap();
        create_test_file(dir.path(), "large1.txt", 150 * 1024 * 1024).unwrap();
        create_test_file(dir.path(), "large2.txt", 200 * 1024 * 1024).unwrap();

        let (mut files, scanned_count) = list_big_files(dir.path(), 100 * 1024 * 1024);
        files.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
        assert_eq!(files.len(), 2);
        assert_eq!(scanned_count, 2);
        assert!(files[0].size_bytes > files[1].size_bytes);
    }

    #[test]
    fn test_list_big_files_mixed_sizes() {
        let dir = tempdir().unwrap();
        create_test_file(dir.path(), "small.txt", 1024).unwrap();
        create_test_file(dir.path(), "large.txt", 150 * 1024 * 1024).unwrap();
        create_test_file(dir.path(), "medium.txt", 50 * 1024 * 1024).unwrap();
        create_test_file(dir.path(), "huge.txt", 500 * 1024 * 1024).unwrap();

        let (mut files, scanned_count) = list_big_files(dir.path(), 100 * 1024 * 1024);
        files.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
        assert_eq!(files.len(), 2);
        assert_eq!(scanned_count, 4);
        assert!(files[0].size_bytes > files[1].size_bytes);
    }

    #[test]
    fn test_list_big_files_nested_directories() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let nested = subdir.join("nested");
        fs::create_dir(&nested).unwrap();

        create_test_file(dir.path(), "root_file.txt", 150 * 1024 * 1024).unwrap();
        create_test_file(&subdir, "sub_file.txt", 200 * 1024 * 1024).unwrap();
        create_test_file(&nested, "nested_file.txt", 100 * 1024 * 1024).unwrap();

        let (files, scanned_count) = list_big_files(dir.path(), 100 * 1024 * 1024);
        assert_eq!(files.len(), 3);
        assert_eq!(scanned_count, 3);
    }

    #[test]
    fn test_list_big_files_threshold_boundary() {
        let dir = tempdir().unwrap();
        create_test_file(dir.path(), "exactly_100mb.txt", 100 * 1024 * 1024).unwrap();
        create_test_file(dir.path(), "just_under_100mb.txt", 100 * 1024 * 1024 - 1).unwrap();

        let (files, scanned_count) = list_big_files(dir.path(), 100 * 1024 * 1024);
        assert_eq!(files.len(), 1);
        assert_eq!(scanned_count, 2);
        assert_eq!(files[0].size_bytes, 100 * 1024 * 1024);
    }

    #[test]
    fn test_list_big_files_size_threshold_bytes() {
        let dir = tempdir().unwrap();
        create_test_file(dir.path(), "1mb.txt", 1024 * 1024).unwrap();
        create_test_file(dir.path(), "2mb.txt", 2 * 1024 * 1024).unwrap();

        let (files, scanned_count) = list_big_files(dir.path(), 1024 * 1024);
        assert_eq!(files.len(), 2);
        assert_eq!(scanned_count, 2);
    }

    #[test]
    fn test_list_big_files_zero_threshold() {
        let dir = tempdir().unwrap();
        create_test_file(dir.path(), "tiny.txt", 1).unwrap();

        let (files, scanned_count) = list_big_files(dir.path(), 0);
        assert_eq!(files.len(), 1);
        assert_eq!(scanned_count, 1);
    }

    #[test]
    fn test_file_info_contains_correct_data() {
        let dir = tempdir().unwrap();
        let test_size = 150 * 1024 * 1024;
        create_test_file(dir.path(), "test.txt", test_size).unwrap();

        let (files, _) = list_big_files(dir.path(), 100 * 1024 * 1024);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].size_bytes, test_size as u64);
        assert!(files[0].path.contains("test.txt"));
    }
}
