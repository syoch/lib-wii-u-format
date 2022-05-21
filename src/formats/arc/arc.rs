use crate::binary_reader::BinaryReader;

pub struct Entry {}
pub struct ARC {
    // entries_count: u32,
    entries: Vec<Entry>,
}
