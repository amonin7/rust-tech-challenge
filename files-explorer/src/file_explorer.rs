use walkdir::{WalkDir, DirEntry};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

pub fn explore_files_recursively(dir: &String, file_extension: &String) {
    let lines_amount = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| ends_with(e, file_extension))
        .fold(0, |acc, e| acc + count_lines(e.path()));

    println!("Total line amount: {}", lines_amount);
}

fn ends_with(e: &DirEntry, file_extension: &String) -> bool {
    return e.file_name()
        .to_string_lossy()
        .ends_with(file_extension);
}

fn count_lines(path: &Path) -> usize {
    return BufReader::new(File::open(path)
        .expect("Failed to open file"))
        .lines()
        .count();
}