#[inline]
pub fn cautious<T>(hint: u32) -> usize {
    let el_size = std::mem::size_of::<T>() as u32;
    std::cmp::max(std::cmp::min(hint, 4096 / el_size), 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_cautious_u8() {
        assert_eq!(cautious::<u8>(10), 10);
    }
}
