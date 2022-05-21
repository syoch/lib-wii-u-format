use super::BinaryReader;

pub trait Parsable {
    fn parse(reader: &mut BinaryReader) -> Self;
}
