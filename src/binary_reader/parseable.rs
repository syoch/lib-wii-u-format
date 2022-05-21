use super::BinaryReader;

pub trait Parsable {
    fn parse(&mut self, reader: &mut BinaryReader) -> Self;
}
