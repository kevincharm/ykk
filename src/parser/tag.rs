use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

pub enum Magic {
    LocalFile,
    CentralDirectoryFile,
    EndOfCentralDirectory,
}

pub fn read_tag(contents: &mut Cursor<&[u8]>) -> Result<Magic, u32> {
    let tag = match contents.read_u32::<LittleEndian>() {
        Ok(t) => t,
        Err(_) => panic!("Failed to read next tag!"),
    };
    match tag {
        0x04034b50 => Ok(Magic::LocalFile),
        0x02014b50 => Ok(Magic::CentralDirectoryFile),
        0x06054b50 => Ok(Magic::EndOfCentralDirectory),
        _ => Err(tag),
    }
}
