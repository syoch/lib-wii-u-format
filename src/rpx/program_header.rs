use super::super::binary_reader::BinaryReader;

pub struct ProgramHeader {
    pub ph_type: u64,
    pub ph_offset: u64,
    pub ph_virtual_address: u64,
    pub ph_physical_addr: u64,
    pub ph_file_size: u64,
    pub ph_mem_size: u64,
    pub ph_flags: u64,
    pub ph_align: u64,
}

impl ProgramHeader {
    pub fn new() -> ProgramHeader {
        ProgramHeader {
            ph_type: 0,
            ph_offset: 0,
            ph_virtual_address: 0,
            ph_physical_addr: 0,
            ph_file_size: 0,
            ph_mem_size: 0,
            ph_flags: 0,
            ph_align: 0,
        }
    }

    pub fn parse(reader: &mut BinaryReader) -> ProgramHeader {
        ProgramHeader {
            ph_type: reader.read_word(),
            ph_offset: reader.read_word(),
            ph_virtual_address: reader.read_word(),
            ph_physical_addr: reader.read_word(),
            ph_file_size: reader.read_word(),
            ph_mem_size: reader.read_word(),
            ph_flags: reader.read_word(),
            ph_align: reader.read_word(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_reader::BinaryReader;

    #[test]
    fn test_parse() {
        let mut reader = BinaryReader::new(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ]);

        let header = super::ProgramHeader::parse(&mut reader);

        assert_eq!(header.ph_type, 0x00010203);
        assert_eq!(header.ph_offset, 0x04050607);
        assert_eq!(header.ph_virtual_address, 0x08090a0b);
        assert_eq!(header.ph_physical_addr, 0x0c0d0e0f);
        assert_eq!(header.ph_file_size, 0x10111213);
        assert_eq!(header.ph_mem_size, 0x14151617);
        assert_eq!(header.ph_flags, 0x18191a1b);
        assert_eq!(header.ph_align, 0x1c1d1e1f);
    }
}
