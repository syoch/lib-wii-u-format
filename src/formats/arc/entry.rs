use crate::{binary_reader::Parsable, formats::common::U8String};

pub struct Entry {
    path: String,
    ptr: u32,
    size: u32,
}

impl Parsable for Entry {
    fn parse(reader: &mut crate::binary_reader::BinaryReader) -> Self {
        Self {
            path: String::from_utf8_lossy(&U8String::parse(reader)).to_string(),
            ptr: reader.read_u32(),
            size: reader.read_u32(),
        }
    }
}
