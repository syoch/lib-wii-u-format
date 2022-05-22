use super::TagPayload;
use crate::binary_reader::{BinaryReader, Parsable};

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub value: TagPayload,
}

impl Parsable for Tag {
    fn parse(reader: &mut BinaryReader) -> Self {
        let name = reader.read_u16_string();
        let tag_type = reader.read_u8();
        let value = TagPayload::parse_payload(reader, tag_type);
        Tag { name, value }
    }
}
