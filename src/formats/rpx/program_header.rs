use crate::binary_reader::BinaryReader;

pub struct ProgramHeader {
    pub ph_type: u64,
    pub offset: u64,
    pub virtual_address: u64,
    pub physical_address: u64,
    pub file_size: u64,
    pub mem_size: u64,
    pub ph_flags: u64,
    pub ph_align: u64,
}

impl std::fmt::Display for ProgramHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ProgramHeader< {:#10x}>[{:#010x}] @ {:#010x}({:#010x}) -> {:#010x}{{{}}}({:#010x}) [{}]",
            self.ph_flags,
            self.ph_type,
            self.offset,
            self.file_size,
            self.virtual_address,
            self.physical_address,
            self.mem_size,
            self.ph_align,
        )
    }
}

impl ProgramHeader {
    pub fn new() -> ProgramHeader {
        ProgramHeader {
            ph_type: 0,
            offset: 0,
            virtual_address: 0,
            physical_address: 0,
            file_size: 0,
            mem_size: 0,
            ph_flags: 0,
            ph_align: 0,
        }
    }

    pub fn parse(reader: &mut BinaryReader) -> ProgramHeader {
        if reader.is_64bit {
            ProgramHeader {
                ph_type: reader.read_word(),
                ph_flags: reader.read_word(),
                offset: reader.read_word(),
                virtual_address: reader.read_word(),
                physical_address: reader.read_word(),
                file_size: reader.read_word(),
                mem_size: reader.read_word(),
                ph_align: reader.read_word(),
            }
        } else {
            ProgramHeader {
                ph_type: reader.read_word(),
                offset: reader.read_word(),
                virtual_address: reader.read_word(),
                physical_address: reader.read_word(),
                file_size: reader.read_word(),
                mem_size: reader.read_word(),
                ph_flags: reader.read_word(),
                ph_align: reader.read_word(),
            }
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
        assert_eq!(header.offset, 0x04050607);
        assert_eq!(header.virtual_address, 0x08090a0b);
        assert_eq!(header.physical_address, 0x0c0d0e0f);
        assert_eq!(header.file_size, 0x10111213);
        assert_eq!(header.mem_size, 0x14151617);
        assert_eq!(header.ph_flags, 0x18191a1b);
        assert_eq!(header.ph_align, 0x1c1d1e1f);
    }
}
