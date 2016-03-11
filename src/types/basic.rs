use std::str;
use std::iter::FromIterator;
use std::convert::Into;
use protocol::FIXValue;


/// Raw byte array datatype.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type Data = Vec<u8>;

impl FIXValue for Data {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Vec<u8>> {
        Some(bytes.to_vec())
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self);
    }
}

/// Floating point datatype.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type FIXFloat = f64;

impl FIXValue for FIXFloat {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<FIXFloat> {
        match str::from_utf8(bytes) {
            Ok(s) => s.parse().ok(),
            _ => None
        }
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self.to_string().as_bytes());
    }
}

/// Float field representing a Price times a Qty.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type Amt = FIXFloat;

/// Represents a percentage value as a decimal.
///
/// Used in FIX Protocol Versions 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type Percentage = FIXFloat;

/// Represents a price as a floating-point value.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type Price = FIXFloat;

/// Representing a price offset, which can be mathematically added to a Price.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type PriceOffset = FIXFloat;

/// Represents a whole number of securities or fractional quantity of securities.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Qty {
    /// Whole number of securities.
    Whole(u32),
    /// Fractional number of securities.
    Fraction(f64)
}

impl Qty {
    /// Return true if the Qty is a whole number, and false otherwise.
    pub fn is_whole(f: Qty) -> bool {
        match f {
            Qty::Whole(_) => true,
            Qty::Fraction(_) => false
        }
    }
}

impl From<Qty> for f64 {
    fn from(f: Qty) -> f64 {
        match f {
            Qty::Whole(w) => w as f64,
            Qty::Fraction(f) => f
        }
    }
}

impl FIXValue for Qty {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Qty> {
        match bytes.contains(&b'.') {
            true => f64::from_bytes(bytes).map(Qty::Fraction),
            false => u32::from_bytes(bytes).map(Qty::Whole)
        }
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        match *self {
            Qty::Whole(w) => u32::to_bytes(&w, v),
            Qty::Fraction(f) => f64::to_bytes(&f, v),
        };
    }
}

/// Integer value datatype.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type FIXInt = i32;

impl FIXValue for FIXInt {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<FIXInt> {
        match str::from_utf8(bytes) {
            Ok(s) => s.parse().ok(),
            _ => None
        }
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self.to_string().as_bytes());
    }
}

impl FIXValue for u32 {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<u32> {
        match str::from_utf8(bytes) {
            Ok(s) => s.parse().ok(),
            _ => None
        }
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self.to_string().as_bytes());
    }
}

impl FIXValue for u8 {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<u8> {
        match str::from_utf8(bytes) {
            Ok(s) => s.parse().ok(),
            _ => None
        }
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self.to_string().as_bytes());
    }
}

/// Represents a FIX message sequence number.
///
/// Used in FIX Protocol Versions 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type SeqNum = u32;

/// Represents a FIX tag number.
///
/// Used in FIX Protocol Versions 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type TagNum = u32;

/// Represents the number of entries in a repeating FIX message group.
///
/// Used in FIX Protocol Versions 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type NumInGroup = u32;

/// Represents the length in bytes of a FIX field.
///
/// Used in FIX Protocol Versions 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type Length = u32;

/// Boolean datatype.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0 SP1, and 5.0 SP2
pub type FIXBoolean = bool;

impl FIXValue for FIXBoolean {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<FIXBoolean> {
        match bytes {
            b"Y" => Some(true),
            b"N" => Some(false),
            _ => None
        }
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        match *self {
            true => v.push(b'Y'),
            false => v.push(b'N'),
        };
    }
}

/// One-byte character datatype. Cannot contain the separator character (SOH).
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct FIXChar {
    chr: u8
}

impl FIXChar {
    /// Returns a new FIXChar unless the provided character is the SOH (\x01) separator character.
    #[inline]
    pub fn new(chr: u8) -> Option<FIXChar> {
        match chr {
            b'\x01' => None,
            _ => Some(FIXChar { chr : chr })
        }
    }
}

impl From<FIXChar> for u8 {
    fn from(f: FIXChar) -> u8 {
        f.chr
    }
}

impl From<FIXChar> for char {
    fn from(f: FIXChar) -> char {
        f.chr as char
    }
}

impl FIXValue for FIXChar {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<FIXChar> {
        match bytes.len() {
            1 => FIXChar::new(bytes[0]),
            _ => None
        }
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.push(self.chr)
    }
}

/// Multi-byte string. Cannot contain the separator character (SOH).
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct FIXString {
    string: String
}

