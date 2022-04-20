pub enum Endian {
    Big,
    Little,
}

pub struct BinaryReader {
    pub data: Vec<u8>,
    pub offset: usize,

    pub endian: Endian,
    pub is_64bit: bool,
}

impl BinaryReader {
    pub fn new(data: Vec<u8>) -> BinaryReader {
        BinaryReader {
            data: data,
            offset: 0,
            endian: Endian::Big,
            is_64bit: false,
        }
    }

    fn _join_number<T>(&self, high: T, low: T, shift: usize) {
        let high = high as usize;
        let low = low as usize;

        if self.endian == Endian::Big {
            return (high << shift) | low;
        } else {
            return (low << shift) | high;
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let result = self.data[self.offset];
        self.offset += 1;
        result
    }

    pub fn read_u16(&mut self) -> u16 {
        let high = self.read_u8() as u16;
        let low = self.read_u8() as u16;
        self._join_number(high, low, 8) as u16
    }

    pub fn read_u32(&mut self) -> u32 {
        let high = self.read_u16() as u32;
        let low = self.read_u16() as u32;
        self._join_number(high, low, 16) as u32
    }

    pub fn read_u64(&mut self) -> u64 {
        let high = self.read_u32() as u64;
        let low = self.read_u32() as u64;
        self._join_number(high, low, 32) as u64
    }

    pub fn read_u128(&mut self) -> u128 {
        let high = self.read_u64() as u128;
        let low = self.read_u64() as u128;
        self._join_number(high, low, 64) as u128
    }

    pub fn read_half(&mut self) -> u32 {
        if self.is_64bit {
            self.read_u32()
        } else {
            self.read_u16()
        }
    }

    pub fn read_word(&mut self) -> u64 {
        if self.is_64bit {
            self.read_u64()
        } else {
            self.read_u32()
        }
    }

    pub fn read_dword(&mut self) -> u128 {
        if self.is_64bit {
            self.read_u128()
        } else {
            self.read_u64()
        }
    }
}
