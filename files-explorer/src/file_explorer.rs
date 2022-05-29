use walkdir::{WalkDir, DirEntry};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

/// A function that given a directory, recursively finds all files with a
/// given file extension in that directory and all sub-directories,
/// and counts the number of lines in the file and prints it to stdout.
pub fn explore_files_recursively(dir: &String, file_extension: &String) {
    let lines_amount = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| ends_with(e, file_extension))
        .fold(0, |acc, e| acc + count_lines(e.path()));

    println!("Total line amount: {}", lines_amount);
}

/// Function to filter out the filenames, that only have a given file extension at the end
fn ends_with(e: &DirEntry, file_extension: &String) -> bool {
    return e.file_name()
        .to_string_lossy()
        .ends_with(file_extension);
}

/// Function to count lines in the file, located at the provided filepath
fn count_lines(path: &Path) -> usize {
    return BufReader::new(File::open(path)
        .expect("Failed to open file"))
        .lines()
        .count();
}