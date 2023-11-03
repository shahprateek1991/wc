use std::{fs, io};

use args::Args;
use clap::Parser;

mod args;

fn main() {
    let args = Args::parse();

    let content = get_content(&args.path);

    let mut result = Result {
        byte_size: 0,
        lines: 0,
        words: 0,
        characters: 0,
    };

    if args.c || args.is_default_option_enabled() {
        result.byte_size = content.len();
    }

    for line in content.lines() {
        if args.l || args.is_default_option_enabled() {
            result.lines = result.lines + 1;
        }

        if args.w || args.is_default_option_enabled() {
            result.words = result.words + line.split_whitespace().count();
        }

        if args.m {
            result.characters = result.characters + line.chars().count();
        }
    }

    result.print(args);
}

fn get_content(file_path: &Option<String>) -> String {
    let content = match file_path {
        Some(ref file_name) => {
            fs::read_to_string(file_name).expect("Should have been able to read the file")
        }
        None => get_content_from_std_input(),
    };
    content
}

fn get_content_from_std_input() -> String {
    let mut s = "".to_string();
    for line in io::stdin().lines() {
        s.push_str(line.unwrap().as_str());
        s.push_str("\n");
    }
    s
}

struct Result {
    byte_size: usize,
    lines: usize,
    words: usize,
    characters: usize,
}

impl Result {
    fn print(&self, args: Args) {
        if args.c || args.is_default_option_enabled() {
            print!("{} ", self.byte_size);
        }
    
        if args.l || args.is_default_option_enabled() {
            print!("{} ", self.lines);
        }
    
        if args.w || args.is_default_option_enabled() {
            print!("{} ", self.words);
        }
    
        if args.m {
            print!("{} ", self.characters);
        }
    
        println!("{}", args.path.unwrap_or("".to_string()));
    }
    
}
