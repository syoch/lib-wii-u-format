#[derive(PartialEq, Debug, Clone)]
pub enum Endian {
    Big,
    Little,
}

impl std::fmt::Display for Endian {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Endian::Big => write!(f, "Big"),
            Endian::Little => write!(f, "Little"),
        }
    }
}
