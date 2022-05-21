use crate::binary_reader::{BinaryReader, Parsable};

pub type U8String = Vec<u8>;
pub type U16String = Vec<u16>;

impl Parsable for U8String {
    fn parse(reader: &mut BinaryReader) -> Self {
        let length = reader.read_u16();
        reader.read_n_bytes(length as usize)
    }
}

impl Parsable for U16String {
    fn parse(reader: &mut BinaryReader) -> Self {
        let length = reader.read_u16();
        let mut ret = vec![];
        for _ in 0..length {
            ret.push(reader.read_u16());
        }
        ret
    }
}
impl Parsable for u8 {
    fn parse(reader: &mut BinaryReader) -> Self {
        reader.read_u8()
    }
}
impl Parsable for u16 {
    fn parse(reader: &mut BinaryReader) -> Self {
        reader.read_u16()
    }
}

impl Parsable for u32 {
    fn parse(reader: &mut BinaryReader) -> Self {
        reader.read_u32()
    }
}

impl Parsable for u64 {
    fn parse(reader: &mut BinaryReader) -> Self {
        reader.read_u64()
    }
}

impl Parsable for usize {
    fn parse(reader: &mut BinaryReader) -> Self {
        reader.read_size() as usize
    }
}
