use super::super::binary_reader::{BinaryReader, Endian};

pub struct ELFIdentifier {
    os_abi: u8,
    abi_version: u8,
}

impl ELFIdentifier {
    pub fn new(os_abi: u8, abi_version: u8) -> ELFIdentifier {
        ELFIdentifier {
            os_abi,
            abi_version,
        }
    }

    pub fn default() -> ELFIdentifier {
        ELFIdentifier::new(0, 0)
    }

    pub fn parse(reader: &mut BinaryReader) -> ELFIdentifier {
        let signature = reader.read_n_bytes(4);

        if signature != b"\x7fELF" {
            eprintln!("Warning: Invalid ELF signature");
        }

        let class = reader.read_u8();

        let data = reader.read_u8();

        if reader.read_u8() != 1 {
            eprintln!("Warning: Invalid ELF Version");
        }

        let os_abi = reader.read_u8();

        let abi_version = reader.read_u8();

        reader.offset += 16 - 9;

        let ret = ELFIdentifier::new(os_abi, abi_version);

        match class {
            1 => {
                reader.is_64bit = false;
            }
            2 => {
                reader.is_64bit = true;
            }
            _ => {
                eprintln!("Warning: Invalid ELF Class")
            }
        }

        match data {
            1 => {
                reader.endian = Endian::Little;
            }
            2 => {
                reader.endian = Endian::Big;
            }
            _ => {
                eprintln!("Warning: Invalid ELF Data Encoding")
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_reader::{BinaryReader, Endian};

    fn auto_test(is_64bit: bool, is_big_endian: bool) {
        let mut reader = BinaryReader::new(vec![
            0x7f,
            0x45,
            0x4c,
            0x46,
            if is_64bit { 2 } else { 1 },
            if is_big_endian { 2 } else { 1 },
            0x01,
            0xca,
            0xfe,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ]);

        let id = super::ELFIdentifier::parse(&mut reader);

        assert_eq!(id.os_abi, 0xca);
        assert_eq!(id.abi_version, 0xfe);

        assert_eq!(reader.is_64bit, is_64bit);
        assert_eq!(
            reader.endian,
            if is_big_endian {
                Endian::Big
            } else {
                Endian::Little
            }
        );
    }

    #[test]
    fn test_32bit_little() {
        auto_test(false, false);
    }
    #[test]
    fn test_32bit_big() {
        auto_test(false, true);
    }

    #[test]
    fn test_64bit_little() {
        auto_test(true, false);
    }
    #[test]
    fn test_64bit_big() {
        auto_test(true, true);
    }
}
