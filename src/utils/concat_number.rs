pub fn concat_number<T>(high: T, low: T, shift: u8) -> u128
where
    T: num::ToPrimitive,
    T: std::ops::Shr<usize, Output = T>,
    T: num::Unsigned,
{
    let high = high.to_u128().unwrap();
    let low = low.to_u128().unwrap();

    (high << shift) | low
}

#[cfg(test)]
mod tests {
    use crate::utils::concat_number;
    #[test]
    fn test_32bit() {
        assert_eq!(
            concat_number(0x11223344 as u32, 0xaabbccdd as u32, 32),
            0x11223344aabbccdd
        );
    }
    #[test]
    fn test_16bit() {
        assert_eq!(concat_number(0x1122 as u16, 0xaabb as u16, 16), 0x1122aabb);
    }
}
