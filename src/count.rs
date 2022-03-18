
use std::{path::PathBuf};
use std::fs::{self, Metadata, File};
use std::io::{self, BufRead, BufReader, Write};

pub fn count_line(file : File) -> io::Result<u64>{
    let mut reader = BufReader::new(file);
    let mut buf = String:: new();

    let mut result = 0u64;
    while reader.read_line(&mut buf)? > 0{
        result += 1;
        buf.clear();
    }
    Ok(result)
}

pub fn count(path : &PathBuf) {
    let file = File::open(path).unwrap();
    let line_len = count_line(file).unwrap();
    println!("This file has {} lines.", line_len);
    let metadata = fs::metadata(path);
    match metadata{
        Ok(data) => {
            println!("This file has {} bytes (from metadata information).", data.len());
        }
        Err(msg) => println!("failed to get the metadata of the file.")
    }
}
