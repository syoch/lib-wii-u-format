use super::{super::binary_reader::BinaryReader, elf_identifier::ELFIdentifier};

#[derive(Debug)]
pub struct ELFHeader {
    pub e_ident: ELFIdentifier,
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub program_header_offset: u64,
    pub section_header_offset: u64,
    pub e_flags: u32,
    pub elf_header_size: u16,
    pub program_header_size: u16,
    pub program_headers_count: u16,
    pub section_header_size: u16,
    pub section_header_count: u16,
    pub str_table_index: u16,
}

impl ELFHeader {
    pub fn default() -> ELFHeader {
        ELFHeader {
            e_ident: ELFIdentifier::default(),
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            program_header_offset: 0,
            section_header_offset: 0,
            e_flags: 0,
            elf_header_size: 0,
            program_header_size: 0,
            program_headers_count: 0,
            section_header_size: 0,
            section_header_count: 0,
            str_table_index: 0,
        }
    }

    pub fn parse(reader: &mut BinaryReader) -> ELFHeader {
        let mut ret = ELFHeader::default();

        ret.e_ident = ELFIdentifier::parse(reader);

        ret.e_type = reader.read_u16();
        ret.e_machine = reader.read_u16();
        ret.e_version = reader.read_u32();
        ret.e_entry = reader.read_addr();
        ret.program_header_offset = reader.read_addr();
        ret.section_header_offset = reader.read_addr();
        ret.e_flags = reader.read_u32();
        ret.elf_header_size = reader.read_u16();
        ret.program_header_size = reader.read_u16();
        ret.program_headers_count = reader.read_u16();
        ret.section_header_size = reader.read_u16();
        ret.section_header_count = reader.read_u16();
        ret.str_table_index = reader.read_u16();
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_reader::BinaryReader;

    use super::ELFHeader;

    #[test]
    fn test_parse() {
        let data = vec![
            0x7f, 0x45, 0x4c, 0x46, 0x01, 0x02, 0x01, 0xca, 0xfe, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xfe, 0x01, 0x00, 0x14, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x00, 0x12, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        let mut reader = BinaryReader::new(data);
        let elf_header = ELFHeader::parse(&mut reader);

        assert_eq!(elf_header.e_type, 0xfe01);
        assert_eq!(elf_header.e_machine, 0x0014);
        assert_eq!(elf_header.e_version, 0x00000001);
        assert_eq!(elf_header.e_entry, 0x02000000);
        assert_eq!(elf_header.program_header_offset, 0x00000000);
        assert_eq!(elf_header.section_header_offset, 0x00000040);
        assert_eq!(elf_header.e_flags, 0x00000000);
        assert_eq!(elf_header.elf_header_size, 0x0034);
        assert_eq!(elf_header.program_header_size, 0x0000);
        assert_eq!(elf_header.program_headers_count, 0x0000);
        assert_eq!(elf_header.section_header_size, 0x0028);
        assert_eq!(elf_header.section_header_count, 0x0012);
        assert_eq!(elf_header.str_table_index, 0x000f);
    }
}
