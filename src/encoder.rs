#[derive(Debug, PartialEq, Eq)]
pub struct EncodeError;

#[inline]
pub fn encode_bool(b: bool) -> u8 {
    match b {
        true => b'Y',
        false => b'N'
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_bool() {
        assert_eq!(encode_bool(true), b'Y');
        assert_eq!(encode_bool(false), b'N');
    }
}
