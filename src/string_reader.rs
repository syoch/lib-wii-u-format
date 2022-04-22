pub struct StringReader {
    pub data: Vec<u8>,
    pub offset: usize,
}

impl StringReader {
    pub fn new(data: Vec<u8>) -> StringReader {
        StringReader {
            data: data,
            offset: 0,
        }
    }
}

impl StringReader {
    pub fn has_data(&self) -> bool {
        self.offset < self.data.len()
    }

    pub fn read_char_abs(&mut self, pos: usize) -> Result<u8, String> {
        if pos >= self.data.len() {
            return Err("EOF".to_string());
        }
        let c = self.data[pos];
        Ok(c)
    }

    pub fn read_char(&mut self) -> Result<u8, String> {
        let c = self.read_char_abs(self.offset)?;
        self.offset += 1;
        Ok(c)
    }

    pub fn read(&mut self, pos: usize, size: usize) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        for i in 0..size {
            result.push(self.read_char_abs(pos + i)?);
        }
        Ok(result)
    }

    pub fn read_n_bytes(&mut self, size: usize) -> Result<Vec<u8>, String> {
        let mut result = self.read(self.offset, size)?;
        self.offset += size;
        Ok(result)
    }

    pub fn consume(&mut self, size: usize) {
        self.offset += size;
    }

    pub fn if_startswith(&mut self, needle: &[u8]) -> Result<bool, String> {
        for i in 0..needle.len() {
            if self.read_char_abs(self.offset + i)? != needle[i] {
                return Ok(false);
            }
        }

        self.offset += needle.len();
        Ok(true)
    }

    pub fn except<T>(&mut self, s: T) -> Result<(), String>
    where
        T: ToString,
    {
        let s = s.to_string().as_bytes().to_vec();
        if self.if_startswith(&s)? {
            Ok(())
        } else {
            Err(format!("except: {:?}", s))
        }
    }

    pub fn find<T>(&mut self, s: T) -> Result<usize, String>
    where
        T: ToString,
    {
        let str = String::from_utf8(self.data[self.offset..].to_vec()).unwrap();
        let pos = str.find(&s.to_string()).unwrap();

        Ok(pos)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read() {
        let data = "abcd1234".as_bytes().to_vec();
        let mut reader = super::StringReader::new(data);

        assert_eq!(reader.read(0, 1).unwrap(), b"a");
        assert_eq!(reader.read(1, 3).unwrap(), b"bcd");
        assert_eq!(reader.read(4, 4).unwrap(), b"1234");
    }

    #[test]
    fn test_read_n_bytes() {
        let data = "abcd1234".as_bytes().to_vec();
        let mut reader = super::StringReader::new(data);

        assert_eq!(reader.read_n_bytes(1).unwrap(), b"a");
        assert_eq!(reader.read_n_bytes(3).unwrap(), b"bcd");
        assert_eq!(reader.read_n_bytes(4).unwrap(), b"1234");
    }

    #[test]
    fn test_consume() {
        let data = "abcd1234".as_bytes().to_vec();
        let mut reader = super::StringReader::new(data);

        reader.consume(1);
        assert_eq!(reader.read_n_bytes(3).unwrap(), b"bcd");
        reader.consume(2);
        assert_eq!(reader.read_n_bytes(2).unwrap(), b"34");
    }

    #[test]
    fn test_if_startswith() {
        let data = "abcd1234".as_bytes().to_vec();
        let mut reader = super::StringReader::new(data);

        assert!(reader.if_startswith(b"abcd").unwrap());
        assert!(!reader.if_startswith(b"1235").unwrap());
    }

    #[test]
    fn test_except() {
        let data = "abcd1234".as_bytes().to_vec();
        let mut reader = super::StringReader::new(data);

        assert!(reader.except("abcd").is_ok());
        assert!(reader.except("1235").is_err());
        assert_eq!(reader.read_n_bytes(4).unwrap(), b"1234");
    }

    #[test]
    fn test_find() {
        let data = "abcd1234".as_bytes().to_vec();
        let mut reader = super::StringReader::new(data);

        assert_eq!(reader.find("cd").unwrap(), 2);
        assert_eq!(reader.find("1234").unwrap(), 4);
    }

    #[test]
    fn test_has_data() {
        let data = "abcd1234".as_bytes().to_vec();
        let mut reader = super::StringReader::new(data);

        assert!(reader.has_data());
        reader.consume(4);
        assert!(reader.has_data());
        reader.consume(4);
        assert!(!reader.has_data());
    }
}
