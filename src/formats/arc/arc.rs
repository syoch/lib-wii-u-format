use super::entry::Entry;
use crate::binary_reader::{BinaryReader, Parsable};

pub struct ARC {
    entries: Vec<Entry>,
}

impl Parsable for ARC {
    fn parse(reader: &mut BinaryReader) -> Self {
        let entries_count = reader.read_u32();
        let mut entries = Vec::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            entries.push(Entry::parse(reader));
        }
        Self { entries }
    }
}
