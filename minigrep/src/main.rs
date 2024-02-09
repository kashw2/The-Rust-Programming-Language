use std::env::args;
use std::fs::{read_to_string};
use std::io::Error;

fn main() {
    let args: Vec<String> = args().collect();
    let config: Config = parse_config(&args);

    match config {
        Config { query: Some(query), file_path: Some(file_path) } => {
            let content: Result<String, Error> = read_to_string(file_path);

            if content.is_ok() {
                println!("Found content in file");
            } else {
                println!("File at {} not found", file_path)
            }

            if content.as_ref().unwrap().find(query).is_some() {
                println!("Found {} is file", query);
            } else {
                panic!("Unable to find {} in {}", query, file_path);
            }

            let mut matches: Vec<String> = Vec::new();

            for line in content.unwrap().lines() {
                if line.contains(query) {
                    matches.push(line.to_string())
                }
            }

            for line in matches {
                println!("matched: {}", line.trim());
            }

        }
        _ => eprintln!("Error parsing config")
    }
}

fn parse_config(args: &Vec<String>) -> Config {
    Config { query: args.get(1), file_path: args.get(2) }
}

pub struct Config<'a> {
    pub query: Option<&'a String>,
    pub file_path: Option<&'a String>,
}
