use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::str;

#[allow(dead_code)]
pub struct EndOfCentralDirectoryHeader {
    pub disk_num: u16,
    pub disk_num_with_cd: u16,
    pub disk_entries: u16,
    pub total_entries: u16,
    pub central_directory_size: u32,
    pub offset_of_cd_wrt_starting_disk: u32,
    pub comment_length: u16,
    pub comment: String,
}

pub fn parse_end_of_central_dir_header(cursor: &mut Cursor<&[u8]>) -> EndOfCentralDirectoryHeader {
    let disk_num = cursor.read_u16::<LittleEndian>().unwrap();
    let disk_num_with_cd = cursor.read_u16::<LittleEndian>().unwrap();
    let disk_entries = cursor.read_u16::<LittleEndian>().unwrap();
    let total_entries = cursor.read_u16::<LittleEndian>().unwrap();
    let central_directory_size = cursor.read_u32::<LittleEndian>().unwrap();
    let offset_of_cd_wrt_starting_disk = cursor.read_u32::<LittleEndian>().unwrap();
    let comment_length = cursor.read_u16::<LittleEndian>().unwrap();

    let mut raw_comment: Vec<u8> = vec![];
    for _ in 0..comment_length {
        raw_comment.push(cursor.read_u8().unwrap());
    }
    let comment = String::from(str::from_utf8(raw_comment.as_slice()).unwrap());

    let header = EndOfCentralDirectoryHeader {
        disk_num,
        disk_num_with_cd,
        disk_entries,
        total_entries,
        central_directory_size,
        offset_of_cd_wrt_starting_disk,
        comment_length,
        comment,
    };

    header
}
