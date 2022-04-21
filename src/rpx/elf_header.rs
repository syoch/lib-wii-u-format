use super::{super::binary_reader::BinaryReader, elf_identifier::ELFIdentifier};

pub struct ELFHeader {
    e_ident: ELFIdentifier,
    e_type: u32,
    e_machine: u32,
    e_version: u64,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u64,
    e_ehsize: u32,
    e_phentsize: u32,
    e_phnum: u32,
    e_shentsize: u32,
    e_shnum: u32,
    e_shstrndx: u32,
}

impl ELFHeader {
    pub fn default() -> ELFHeader {
        ELFHeader {
            e_ident: ELFIdentifier::default(),
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }

    pub fn parse(reader: &mut BinaryReader) -> ELFHeader {
        let mut ret = ELFHeader::default();

        ret.e_ident = ELFIdentifier::parse(reader);

        ret.e_type = reader.read_half();
        ret.e_machine = reader.read_half();
        ret.e_version = reader.read_word();
        ret.e_entry = reader.read_word();
        ret.e_phoff = reader.read_word();
        ret.e_shoff = reader.read_word();
        ret.e_flags = reader.read_word();
        ret.e_ehsize = reader.read_half();
        ret.e_phentsize = reader.read_half();
        ret.e_phnum = reader.read_half();
        ret.e_shentsize = reader.read_half();
        ret.e_shnum = reader.read_half();
        ret.e_shstrndx = reader.read_half();

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
        assert_eq!(elf_header.e_phoff, 0x00000000);
        assert_eq!(elf_header.e_shoff, 0x00000040);
        assert_eq!(elf_header.e_flags, 0x00000000);
        assert_eq!(elf_header.e_ehsize, 0x0034);
        assert_eq!(elf_header.e_phentsize, 0x0000);
        assert_eq!(elf_header.e_phnum, 0x0000);
        assert_eq!(elf_header.e_shentsize, 0x0028);
        assert_eq!(elf_header.e_shnum, 0x0012);
        assert_eq!(elf_header.e_shstrndx, 0x000f);
    }
}