impl FIXString {
    /// Returns a new FIXString unless the SOH (\x01) separator character is in the string.
    #[inline]
    pub fn new<S: Into<String>>(string: S) -> Option<FIXString> {
        let s = string.into();
        match s.find('\x01') {
            None => Some(FIXString { string: s }),
            _ => None
        }
    }
}

impl From<FIXString> for String {
    fn from(fs: FIXString) -> String {
        fs.string
    }
}

impl From<MultipleValueString> for FIXString {
    fn from(mvs: MultipleValueString) -> FIXString {
        FIXString { string: String::from(mvs) }
    }
}

impl FIXValue for FIXString {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<FIXString> {
        str::from_utf8(bytes).ok()
            .map(|s| s.to_string())
            .and_then(FIXString::new)
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self.string.as_bytes())
    }
}

/// String datatype consisting of one or more space-separated Strings.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0 SP1, and 5.0 SP2
pub struct MultipleValueString {
    strings: Vec<FIXString>
}

/*impl From<FIXString> for MultipleValueString {
    fn from(s: FIXString) -> MultipleValueString {
        MultipleValueString { strings: s.string.split(' ') }
    }
}*/

impl From<MultipleValueString> for Vec<String> {
    fn from(mvs: MultipleValueString) -> Vec<String> {
        mvs.strings.iter().map(|fs| fs.string.to_string()).collect()
    }
}

impl From<MultipleValueString> for String {
    fn from(mvs: MultipleValueString) -> String {
        String::from_iter(mvs.strings.iter().map(|fs| fs.string.to_string()))
    }
}

/// String datatype consisting of one or more space-separated characters.
///
/// Used in FIX Protocol Versions 5.0 SP1, and 5.0 SP2
pub struct MultipleValueChar {
    chars: Vec<FIXChar>
}

/// Represents an XML document that can be sent through FIX.
///
/// Used in FIX Protocol Versions 5.0 SP1, and 5.0 SP2
pub type XMLData = Data;

/// Datatype representing positive integer values above 100.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Reserved100Plus {
    number: u32
}

impl Reserved100Plus {
    pub fn new(number: u32) -> Option<Reserved100Plus> {
        match number >= 100 {
            true => Some(Reserved100Plus { number: number }),
            false => None
        }
    }
}

impl From<Reserved100Plus> for u32 {
    fn from(r: Reserved100Plus) -> u32 { r.number }
}

impl FIXValue for Reserved100Plus {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Reserved100Plus> {
        u32::from_bytes(&bytes).and_then(Reserved100Plus::new)
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        u32::to_bytes(&self.number, v);
    }
}


/// Datatype representing positive integer values above 1000.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Reserved1000Plus {
    number: u32
}

impl Reserved1000Plus {
    pub fn new(number: u32) -> Option<Reserved1000Plus> {
        match number >= 1000 {
            true => Some(Reserved1000Plus { number: number }),
            false => None
        }
    }
}

impl From<Reserved1000Plus> for u32 {
    fn from(r: Reserved1000Plus) -> u32 { r.number }
}

impl FIXValue for Reserved1000Plus {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Reserved1000Plus> {
        u32::from_bytes(&bytes).and_then(Reserved1000Plus::new)
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        u32::to_bytes(&self.number, v);
    }
}

/// Datatype representing positive integer values above 4000.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Reserved4000Plus {
    number: u32
}

impl Reserved4000Plus {
    pub fn new(number: u32) -> Option<Reserved4000Plus> {
        match number >= 4000 {
            true => Some(Reserved4000Plus { number: number }),
            false => None
        }
    }
}

impl From<Reserved4000Plus> for u32 {
    fn from(r: Reserved4000Plus) -> u32 { r.number }
}

impl FIXValue for Reserved4000Plus {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Reserved4000Plus> {
        u32::from_bytes(&bytes).and_then(Reserved4000Plus::new)
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        u32::to_bytes(&self.number, v);
    }
}
/// Represents the standard FX tenors.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tenor {
    /// Day tenor.
    Day(u32),
    /// Week tenor.
    Week(u32),
    /// Month tenor.
    Month(u32),
    /// Year tenor.
    Year(u32),
}

