use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub c: bool,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub l: bool,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub w: bool,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub m: bool,

    #[arg()]
    pub path: Option<String>,
}

impl Args {
    pub fn is_default_option_enabled(&self) -> bool {
        !(self.c || self.l || self.w) && !self.m
    }
    
}
