use std::result;
use types::{Qty, MonthYear, Tenor};


#[derive(Debug, PartialEq, Eq)]
pub enum EncodeError {
    InvalidChar(u8),
}

pub type Result<T> = result::Result<T, EncodeError>;


#[inline]
pub fn encode_bool(b: bool) -> u8 {
    match b {
        true => b'Y',
        false => b'N'
    }
}


/*#[inline]
pub fn encode_i32(i: &i32) -> &[u8] {
    unimplemented!()
}


#[inline]
pub fn encode_f64(f: &f64) -> &[u8] {
    unimplemented!()
}


#[inline]
pub fn encode_u32(i: &u32) -> &[u8] {
    unimplemented!()
}*/


#[inline]
pub fn encode_fixchar(&chr: &u8) -> Result<u8> {
    match chr {
        b'\x01' => Err(EncodeError::InvalidChar(chr)),
        _ => Ok(chr)
    }
}


/*#[inline]
pub fn encode_fixstring(string: &str) -> Result<&[u8]> {
    unimplemented!()
}*/


#[inline]
pub fn encode_dayofmonth(dom: u8) -> Result<[u8; 2]> {
    unimplemented!()
}


#[inline]
pub fn encode_monthyear(my: MonthYear) -> Result<[u8; 6]>{
    unimplemented!()
}


/*#[inline]
pub fn encode_qty(qty: Qty) -> Result<&[u8]> {
    unimplemented!()
}


#[inline]
pub fn encode_tenor(msg: Tenor) -> Result<&[u8]> {
    unimplemented!()
}*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_bool() {
        assert_eq!(encode_bool(true), b'Y');
        assert_eq!(encode_bool(false), b'N');
    }

    #[test]
    fn test_encode_i32() {
    }

    #[test]
    fn test_encode_f64() {
    }

    #[test]
    fn test_encode_u32() {
    }

    #[test]
    fn test_encode_fixchar() {
    }

    #[test]
    fn test_encode_fixstring() {
    }

    #[test]
    fn test_encode_dayofmonth() {
    }

    #[test]
    fn test_encode_monthyear() {
    }

    #[test]
    fn test_encode_qty() {
    }

    #[test]
    fn test_encode_tenor() {
    }
}