impl FIXValue for Tenor {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Tenor> {
        match bytes[0] {
            b'D' => u32::from_bytes(&bytes[1..]).map(Tenor::Day),
            b'W' => u32::from_bytes(&bytes[1..]).map(Tenor::Week),
            b'M' => u32::from_bytes(&bytes[1..]).map(Tenor::Month),
            b'Y' => u32::from_bytes(&bytes[1..]).map(Tenor::Year),
            _ => None
        }
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        match *self {
            Tenor::Day(x) => { v.push(b'D'); u32::to_bytes(&x, v); },
            Tenor::Week(x) => { v.push(b'W'); u32::to_bytes(&x, v); },
            Tenor::Month(x) => { v.push(b'M'); u32::to_bytes(&x, v); },
            Tenor::Year(x) => { v.push(b'Y'); u32::to_bytes(&x, v); },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::testutils::{check_decode_encode, check_no_decode};
    use protocol::FIXValue;
    use std::convert::From;

    #[test]
    fn test_data() {
        check_decode_encode(b"", Data::new());
        check_decode_encode(b"abc", vec![b'a', b'b', b'c']);
        check_decode_encode(b"A\x012", vec![b'A', b'\x01', b'2']);
    }

    #[test]
    fn test_bool() {
        check_decode_encode(b"Y", true);
        check_decode_encode(b"N", false);
        check_no_decode::<FIXBoolean>(b"YY");
        check_no_decode::<FIXBoolean>(b"NN");
        check_no_decode::<FIXBoolean>(&Vec::new());
        check_no_decode::<FIXBoolean>(b"y");
        check_no_decode::<FIXBoolean>(b"n");
        check_no_decode::<FIXBoolean>(b"T");
        check_no_decode::<FIXBoolean>(b"F");
        check_no_decode::<FIXBoolean>(b"t");
        check_no_decode::<FIXBoolean>(b"f");

        // encode
        let mut v: Vec<u8> = vec![b'a', b'b', b'='];
        FIXBoolean::to_bytes(&true, &mut v);
        FIXBoolean::to_bytes(&false, &mut v);
        FIXBoolean::to_bytes(&true, &mut v);
        assert_eq!(&v, b"ab=YNY");
    }

    #[test]
    fn test_float() {
        // decode
        assert_eq!(0.0, FIXFloat::from_bytes(b"0.0").unwrap());
        assert_eq!(2.0, FIXFloat::from_bytes(b"2.0").unwrap());
        assert_eq!(-2.0, FIXFloat::from_bytes(b"-2.0").unwrap());
        check_no_decode::<FIXFloat>(b"1.0.0");

        assert_eq!(0.0, FIXFloat::from_bytes(b"0").unwrap());
        assert_eq!(0.0, FIXFloat::from_bytes(b"-0.0").unwrap());
        assert_eq!(2.0, FIXFloat::from_bytes(b"2").unwrap());
        assert_eq!(0.0, FIXFloat::from_bytes(b"0000.0000").unwrap());
        assert_eq!(2.0, FIXFloat::from_bytes(b"0002.0000").unwrap());
        assert_eq!(-2.0, FIXFloat::from_bytes(b"-0002.0000").unwrap());
    }

    #[test]
    fn test_int() {
        check_decode_encode(b"0", 0i32);
        check_decode_encode(b"1", 1i32);
        check_decode_encode(b"-1", -1i32);
        check_decode_encode(b"1000", 1000i32);
        check_decode_encode(b"-1000", -1000i32);

        assert_eq!(0, FIXInt::from_bytes(b"0000").unwrap());
        assert_eq!(0, FIXInt::from_bytes(b"-0").unwrap());
        assert_eq!(0, FIXInt::from_bytes(b"-0000").unwrap());
        assert_eq!(2, FIXInt::from_bytes(b"0002").unwrap());
        assert_eq!(-2, FIXInt::from_bytes(b"-0002").unwrap());

        check_no_decode::<FIXInt>(b"a");
        check_no_decode::<FIXInt>(b"1.0");
        check_no_decode::<FIXInt>(b"1.1");
    }

    #[test]
    fn test_u32() {
        check_decode_encode(b"0", 0u32);
        check_decode_encode(b"1", 1u32);
        check_decode_encode(b"1000", 1000u32);

        assert_eq!(0, u32::from_bytes(b"0000").unwrap());
        assert_eq!(2, u32::from_bytes(b"0002").unwrap());

        check_no_decode::<u32>(b"a");
        check_no_decode::<u32>(b"1.0");
        check_no_decode::<u32>(b"1.1");
        check_no_decode::<u32>(b"-0");
        check_no_decode::<u32>(b"-1");
    }

    #[test]
    fn test_fixchar() {
        // new
        assert!(FIXChar::new(b'B').is_some());
        assert!(FIXChar::new(b'\x00').is_some());
        assert!(FIXChar::new(b'\x02').is_some());
        assert!(FIXChar::new(b'\x01').is_none());

        // from
        assert_eq!(100u8, From::from(FIXChar::new(b'd').unwrap()));
        assert_eq!('d', From::from(FIXChar::new(b'd').unwrap()));

        check_decode_encode(b"\x00", FIXChar::new(b'\x00').unwrap());
        check_decode_encode(b"\x02", FIXChar::new(b'\x02').unwrap());
        check_decode_encode(b"\x03", FIXChar::new(b'\x03').unwrap());
        check_decode_encode(b"w", FIXChar::new(b'w').unwrap());
        check_decode_encode(b"W", FIXChar::new(b'W').unwrap());
        check_decode_encode(b"=", FIXChar::new(b'=').unwrap());
        check_decode_encode(b"1", FIXChar::new(b'1').unwrap());
        assert_eq!(FIXChar::from_bytes(b"aa"), None);
        assert_eq!(FIXChar::from_bytes(b""), None);
        assert_eq!(FIXChar::from_bytes(b"\x01"), None);

        // encode
        let mut v = Vec::new();
        FIXChar::new(b'a').unwrap().to_bytes(&mut v);
        FIXChar::new(b'b').unwrap().to_bytes(&mut v);
        FIXChar::new(b'c').unwrap().to_bytes(&mut v);
        FIXChar::new(b'=').unwrap().to_bytes(&mut v);
        FIXChar::new(b'1').unwrap().to_bytes(&mut v);
        FIXChar::new(b'2').unwrap().to_bytes(&mut v);
        assert_eq!(&v, b"abc=12");
    }

    #[test]
    fn test_fixstring() {
        let fs = |s: &str| { FIXString::new(s) };
        let check_new = |s: &str| { assert_eq!(s, String::from(fs(s).unwrap())); };

        check_new("");
        check_new("A");
        check_new("Good");
        check_new("     ");
        check_new("Good \x00\x02\x03 \n \n \n \x00\x02\x03");
        assert_eq!(fs("\x01"), None);
        assert_eq!(fs("A=10\x01"), None);

        check_decode_encode(b"", fs("").unwrap());
        check_decode_encode(b"A", fs("A").unwrap());
        check_decode_encode(b"Good", fs("Good").unwrap());
        check_decode_encode(b"\x00", fs("\x00").unwrap());

        check_no_decode::<FIXString>(b"\x01");
        check_no_decode::<FIXString>(b"A=35\x01");
        check_no_decode::<FIXString>(b"A=35\x0134");
    }

    #[test]
    fn test_multiplevaluechar() {
    }

    #[test]
    fn test_multiplevaluestring() {
        //assert_eq!(FIXString::from("
    }

    #[test]
    fn test_reserved() {
        // test encode/decode
        check_no_decode::<Reserved100Plus>(b"1");
        check_no_decode::<Reserved100Plus>(b"99");
        check_decode_encode(b"100", Reserved100Plus::new(100).unwrap());
        check_decode_encode(b"200", Reserved100Plus::new(200).unwrap());

        check_no_decode::<Reserved1000Plus>(b"10");
        check_no_decode::<Reserved1000Plus>(b"999");
        check_decode_encode(b"1000", Reserved1000Plus::new(1000).unwrap());
        check_decode_encode(b"2000", Reserved1000Plus::new(2000).unwrap());

        check_no_decode::<Reserved4000Plus>(b"1000");
        check_no_decode::<Reserved4000Plus>(b"3999");
        check_decode_encode(b"4000", Reserved4000Plus::new(4000).unwrap());
        check_decode_encode(b"4200", Reserved4000Plus::new(4200).unwrap());
    }

    #[test]
    fn test_tenor() {
        check_decode_encode(b"D0", Tenor::Day(0));
        check_decode_encode(b"D12", Tenor::Day(12));
        check_decode_encode(b"D9999", Tenor::Day(9999));
        check_decode_encode(b"W0", Tenor::Week(0));
        check_decode_encode(b"W12", Tenor::Week(12));
        check_decode_encode(b"W9999", Tenor::Week(9999));
        check_decode_encode(b"M0", Tenor::Month(0));
        check_decode_encode(b"M12", Tenor::Month(12));
        check_decode_encode(b"M9999", Tenor::Month(9999));
        check_decode_encode(b"Y0", Tenor::Year(0));
        check_decode_encode(b"Y12", Tenor::Year(12));
        check_decode_encode(b"Y9999", Tenor::Year(9999));
        assert_eq!(Tenor::from_bytes(b"D001").unwrap(), Tenor::Day(1));
        assert_eq!(Tenor::from_bytes(b"W001").unwrap(), Tenor::Week(1));

        check_no_decode::<Tenor>(b"100");
        check_no_decode::<Tenor>(b"D");
        check_no_decode::<Tenor>(b"D00 ");
        check_no_decode::<Tenor>(b"d12");
        check_no_decode::<Tenor>(b"w12");
    }
}
