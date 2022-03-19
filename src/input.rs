use clap::{Parser, Subcommand};
use std::{path::PathBuf};


#[derive(Parser)]
#[clap(
    name = "File Divider EX",
    version = "0.0.1",
    author = "author : mocnamo jimukyoku",
    long_about = "It aim to divide a large file into smaller files. It can also count the number of lines in the file.")]
pub struct Cli {
    /// Process type
    #[clap(subcommand)]
    pub command: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {

    /// count the number of lines contained in a file.
    Count {
        /// File to be searched(required)
        #[clap(short, long)]
        path: PathBuf,
    },

    /// split a file by the specified size.
    /// Be sure to specify only one of the number of lines(-l) or the number of bytes(-b).
    Divide {
        /// File to be searched(required)
        #[clap(short, long)]
        path: PathBuf,

        /// Maximum bytes per file
        #[clap(long, short)]
        byte: Option<u64>,

        /// Maximum lines per file
        #[clap(long, short)]
        line: Option<u64>,
    },
}