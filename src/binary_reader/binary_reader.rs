use super::super::utils::concat_number;
use super::Endian;

#[derive(PartialEq, Debug, Clone)]
pub struct BinaryReader {
    pub data: Vec<u8>,
    pub offset: usize,

    pub endian: Endian,
    pub is_64bit: bool,
}

impl Default for BinaryReader {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            offset: 0,
            endian: Endian::Little,
            is_64bit: false,
        }
    }
}

impl BinaryReader {
    pub fn new(data: Vec<u8>) -> BinaryReader {
        BinaryReader {
            data,
            offset: 0,
            endian: Endian::Big,
            is_64bit: false,
        }
    }

    pub fn _concat<T>(&self, high: T, low: T, shift: u8) -> u128
    where
        T: num::ToPrimitive,
        T: std::ops::Shr<usize, Output = T>,
        T: num::Unsigned,
    {
        if self.endian == Endian::Big {
            concat_number(high, low, shift)
        } else {
            concat_number(low, high, shift)
        }
    }

    pub fn seek(&mut self, offset: usize) {
        self.offset = offset;
    }
}

impl BinaryReader {
    pub fn peek_u8(&self) -> u8 {
        self.data[self.offset]
    }

    pub fn read_u8(&mut self) -> u8 {
        let result = self.data[self.offset];
        self.offset += 1;
        result
    }

    pub fn read_u16(&mut self) -> u16 {
        let high = self.read_u8() as u16;
        let low = self.read_u8() as u16;
        self._concat(high, low, 8) as u16
    }

    pub fn read_u32(&mut self) -> u32 {
        let high = self.read_u16() as u32;
        let low = self.read_u16() as u32;
        self._concat(high, low, 16) as u32
    }

    pub fn read_u64(&mut self) -> u64 {
        let high = self.read_u32() as u64;
        let low = self.read_u32() as u64;
        self._concat(high, low, 32) as u64
    }

    pub fn read_u128(&mut self) -> u128 {
        let high = self.read_u64() as u128;
        let low = self.read_u64() as u128;
        self._concat(high, low, 64) as u128
    }
}

impl BinaryReader {
    pub fn read_half(&mut self) -> u32 {
        if self.is_64bit {
            self.read_u32()
        } else {
            self.read_u16() as u32
        }
    }

    pub fn read_word(&mut self) -> u64 {
        if self.is_64bit {
            self.read_u64()
        } else {
            self.read_u32() as u64
        }
    }

    pub fn read_dword(&mut self) -> u128 {
        if self.is_64bit {
            self.read_u128()
        } else {
            self.read_u64() as u128
        }
    }

    pub fn read_addr(&mut self) -> u64 {
        if self.is_64bit {
            self.read_u64()
        } else {
            self.read_u32() as u64
        }
    }

    pub fn read_size(&mut self) -> usize {
        if self.is_64bit {
            self.read_u64() as usize
        } else {
            self.read_u32() as usize
        }
    }
}

impl BinaryReader {
    pub fn read(&mut self, pos: usize, size: usize) -> Vec<u8> {
        self.data[pos..pos + size].to_vec()
    }

    pub fn read_n_bytes(&mut self, size: usize) -> Vec<u8> {
        let mut result = Vec::new();
        for _ in 0..size {
            result.push(self.read_u8());
        }
        result
    }

    pub fn read_u16_string(&mut self) -> String {
        let length = self.read_u16() as usize;
        let bytes = self.read_n_bytes(length);
        String::from_utf8_lossy(&bytes).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_big_endian_read() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = super::BinaryReader::new(data);
        reader.endian = super::Endian::Big;
        assert_eq!(reader.read_u64(), 0x0102030405060708);
    }

    #[test]
    fn test_little_endian_read() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = super::BinaryReader::new(data);
        reader.endian = super::Endian::Little;
        assert_eq!(reader.read_u64(), 0x0807060504030201);
    }

    #[test]
    fn test_u128() {
        let data = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ];
        let mut reader = super::BinaryReader::new(data);
        assert_eq!(reader.read_u128(), 0x0001020304050607080a0b0c0d0e0f10);
    }

    #[test]
    fn test_u64() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = super::BinaryReader::new(data);
        assert_eq!(reader.read_u64(), 0x0102030405060708);
    }

    #[test]
    fn test_u32() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = super::BinaryReader::new(data);
        assert_eq!(reader.read_u32(), 0x01020304);
    }

    #[test]
    fn test_u16() {
        let data = vec![0x01, 0x02];
        let mut reader = super::BinaryReader::new(data);
        assert_eq!(reader.read_u16(), 0x0102);
    }

    #[test]
    fn test_u8() {
        let data = vec![0x01];
        let mut reader = super::BinaryReader::new(data);
        assert_eq!(reader.read_u8(), 0x01);
    }

    #[test]
    fn test_half_64bit() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = super::BinaryReader::new(data);
        reader.is_64bit = true;
        assert_eq!(reader.read_half(), 0x01020304);
    }

    #[test]
    fn test_half_32bit() {
        let data = vec![0x01, 0x02];
        let mut reader = super::BinaryReader::new(data);
        reader.is_64bit = false;
        assert_eq!(reader.read_half(), 0x0102);
    }

    #[test]
    fn test_word_64bit() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = super::BinaryReader::new(data);
        reader.is_64bit = true;
        assert_eq!(reader.read_word(), 0x0102030405060708);
    }

    #[test]
    fn test_word_32bit() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = super::BinaryReader::new(data);
        reader.is_64bit = false;
        assert_eq!(reader.read_word(), 0x01020304);
    }

    #[test]
    fn test_dword_64bit() {
        let data = vec![
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10,
        ];
        let mut reader = super::BinaryReader::new(data);
        reader.is_64bit = true;
        assert_eq!(reader.read_dword(), 0x0102030405060708090a0b0c0d0e0f10);
    }

    #[test]
    fn test_dword_32bit() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = super::BinaryReader::new(data);
        reader.is_64bit = false;
        assert_eq!(reader.read_dword(), 0x0102030405060708);
    }

    #[test]
    fn test_read_n_bytes() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = super::BinaryReader::new(data);
        assert_eq!(reader.read_n_bytes(4), vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_read() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = super::BinaryReader::new(data);
        assert_eq!(reader.read(4, 2), vec![0x05, 0x06]);
    }

    #[test]
    fn test_read_all() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = super::BinaryReader::new(data);
        assert_eq!(
            reader.read(0, 8),
            vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
        );
    }

    #[test]
    fn test_peek_u8() {
        let data = vec![0x01, 0x02];
        let mut reader = super::BinaryReader::new(data);
        assert_eq!(reader.peek_u8(), 0x01);
        assert_eq!(reader.read_u8(), 0x01);
        assert_eq!(reader.peek_u8(), 0x02);
        assert_eq!(reader.read_u8(), 0x02);
    }
}
