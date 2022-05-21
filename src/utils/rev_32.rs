pub fn rev_32(x: u32) -> u32 {
    (x & 0xFF) << 0x18
        | ((x >> 0x08) & 0xFF) << 0x10
        | ((x >> 0x10) & 0xFF) << 0x08
        | ((x >> 0x18) & 0xFF)
}

#[cfg(test)]
mod tests {
    use crate::utils::rev_32;
    #[test]
    fn test_rev1() {
        assert_eq!(rev_32(0x12345678), 0x78563412);
    }
    #[test]
    fn test_rev2() {
        assert_eq!(rev_32(0x78563412), 0x12345678);
    }
}
