use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::str;

#[allow(dead_code)]
pub struct LocalFileHeader {
    pub version: u16,
    pub flags: u16,
    pub compression: u16,
    pub modified_time: u16,
    pub modified_date: u16,
    pub crc32: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub filename_length: u16,
    pub extra_field_length: u16,
    pub filename: String,
    pub extra_field: Vec<u8>,
    pub data: Vec<u8>,
    pub data_descriptor: Vec<u8>,
}

pub fn parse_local_file_header(contents: &mut Cursor<&[u8]>) -> LocalFileHeader {
    let version = contents.read_u16::<LittleEndian>().unwrap();
    let flags = contents.read_u16::<LittleEndian>().unwrap();
    let compression = contents.read_u16::<LittleEndian>().unwrap();
    let modified_time = contents.read_u16::<LittleEndian>().unwrap();
    let modified_date = contents.read_u16::<LittleEndian>().unwrap();
    let crc32 = contents.read_u32::<LittleEndian>().unwrap();
    let compressed_size = contents.read_u32::<LittleEndian>().unwrap();
    let uncompressed_size = contents.read_u32::<LittleEndian>().unwrap();
    let filename_length = contents.read_u16::<LittleEndian>().unwrap();
    let extra_field_length = contents.read_u16::<LittleEndian>().unwrap();

    let mut raw_filename: Vec<u8> = vec![];
    for _ in 0..filename_length {
        raw_filename.push(contents.read_u8().unwrap());
    }
    let filename = String::from(str::from_utf8(raw_filename.as_slice()).unwrap());

    let mut extra_field = vec![];
    for _ in 0..extra_field_length {
        extra_field.push(contents.read_u8().unwrap());
    }

    let mut data = vec![];
    for _ in 0..compressed_size {
        data.push(contents.read_u8().unwrap());
    }

    let mut data_descriptor_len = 0;
    if (flags >> 2) & 0x01 == 1 {
        data_descriptor_len = 3;
    }
    let mut data_descriptor = vec![];
    for _ in 0..data_descriptor_len {
        data_descriptor.push(contents.read_u8().unwrap());
    }

    let header = LocalFileHeader {
        version,
        flags,
        compression,
        modified_time,
        modified_date,
        crc32,
        compressed_size,
        uncompressed_size,
        filename_length,
        extra_field_length,
        filename,
        extra_field,
        data,
        data_descriptor,
    };

    header
}
