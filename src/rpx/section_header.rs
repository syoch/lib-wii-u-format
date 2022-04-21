use super::super::binary_reader::BinaryReader;
use flate2::read::ZlibDecoder;
use std::io::{Read, Seek, SeekFrom};

pub struct SectionHeader {
    sh_name: u64,
    sh_type: u64,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u64,
    sh_info: u64,
    sh_addr_align: u64,
    sh_ent_size: u64,

    remained_flags: u64,
    data: Vec<u8>,
}

impl SectionHeader {
    pub fn new(
        sh_name: u64,
        sh_type: u64,
        sh_flags: u64,
        sh_addr: u64,
        sh_offset: u64,
        sh_size: u64,
        sh_link: u64,
        sh_info: u64,
        sh_addr_align: u64,
        sh_ent_size: u64,

        reader: Option<&mut BinaryReader>,
    ) -> SectionHeader {
        let mut ret = SectionHeader {
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addr_align,
            sh_ent_size,

            remained_flags: sh_flags,
            data: vec![],
        };

        if let Some(reader) = reader {
            ret.data = reader.read(ret.sh_offset as usize, ret.sh_size as usize);
        }

        if (ret.sh_flags & 1 << 27) != 0 {
            // zlib deflate
            println!("{:?}", ret.data);
            let mut decoder = ZlibDecoder::new(&ret.data[4..]);
            let mut buf = vec![];
            decoder.read_to_end(&mut buf).unwrap();
            ret.data = buf;
        }

        return ret;
    }

    pub fn default() -> SectionHeader {
        SectionHeader::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, None)
    }

    pub fn parse(reader: &mut BinaryReader) -> SectionHeader {
        SectionHeader::new(
            reader.read_u32() as u64,
            reader.read_u32() as u64,
            reader.read_u32() as u64,
            reader.read_u32() as u64,
            reader.read_u32() as u64,
            reader.read_u32() as u64,
            reader.read_u32() as u64,
            reader.read_u32() as u64,
            reader.read_u32() as u64,
            reader.read_u32() as u64,
            Some(reader),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_reader::BinaryReader;

    #[test]
    fn test_parse() {
        let mut data = vec![
            0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x33, 0x33,
            0x33, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x66, 0x66, 0x66, 0x66,
            0x77, 0x77, 0x77, 0x77, 0x88, 0x88, 0x88, 0x88, 0x99, 0x99, 0x99, 0x99,
        ];

        let mut reader = BinaryReader::new(data.clone());

        let header = super::SectionHeader::parse(&mut reader);

        assert_eq!(header.sh_name, 0x00000000, "[sh_name]");
        assert_eq!(header.sh_type, 0x11111111, "[sh_type]");
        assert_eq!(header.sh_flags, 0x22222222, "[sh_flags]");
        assert_eq!(header.sh_addr, 0x33333333, "[sh_addr]");
        assert_eq!(header.sh_offset, 0, "[sh_offset]");
        assert_eq!(header.sh_size, 40, "[sh_size]");
        assert_eq!(header.sh_link, 0x66666666, "[sh_link]");
        assert_eq!(header.sh_info, 0x77777777, "[sh_info]");
        assert_eq!(header.sh_addr_align, 0x88888888, "[sh_addr_align]");
        assert_eq!(header.sh_ent_size, 0x99999999, "[sh_ent_size]");

        assert_eq!(header.data, data.clone(), "[data]");
    }

    #[test]
    fn test_zlib() {
        let mut data = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x9c,
            0xcb, 0xc8, 0x04, 0x00, 0x01, 0x3b, 0x00, 0xd2,
        ];

        let mut reader = BinaryReader::new(data.clone());

        let header = super::SectionHeader::parse(&mut reader);
        assert_eq!(header.data, vec![0x68, 0x69], "[data]");
    }
}
