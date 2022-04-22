use crate::string_reader;

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolName {
    base_name: String,
}

impl SymbolName {
    pub fn new(base_name: String) -> Self {
        let mut ret = Self::default();
        ret.base_name = base_name;
        ret
    }

    pub fn default() -> Self {
        SymbolName {
            base_name: String::new(),
        }
    }
}

pub fn demangle(input: &str) -> Result<SymbolName, Box<dyn std::error::Error>> {
    let mut ret = SymbolName::default();
    let mut reader = string_reader::StringReader::new(input.as_bytes().to_vec());

    let end_of_base_name = reader.find("__");

    if let Err(ref e) = end_of_base_name {
        if !e.starts_with("find: ") {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}", e),
            )));
        }

        ret.base_name = input.to_string();
        return Ok(ret);
    };

    let end_of_base_name = end_of_base_name.unwrap();

    let base_name = reader.read_n_bytes(end_of_base_name)?;
    ret.base_name = String::from_utf8(base_name)?;

    reader.consume(2);

    println!("{:?}", ret);
    println!(
        "Remained: {:?}",
        String::from_utf8(reader.data.as_slice()[reader.offset..].to_vec())?
    );

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "o".to_string(),
    )))
}
