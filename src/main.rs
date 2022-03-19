use clap::{Parser};

mod input;
mod count;
mod divide;

fn main() {
    let args = input::Cli::parse();
    match args.command {
        input::Action::Count { path } => count::count(&path),
        input::Action::Divide { path, byte, line } => divide::divide(&path, byte, line),
    }
}


