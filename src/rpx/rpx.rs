use super::super::binary_reader::BinaryReader;
use super::elf_header::ELFHeader;
use super::program_header::ProgramHeader;
use super::section_header::SectionHeader;

pub struct Rpx {
    pub elf_header: ELFHeader,
    pub program_headers: Vec<ProgramHeader>,
    pub section_headers: Vec<SectionHeader>,
    pub reader: BinaryReader,
}

impl Default for Rpx {
    fn default() -> Self {
        Self {
            elf_header: ELFHeader::default(),
            program_headers: Vec::new(),
            section_headers: Vec::new(),
            reader: BinaryReader::default(),
        }
    }
}

impl Rpx {
    pub fn parse(reader: BinaryReader) -> Rpx {
        let mut ret = Rpx {
            elf_header: ELFHeader::default(),
            program_headers: Vec::new(),
            section_headers: Vec::new(),
            reader: reader,
        };

        ret.init();

        ret
    }

    pub fn init(&mut self) -> () {
        self.elf_header = ELFHeader::parse(&mut self.reader);

        self.reader
            .seek(self.elf_header.program_header_offset as usize);
        for _ in 0..self.elf_header.program_headers_count {
            self.program_headers
                .push(ProgramHeader::parse(&mut self.reader));
        }

        self.reader
            .seek(self.elf_header.section_header_offset as usize);
        for _ in 0..self.elf_header.section_header_count {
            self.section_headers
                .push(SectionHeader::parse(&mut self.reader));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_reader::BinaryReader;
}
