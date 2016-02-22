use std::str;
use chrono::{Date, DateTime, NaiveTime, UTC, Local};
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
#[derive(Clone, Copy, PartialEq)]
pub enum Qty {
    /// Whole number of securities.
    Whole(u32),
    /// Fractional number of securities.
    Fraction(f64)
}

impl Qty {
    /// Return a Qty as a float value, even if the Qty is whole.
    pub fn as_float(f: Qty) -> f64 {
        match f {
            Qty::Whole(w) => w as f64,
            Qty::Fraction(f) => f
        }
    }

    /// Return true if the Qty is a whole number, and false otherwise.
    pub fn is_whole(f: Qty) -> bool {
        match f {
            Qty::Whole(_) => true,
            Qty::Fraction(_) => false
        }
    }
}

impl FIXValue for Qty {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Qty> {
        if bytes.contains(&b'.') {
            f64::from_bytes(bytes).map(Qty::Fraction)
        }
        else {
            u32::from_bytes(bytes).map(Qty::Whole)
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

    /// Getter method for FIXChar.
    #[inline]
    pub fn char(&self) -> u8 {
        self.chr
    }
}

impl FIXValue for FIXChar {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<FIXChar> {
        match bytes.len() {
            1 => None,
            _ => FIXChar::new(bytes[0])
        }
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.push(self.char())
    }
}

/// Multi-byte string. Cannot contain the separator character (SOH).
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0 SP1, and 5.0 SP2
pub struct FIXString {
    string: String
}

impl FIXString {
    /// Returns a new FIXString unless the SOH (\x01) separator character is in the string.
    #[inline]
    pub fn new(s: String) -> Option<FIXString> {
        match s.find('\x01') {
            Some(_) => Some(FIXString { string: s }),
            _ => None
        }
    }

    /// Getter method for FIXString.
    #[inline]
    pub fn string(&self) -> &str {
        &self.string
    }
}

impl FIXValue for FIXString {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<FIXString> {
        None
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self.string().as_bytes())
    }
}

/// Represents one of the days of a month, i.e. numbers 1-31.
///
/// Used in FIX Protocol Versions 4.1, 4.2, 4.4, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DayOfMonth {
    day: u8
}

impl DayOfMonth {
    /// Returns a new DayOfMonth if the value provided represents a valid day of month (i.e. 1-31).
    pub fn new(day: u8) -> Option<DayOfMonth> {
        if day > 0 && day < 32 {
            Some(DayOfMonth { day: day })
        }
        else {
            None
        }
    }

    pub fn day(&self) -> u8 {
        self.day
    }
}

impl FIXValue for DayOfMonth {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<DayOfMonth> {
        u8::from_bytes(bytes).and_then(DayOfMonth::new)
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        u8::to_bytes(&self.day(), v);
    }
}

/// Represents a month and a year, e.g. "March 2015."
///
/// Used in FIX Protocol Versions 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonthYear {
    monthyear: u32
}

impl MonthYear {
    /// Returns a new MonthYear if the numeric value represents a valid year between
    /// the year 0000 and 9999 (inclusive) and a valid month (1-12, inclusive).
    /// E.g., 201503 for "March 2015".
    pub fn new(monthyear: u32) -> Option<MonthYear> {
        let month = monthyear % 100;
        let year = monthyear / 100;

        if month < 13 && month > 0 && year <= 9999 {
            Some(MonthYear { monthyear: monthyear })
        }
        else {
            None
        }
    }

    /// Return the numeric representation of a MonthYear, e.g. 201503 for "March 2015".
    pub fn monthyear(&self) -> u32 {
        self.monthyear
    }

    /// Return the year of a MonthYear, e.g. 2015 for "March 2015".
    pub fn year(&self) -> u32 {
        (self.monthyear / 100)
    }

    /// Return the month of a MonthYear, e.g. 3 for "March 2015".
    pub fn month(&self) -> u8 {
        (self.monthyear % 100) as u8
    }
}

