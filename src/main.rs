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

    let input_file_path = Path::new(&args[1]);
    let cwd = match input_file_path.parent() {
        Some(p) => p,
        None => Path::new("/"),
    };
    let bytes = match fs::read(input_file_path) {
        Ok(s) => s,
        Err(err) => panic!("Error reading file: {:?}", err.kind()),
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

    let mut headers = local_file_headers.into_iter();
    for header in central_dir_file_headers.into_iter() {
        println!("Extracting {}...", header.filename);
        let header = match headers.find(|s| s.filename == header.filename) {
            Some(h) => h,
            None => continue,
        };
        let mut decoder = DeflateDecoder::new(header.data.as_slice());
        let mut buffer: Vec<u8> = vec![];
        match decoder.read_to_end(&mut buffer) {
            Ok(content) => content,
            Err(_) => {
                println!("Error deflating {}!", header.filename);
                continue;
            }
        };
        let header_path = Path::new(&header.filename);
        let full_path = cwd.join(header_path);
        match fs::write(full_path, buffer) {
            Ok(_) => (),
            Err(err) => {
                println!("Error deflating {}: {:?}", header.filename, err.kind());
            }
        }
    }
}
