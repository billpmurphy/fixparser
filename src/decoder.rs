use std::result;

#[derive(Debug, PartialEq, Eq)]
pub struct DecodeError;

pub type Result<T> = result::Result<T, DecodeError>;


#[inline]
pub fn decode_bool(&msg: &u8) -> Result<bool> {
    match msg {
        b'Y' => Ok(true),
        b'N' => Ok(false),
        _ => Err(DecodeError)
    }
}


#[inline]
pub fn decode_i32(msg: &[u8]) {
    unimplemented!()
}


#[inline]
pub fn decode_f64(msg: &[u8]) {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_bool() {
        for i in 0..255 {
            if i == b'Y' {
                assert_eq!(decode_bool(&i), Ok(true));
            }
            else if i == b'N' {
                assert_eq!(decode_bool(&i), Ok(false));
            }
            else {
                assert_eq!(decode_bool(&i), Err(DecodeError));
            }
        }
    }

    #[test]
    fn test_decode_i32() {
    }

    #[test]
    fn test_decode_f64() {
    }
}
