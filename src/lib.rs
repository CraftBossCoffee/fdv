use chrono::Local;
use std::path::{Path, PathBuf};
use std::fs::{self, Metadata, File};
use std::io::{self, BufRead, BufReader, Write};

mod input;
mod count;
mod constants;

pub enum DivideType{
    Byte(u64),
    Line(u64),
    Both{b: u64, l: u64},
}

impl DivideType{
    pub fn new(byte: Option<u64>, line: Option<u64>) -> Self{
        if let (Some(b), None) = (byte, line) {
            Self::Byte(b)
        }else if let(None, Some(l)) = (byte, line) {
            Self::Line(l)
        }else if let(Some(b), Some(l)) = (byte, line) {
            Self::Both { b, l }
        }else{
            panic!("must set either or both of `-b`(dividing by bytes) and `-l`(dividing by lines).");
        }
    }

    pub fn inform_limit_over(&self, byte: &u64, line: &u64) -> bool{
        match self {
            Self::Byte(b) => {
                b <= byte
            },
            Self::Line(l) => {
                l <= line
            },
            Self::Both { b, l} => {
                b <= byte || l <= line
            }
        }
    }
}

pub fn get_zero_padding(padding: char, i: usize, digit: usize) -> String{
    let iter_root = std::iter::repeat(padding).take(digit);
    let mut root = String::from_iter(iter_root);

    root.push_str(&i.to_string()[..]);

    let iter_result = root.chars().collect::<Vec<char>>();
    String::from_iter(iter_result[iter_result.len() - digit..].iter())
}

pub fn get_padded_filename(path : &PathBuf, parent: &str, i: usize) -> String{
    let stem = path.as_path().file_stem().unwrap().to_string_lossy();
    let extension = path.as_path().extension().unwrap().to_string_lossy();
    let dir = path.as_path().parent().unwrap().to_string_lossy();

    format!("{}/{}/{}_{}.{}", dir, parent, stem, get_zero_padding('0', i, 8), extension)
}

pub fn get_parent_path(path: &PathBuf) -> String{
    path.as_path().parent().unwrap().to_string_lossy().to_string()
}

pub fn get_now_time() -> String{
    let dt = Local::now();
    format!("{}", dt.format("%Y%m%d_%H%M%S"))
}

pub fn divide(path: &PathBuf, byte: Option<u64>, line: Option<u64>) {

    let dt = Local::now();

    let dt = format!("{}", dt.format("%Y%m%d_%H%M%S"));
    let file = File::open(path).unwrap();

    let reader = BufReader::new(file);
    let divide_type = DivideType::new(byte, line);


    let mut file_count = 1;
    let mut now_bytes = 0;
    let mut now_lines = 0;

    let parent = &format!("result{}", get_now_time())[..];
    if Path::new(&(get_parent_path(path) + "/" + parent)).exists(){
        println!("info: folder aleady exists. skip create a folder.");
    }else{
        match fs::create_dir(get_parent_path(path) + "/" + parent){
            Ok(_) => {},
            Err(msg) => panic!("failed make a directory named {}, cause by {}", parent, msg),
        }
    }

    let mut write_file = File::create(get_padded_filename(path, parent, 1)).expect("failed to make a file.");
    for line_result in reader.lines(){
        if divide_type.inform_limit_over(&now_bytes, &now_lines){
            file_count += 1;
            write_file = File::create(get_padded_filename(path, parent, file_count)).expect(&format!("failed to make {}th file file.", file_count)[..]);
            now_bytes = 0;
            now_lines = 0;
        }

        let line = match line_result {
            Ok(s) => {
                let mut s = s;
                s.push_str(constants::NEW_LINE_CHARACTER_CRLF);
                s
            }
            Err(msg)=> panic!("failed to read a line. caused by {:?}", msg)
        };

        let write_bytes = &line[..].as_bytes();
        write_file.write_all(&write_bytes);
        now_lines += 1;
        now_bytes += write_bytes.len() as u64;
    }



}

// pub fn create_big_file(){
//     let mut file = File::create("20_000_000.text").unwrap();
//     for i in 0..20_000_000{
//         file.write_all("sentence sentence sentence. sentence sentencesentence.\r\n".as_bytes());
//     }
// }

// pub fn create_small_file(){
//     let mut file = File::create("10.text").unwrap();
//     for i in 0..10{
//         file.write_all("sentence sentence sentence. sentence sentencesentence.\r\n".as_bytes());
//     }

// }