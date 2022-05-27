use file_explorer::explore_files_recursively;
use std::io;

mod file_explorer;

fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let line_split: Vec<&str> = input.split_whitespace().collect();
                if line_split.len() != 2 {
                    println!("Please provide correct dir and file extension");
                }
                explore_files_recursively(&line_split[0].to_string(), &line_split[1].to_string());
            }
            Err(e) => panic!("Failed to read input {:?}", e)
        }
    }
}