impl FIXValue for MonthYear {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<MonthYear> {
        u32::from_bytes(bytes).and_then(MonthYear::new)
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        u32::to_bytes(&self.monthyear(), v);
    }
}

/// Date of Local Market (vs. UTC).
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type LocalMktDate = Date<Local>;

/// UTC date.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type UTCDateOnly = Date<UTC>;

/// UTC time.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type UTCTimeOnly = NaiveTime;

/// UTC date and time.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0 SP1, and 5.0 SP2
pub type UTCTimestamp = DateTime<UTC>;

/// ISO 8601 local time zone time.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
pub type TZTimeOnly = NaiveTime;

/// Local time zone date and time.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
pub type TZTimestamp = DateTime<Local>;

/// String datatype consisting of one or more space-separated Strings.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0 SP1, and 5.0 SP2
pub type MultipleValueString = String;

/// String datatype consisting of one or more space-separated characters.
///
/// Used in FIX Protocol Versions 5.0 SP1, and 5.0 SP2
pub type MultipleValueChar = String;

/// Represents an XML document that can be sent through FIX.
///
/// Used in FIX Protocol Versions 5.0 SP1, and 5.0 SP2
pub type XMLData = Data;

/// Datatype representing positive integer values above 1000.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
pub type Reserved1000Plus = u32;

/// Datatype representing positive integer values above 4000.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
pub type Reserved4000Plus = u32;

/// Datatype representing positive integer values above 100.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
pub type Reserved100Plus = u32;

/// Represents the standard FX tenors.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Copy, PartialEq, Eq)]
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


#[cfg(test)]
mod tests {
    use super::*;
    use protocol::FIXValue;

    #[test]
    fn test_data() {
        // encode
        let v1: Vec<u8> = vec![b'0', b'1', b'p', b'2'];
        let d: Data = Data::from_bytes(&v1).unwrap();

        // decode
        let mut v2: Vec<u8> = Vec::new();
        d.to_bytes(&mut v2);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_bool() {
        // decode
        assert_eq!(true, FIXBoolean::from_bytes(b"Y").unwrap());
        assert_eq!(false, FIXBoolean::from_bytes(b"N").unwrap());
        assert!(FIXBoolean::from_bytes(b"YY").is_none());
        assert!(FIXBoolean::from_bytes(b"NN").is_none());
        assert!(FIXBoolean::from_bytes(&Vec::new()).is_none());
        assert!(FIXBoolean::from_bytes(b"y").is_none());
        assert!(FIXBoolean::from_bytes(b"n").is_none());
        assert!(FIXBoolean::from_bytes(b"T").is_none());
        assert!(FIXBoolean::from_bytes(b"F").is_none());
        assert!(FIXBoolean::from_bytes(b"t").is_none());
        assert!(FIXBoolean::from_bytes(b"f").is_none());

        // encode
        let mut v: Vec<u8> = vec![b'a', b'b', b'='];
        FIXBoolean::to_bytes(&true, &mut v);
        FIXBoolean::to_bytes(&false, &mut v);
        assert_eq!(&v, b"ab=YN");
    }

    #[test]
    fn test_float() {
        // decode
        assert_eq!(0.0, FIXFloat::from_bytes(b"0").unwrap());
        assert_eq!(0.0, FIXFloat::from_bytes(b"0.0").unwrap());
        assert_eq!(0.0, FIXFloat::from_bytes(b"-0.0").unwrap());
        assert_eq!(0.0, FIXFloat::from_bytes(b"0000.0000").unwrap());

        // encode
        let mut v: Vec<u8> = vec![b'a', b'b', b'='];
        FIXFloat::to_bytes(&1.234, &mut v);
        assert_eq!(&v, b"ab=1.234");
    }

/*    #[test]
    fn test_int() {
    }

    #[test]
    fn test_u8() {
    }

    #[test]
    fn test_u32() {
    }*/
}
