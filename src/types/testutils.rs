/// Utility functions for testing FIX value types

use protocol::FIXValue;
use std::fmt::Debug;

/// Check successful encode/decode
#[allow(dead_code)]
pub fn check_decode_encode<T: Debug + PartialEq + FIXValue>(bytes: &[u8], value: T) {
    // decode
    assert_eq!(value, FIXValue::from_bytes(bytes).unwrap());

    // encode
    let mut v = Vec::new();
    value.to_bytes(&mut v);
    assert_eq!(&v[..], bytes);
}

/// Check unsuccessful decode
#[allow(dead_code)]
pub fn check_no_decode<T: Debug + PartialEq + FIXValue>(bytes: &[u8]) {
    assert_eq!(T::from_bytes(bytes), None);
}
