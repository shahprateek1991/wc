use std::{fs, io};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    c: bool,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    l: bool,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    w:bool,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    m: bool,

    #[arg()]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let content = get_content(&args);

    let default_option:bool = !(args.c || args.l || args.w) && !args.m;

    let mut byte_size:usize = 0; let mut lines:usize = 0; let mut words:usize = 0; let mut characters:usize = 0;

    if args.c || default_option {
        byte_size = content.len();
    }

    for line in content.lines() {
        if args.l || default_option {
            lines = lines + 1;
        }

        if args.w || default_option {
            words = words + line.split_whitespace().count();
            
        }

        if args.m {
            characters = characters + line.chars().count();
            
        }
    }

    if args.c || default_option {
        print!("{} ", byte_size);
        
    }

    if args.l || default_option {
        print!("{} ", lines);
        
    }

    if args.w || default_option {
        print!("{} ", words);
        
    }

    if args.m {
        print!("{} ", characters);
        
    }

    println!("{}", args.path.unwrap_or("".to_string()));
}

fn get_content(args: &Args) -> String {
    let content = match args.path {
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
