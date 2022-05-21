use super::elf_header::ELFHeader;
use super::program_header::ProgramHeader;
use super::section_header::{SectionHeader, SectionName};
use crate::binary_reader::BinaryReader;
use crate::utils::find_zero;

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
        let mut ret = Rpx::default();
        ret.reader = reader;

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

        for i in 0..self.section_headers.len() {
            if let SectionName::Offset(offset) = self.section_headers[i].name {
                self.section_headers[i].name =
                    SectionName::String(self.read_str_from_strtab(offset));
            }
        }
    }
}

impl Rpx {
    pub fn read_str_from_strtab(&mut self, offset: usize) -> String {
        let data = &self.section_headers[self.elf_header.str_table_index as usize].data;
        let end = find_zero(data.to_vec(), offset);
        if offset == end {
            return String::new();
        }
        String::from_utf8(data[offset..end].to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    
}
