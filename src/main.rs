extern crate byteorder;
mod parser;
use parser::local_file_header::*;
use parser::tag::*;
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Requires at least 1 path argument!")
    }

    let path = Path::new(&args[1]);
    let bytes = match fs::read(path) {
        Err(err) => panic!("Error reading file: {:?}", err.kind()),
        Ok(s) => s,
    };

    let mut cursor = Cursor::new(bytes.as_slice());
    let mut local_file_headers: Vec<LocalFileHeader> = vec![];
    loop {
        let tag = match read_tag(&mut cursor) {
            Ok(t) => t,
            Err(t) => {
                panic!("Bad tag: {}", t);
            }
        };
        match tag {
            Magic::LocalFile => {
                let local_file_header = read_local_file(&mut cursor);
                println!("Filename: {}", local_file_header.filename);
                local_file_headers.push(local_file_header);
            }
            Magic::CentralDirectoryFile => (),
            Magic::EndOfCentralDirectory => {
                break;
            }
        };
    }
}
