#![allow(overflowing_literals)]
use super::Tag;
use crate::binary_reader::{BinaryReader, Parsable};

#[derive(Debug, PartialEq)]
pub enum TagPayload {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<TagPayload>),
    Compound(Vec<Tag>),
    IntArray(Vec<i32>),
}

impl Default for TagPayload {
    fn default() -> Self {
        TagPayload::End
    }
}

impl TagPayload {
    pub fn parse_payload(reader: &mut BinaryReader, tag_type: u8) -> Self {
        match tag_type {
            0 => TagPayload::End,
            1 => TagPayload::Byte(reader.read_u8() as i8),
            2 => TagPayload::Short(reader.read_u16() as i16),
            3 => TagPayload::Int(reader.read_u32() as i32),
            4 => TagPayload::Long(reader.read_u64() as i64),
            5 => TagPayload::Float(f32::from_bits(reader.read_u32())),
            6 => TagPayload::Double(f64::from_bits(reader.read_u64())),
            7 => {
                let size = reader.read_u32() as usize;
                TagPayload::ByteArray(reader.read_n_bytes(size))
            }
            8 => {
                let size = reader.read_u16();
                let bytes = reader.read_n_bytes(size as usize);
                let string = String::from_utf8_lossy(&bytes).to_string();
                TagPayload::String(string)
            }
            9 => {
                let sub_type = reader.read_u8();
                let size = reader.read_u32() as usize;
                let mut list = Vec::new();
                for _ in 0..size {
                    list.push(TagPayload::parse_payload(reader, sub_type));
                }
                TagPayload::List(list)
            }
            10 => {
                let mut elements = Vec::new();
                while reader.peek_u8() != 0 {
                    elements.push(Tag::parse(reader));
                }
                reader.read_u8();
                TagPayload::Compound(elements)
            }
            11 => {
                let size = reader.read_u32() as usize;
                let mut array = Vec::new();
                for _ in 0..size {
                    array.push(reader.read_u32() as i32);
                }
                TagPayload::IntArray(array)
            }
            _ => panic!("Unknown tag type: {}", tag_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TagPayload;
    use crate::binary_reader::BinaryReader;
    #[test]
    fn test_end() {
        let data = vec![];
        let mut reader = BinaryReader::new(data);

        assert_eq!(TagPayload::parse_payload(&mut reader, 0), TagPayload::End);
    }

    #[test]
    fn test_byte() {
        let data = vec![0x12];
        let mut reader = BinaryReader::new(data);

        assert_eq!(
            TagPayload::parse_payload(&mut reader, 1),
            TagPayload::Byte(0x12)
        );
    }

    #[test]
    fn test_short() {
        let data = vec![0x12, 0x34];
        let mut reader = BinaryReader::new(data);

        assert_eq!(
            TagPayload::parse_payload(&mut reader, 2),
            TagPayload::Short(0x1234)
        );
    }

    #[test]
    fn test_int() {
        let data = vec![0x12, 0x34, 0x56, 0x78];
        let mut reader = BinaryReader::new(data);

        assert_eq!(
            TagPayload::parse_payload(&mut reader, 3),
            TagPayload::Int(0x12345678)
        );
    }

    #[test]
    fn test_long() {
        let data = vec![0x12, 0x34, 0x56, 0x78, 0xaa, 0xbb, 0xcc, 0xdd];
        let mut reader = BinaryReader::new(data);

        assert_eq!(
            TagPayload::parse_payload(&mut reader, 4),
            TagPayload::Long(0x12345678aabbccdd)
        );
    }

    #[test]
    fn test_float() {
        let data = vec![0x3f, 0x80, 0x00, 0x00];
        let mut reader = BinaryReader::new(data);

        assert_eq!(
            TagPayload::parse_payload(&mut reader, 5),
            TagPayload::Float(1.0_f32)
        );
    }

    #[test]
    fn test_double() {
        let data = vec![0x3f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut reader = BinaryReader::new(data);

        assert_eq!(
            TagPayload::parse_payload(&mut reader, 6),
            TagPayload::Double(1.0_f64)
        );
    }

    #[test]
    fn test_bytearray() {
        let data = vec![0x00, 0x00, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04];
        let mut reader = BinaryReader::new(data);

        assert_eq!(
            TagPayload::parse_payload(&mut reader, 7),
            TagPayload::ByteArray(vec![0x01, 0x02, 0x03, 0x04])
        );
    }

    #[test]
    fn test_string() {
        let data = vec![0x00, 0x02, 0x61, 0x62];
        let mut reader = BinaryReader::new(data);

        assert_eq!(
            TagPayload::parse_payload(&mut reader, 8),
            TagPayload::String("ab".to_string())
        );
    }
}
