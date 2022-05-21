pub fn find_zero(data: Vec<u8>, offset: usize) -> usize {
    if data.len() == 0 {
        return offset;
    }
    let mut offset = offset;
    while data[offset] != 0 {
        offset += 1;
    }
    offset
}

#[cfg(test)]
mod tests {
    use crate::utils::find_zero;
    #[test]
    fn test_all() {
        for i in 1..100 {
            let mut data = vec![1; i];
            data[i - 1] = 0;
            assert_eq!(find_zero(data, 0), i - 1, "i: {}", i);
        }
    }
}
