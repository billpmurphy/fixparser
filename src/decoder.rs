use std::{result, str};
use types::{Qty, MonthYear, Tenor};


#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    InvalidChar(u8),
    StrDecodeError(str::Utf8Error),
    Overflow
}

pub type Result<T> = result::Result<T, DecodeError>;

/// Find and parse the next tag in a slice of bytes representing a raw FIX message.
/// TODO: make more efficient
/*
pub fn parse_next_tag(msg: &[u8]) -> Result<TagValue, PreprocessError>{
    let eq = try!(msg.iter().position(|&n| n == EQ).ok_or(PreprocessError::Invalid));
    let end = try!(msg.iter().position(|&n| n == SOH).ok_or(PreprocessError::Invalid));
    let tag = try!(decode_u32(&msg[0..eq]).map_err(|_| PreprocessError::Invalid));
    let tag = 0;
    Ok(TagValue { tag : tag, value: &msg[eq..end], len: end })
}
*/


#[inline]
pub fn decode_bool(&msg: &u8) -> Result<bool> {
    match msg {
        b'Y' => Ok(true),
        b'N' => Ok(false),
        _ => Err(DecodeError::InvalidChar(msg))
    }
}


#[inline]
pub fn decode_i32(msg: &[u8]) -> Result<i32> {
    unimplemented!()
}


#[inline]
pub fn decode_f64(msg: &[u8]) -> Result<f64> {
    unimplemented!()
}


#[inline]
pub fn decode_u32(msg: &[u8]) -> Result<u32> {
    unimplemented!()
}


#[inline]
pub fn decode_fixchar(&msg: &u8) -> Result<u8> {
    match msg {
        b'\x01' => Err(DecodeError::InvalidChar(b'\x01')),
        ch => Ok(ch)
    }
}


#[inline]
pub fn decode_fixstring(msg: &[u8]) -> Result<&str> {
    match str::from_utf8(msg) {
        Ok(s) => Ok(s),
        Err(e) => Err(DecodeError::StrDecodeError(e)),
    }
}


#[inline]
pub fn decode_dayofmonth(msg: &[u8; 2]) -> Result<u8> {
    unimplemented!()
}


#[inline]
pub fn decode_monthyear(msg: &[u8; 6]) -> Result<MonthYear>{
    unimplemented!()
}


#[inline]
pub fn decode_qty(msg: &[u8]) -> Result<Qty> {
    unimplemented!()
}


#[inline]
pub fn decode_tenor(msg: &[u8]) -> Result<Tenor> {
    match msg[0] {
        b'D' => Ok(Tenor::Day(try!(decode_u32(&msg[1..])))),
        b'W' => Ok(Tenor::Week(try!(decode_u32(&msg[1..])))),
        b'M' => Ok(Tenor::Month(try!(decode_u32(&msg[1..])))),
        b'Y' => Ok(Tenor::Year(try!(decode_u32(&msg[1..])))),
        _ => Err(DecodeError::InvalidChar(msg[0]))
    }
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
                assert!(decode_bool(&i).is_err());
            }
        }
    }

    #[test]
    fn test_decode_i32() {
    }

    #[test]
    fn test_decode_f64() {
    }

    #[test]
    fn test_decode_u32() {
    }

    #[test]
    fn test_decode_fixchar() {
        for i in 0..255 {
            if i == b'\x01' {
                assert!(decode_fixchar(&i).is_err());
            }
            else {
                assert_eq!(decode_fixchar(&i), Ok(i));
            }
        }
    }

    #[test]
    fn test_decode_fixstring() {
    }

    #[test]
    fn test_decode_dayofmonth() {
    }

    #[test]
    fn test_decode_monthyear() {
    }

    #[test]
    fn test_decode_qty() {
    }

    #[test]
    fn test_decode_tenor() {
    }
}
