extern crate byteorder;
extern crate flate2;

mod parser;

use flate2::read::DeflateDecoder;
use parser::central_dir_file_header::*;
use parser::end_of_central_dir_header::*;
use parser::local_file_header::*;
use parser::tag::*;
use std::env;
use std::fs;
use std::io::prelude::*;
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
    let mut central_dir_file_headers: Vec<CentralDirectoryFileHeader> = vec![];
    #[allow(unused_assignments)]
    let mut end_of_central_dir_header: Option<EndOfCentralDirectoryHeader> = None;
    loop {
        let tag = match parse_tag(&mut cursor) {
            Ok(t) => t,
            Err(t) => {
                panic!("Bad tag: {}", t);
            }
        };
        match tag {
            Magic::LocalFile => {
                let local_file_header = parse_local_file_header(&mut cursor);
                local_file_headers.push(local_file_header);
            }
            Magic::CentralDirectoryFile => {
                let central_dir_file_header = parse_central_dir_file_header(&mut cursor);
                central_dir_file_headers.push(central_dir_file_header);
            }
            Magic::EndOfCentralDirectory => {
                end_of_central_dir_header = Some(parse_end_of_central_dir_header(&mut cursor));
                break;
            }
        };
    }

    match end_of_central_dir_header {
        Some(_) => (),
        None => panic!("Invalid zip format: end of central directory not found!"),
    }

    for header in local_file_headers.into_iter() {
        println!("[lf] Filename: {}", header.filename);
        let mut d = DeflateDecoder::new(header.data.as_slice());
        let mut s = String::new();
        let text_content = match d.read_to_string(&mut s) {
            Ok(content) => content,
            Err(_) => {
                println!("Data is not valid UTF-8; aborting text decode.");
                continue;
            }
        };
        println!("Text: {}", text_content);
    }

    for header in central_dir_file_headers.into_iter() {
        println!("[cd] Filename: {}", header.filename);
    }
}
