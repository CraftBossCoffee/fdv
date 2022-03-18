use clap::{Parser};
use fdv::{divide};

mod input;
mod count;

fn main() {
    let args = input::Cli::parse();
    // create_big_file();
    // create_small_file();
    match args.command {
        input::Action::Count { path } => count::count(&path),
        input::Action::Divide { path, byte, line } => divide(&path, byte, line),
        input::Action::Find { path, word } => todo!(),
        input::Action::Grep { path, word, replace } => todo!(),
    }
}
