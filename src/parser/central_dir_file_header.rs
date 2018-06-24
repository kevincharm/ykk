use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::str;

#[allow(dead_code)]
pub struct CentralDirectoryFileHeader {
    pub version: u16,
    pub version_needed: u16,
    pub flags: u16,
    pub compression: u16,
    pub modified_time: u16,
    pub modified_date: u16,
    pub crc32: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub filename_length: u16,
    pub extra_field_length: u16,
    pub file_comment_length: u16,
    pub disk_num_start: u16,
    pub internal_attr: u16,
    pub external_attr: u32,
    pub local_header_offset: u32,
    pub filename: String,
    pub extra_field: Vec<u8>,
    pub file_comment: String,
}

pub fn parse_central_dir_file_header(cursor: &mut Cursor<&[u8]>) -> CentralDirectoryFileHeader {
    let version = cursor.read_u16::<LittleEndian>().unwrap();
    let version_needed = cursor.read_u16::<LittleEndian>().unwrap();
    let flags = cursor.read_u16::<LittleEndian>().unwrap();
    let compression = cursor.read_u16::<LittleEndian>().unwrap();
    let modified_time = cursor.read_u16::<LittleEndian>().unwrap();
    let modified_date = cursor.read_u16::<LittleEndian>().unwrap();
    let crc32 = cursor.read_u32::<LittleEndian>().unwrap();
    let compressed_size = cursor.read_u32::<LittleEndian>().unwrap();
    let uncompressed_size = cursor.read_u32::<LittleEndian>().unwrap();
    let filename_length = cursor.read_u16::<LittleEndian>().unwrap();
    let extra_field_length = cursor.read_u16::<LittleEndian>().unwrap();
    let file_comment_length = cursor.read_u16::<LittleEndian>().unwrap();
    let disk_num_start = cursor.read_u16::<LittleEndian>().unwrap();
    let internal_attr = cursor.read_u16::<LittleEndian>().unwrap();
    let external_attr = cursor.read_u32::<LittleEndian>().unwrap();
    let local_header_offset = cursor.read_u32::<LittleEndian>().unwrap();

    let mut raw_filename: Vec<u8> = vec![];
    for _ in 0..filename_length {
        raw_filename.push(cursor.read_u8().unwrap());
    }
    let filename = String::from(str::from_utf8(raw_filename.as_slice()).unwrap());

    let mut extra_field = vec![];
    for _ in 0..extra_field_length {
        extra_field.push(cursor.read_u8().unwrap());
    }

    let mut raw_file_comment: Vec<u8> = vec![];
    for _ in 0..file_comment_length {
        raw_file_comment.push(cursor.read_u8().unwrap());
    }
    let file_comment = String::from(str::from_utf8(raw_file_comment.as_slice()).unwrap());

    let header = CentralDirectoryFileHeader {
        version,
        version_needed,
        flags,
        compression,
        modified_time,
        modified_date,
        crc32,
        compressed_size,
        uncompressed_size,
        filename_length,
        extra_field_length,
        file_comment_length,
        disk_num_start,
        internal_attr,
        external_attr,
        local_header_offset,
        filename,
        extra_field,
        file_comment,
    };

    header
}
