/// Date/time related types

use chrono::{Date, DateTime, NaiveDate, NaiveTime, NaiveDateTime, UTC, Local};
use protocol::FIXValue;
use std::str;

/// Represents one of the days of a month, i.e. numbers 1-31.
///
/// Used in FIX Protocol Versions 4.1, 4.2, 4.4, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DayOfMonth {
    day: u8
}

impl DayOfMonth {
    /// Returns a new DayOfMonth if the value provided represents a valid day of month (i.e. 1-31).
    pub fn new(day: u8) -> Option<DayOfMonth> {
        match day > 0 && day < 32 {
            true => Some(DayOfMonth { day: day }),
            false => None
        }
    }
}

impl From<DayOfMonth> for u8 {
    fn from(d: DayOfMonth) -> u8 {
        d.day
    }
}

impl From<DayOfMonth> for u16 {
    fn from(d: DayOfMonth) -> u16 {
        d.day as u16
    }
}

impl From<DayOfMonth> for u32 {
    fn from(d: DayOfMonth) -> u32 {
        d.day as u32
    }
}

impl FIXValue for DayOfMonth {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<DayOfMonth> {
        u8::from_bytes(bytes).and_then(DayOfMonth::new)
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        u8::to_bytes(&self.day, v);
    }
}

/// Represents a month and a year, e.g. "March 2015."
///
/// Used in FIX Protocol Versions 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonthYear {
    monthyear: u32
}

impl MonthYear {
    /// Returns a new MonthYear if the specified year is between the year 0000 and 9999 (inclusive)
    /// and the specified month is a valid month (1-12, inclusive).
    /// E.g., `MonthYear::new(2015, 3)` for "March 2015".
    pub fn new(year: u32, month: u8) -> Option<MonthYear> {
        println!("{} {}", year, month);
        match month < 13 && month > 0 && year <= 9999 {
            true => Some(MonthYear { monthyear: year * 100 + month as u32 }),
            false => None
        }
    }

    /// Returns a new MonthYear if the numeric value represents a valid year between
    /// the year 0000 and 9999 (inclusive) and a valid month (1-12, inclusive).
    /// E.g., `MonthYear::new_from_monthyear(201503)` for "March 2015".
    pub fn new_from_yyyymm(monthyear: u32) -> Option<MonthYear> {
        MonthYear::new(monthyear / 100, (monthyear % 100) as u8)
    }

    /// Return the numeric representation of a MonthYear, e.g. 201503 for "March 2015".
    pub fn monthyear(&self) -> u32 {
        self.monthyear
    }

    /// Return the year of a MonthYear, e.g. 2015 for "March 2015".
    pub fn year(&self) -> u32 {
        self.monthyear / 100
    }

    /// Return the month of a MonthYear, e.g. 3 for "March 2015".
    pub fn month(&self) -> u8 {
        (self.monthyear % 100) as u8
    }
}

impl FIXValue for MonthYear {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<MonthYear> {
        u32::from_bytes(bytes).and_then(MonthYear::new_from_yyyymm)
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        u32::to_bytes(&self.monthyear, v);
    }
}


impl FIXValue for NaiveTime {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<NaiveTime> {
        match str::from_utf8(bytes).ok() {
            Some(bytestr) => {
                match bytestr.len() {
                    12 => NaiveTime::parse_from_str(bytestr, "%H:%M:%S%.3f").ok(),
                    8 => NaiveTime::parse_from_str(bytestr, "%H:%M:%S").ok(),
                    _ => None
                }
            },
            _ => None
        }
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self.format("%H:%M:%S%.3f").to_string().as_bytes());
    }
}

/// Date of Local Market (vs. UTC).
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type LocalMktDate = NaiveDate;

impl FIXValue for LocalMktDate {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<LocalMktDate> {
        str::from_utf8(bytes).ok()
            .and_then(|b| { NaiveDate::parse_from_str(b, "%Y%m%d").ok() })
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self.format("%Y%m%d").to_string().as_bytes());
    }
}

/// UTC date.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type UTCDateOnly = Date<UTC>;

impl FIXValue for UTCDateOnly {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<UTCDateOnly> {
        str::from_utf8(bytes).ok()
            .and_then(|b| { NaiveDate::parse_from_str(b, "%Y%m%d").ok() })
            .map(|dt| Date::from_utc(dt, UTC))
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self.format("%Y%m%d").to_string().as_bytes());
    }
}

/// UTC time.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type UTCTimeOnly = NaiveTime;

/// UTC date and time.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0 SP1, and 5.0 SP2
pub type UTCTimestamp = DateTime<UTC>;

impl FIXValue for UTCTimestamp {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<UTCTimestamp> {
        str::from_utf8(bytes).ok()
            .and_then(|b| {
                match b.len() {
                    21 => NaiveDateTime::parse_from_str(b, "%Y%m%d-%H:%M:%S%.3f").ok(),
                    17 => NaiveDateTime::parse_from_str(b, "%Y%m%d-%H:%M:%S").ok(),
                    _ => None
                }
            })
            .map(|dt| DateTime::from_utc(dt, UTC))
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
        v.extend(self.format("%Y%m%d-%H:%M:%S%.3f").to_string().as_bytes());
    }
}

/// ISO 8601 local time zone time.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
pub type TZTimeOnly = NaiveTime;

/// Local time zone date and time.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
pub type TZTimestamp = DateTime<Local>;

impl FIXValue for TZTimestamp {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<TZTimestamp> {
        None
    }

    #[inline]
    fn to_bytes(&self, v: &mut Vec<u8>) {
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::testutils::{check_decode_encode, check_no_decode};
    use chrono::*;
    use std::convert::From;

    #[test]
    fn test_dayofmonth() {
        let check_new = |n, v| { assert_eq!(DayOfMonth::new(n).map(|s| u8::from(s)), v); };

        check_new(1, Some(1));
        check_new(12, Some(12));
        check_new(30, Some(30));
        check_new(31, Some(31));
        check_new(0, None);
        check_new(32, None);
        check_new(220, None);

        check_decode_encode(b"1", DayOfMonth::new(1).unwrap());
        check_decode_encode(b"12", DayOfMonth::new(12).unwrap());
        check_decode_encode(b"30", DayOfMonth::new(30).unwrap());
        check_decode_encode(b"31", DayOfMonth::new(31).unwrap());

        check_no_decode::<DayOfMonth>(b"");
        check_no_decode::<DayOfMonth>(b"0");
        check_no_decode::<DayOfMonth>(b"32");
        check_no_decode::<DayOfMonth>(b"34");
    }

    #[test]
    fn test_monthyear() {
        let check_new = |n: u32, y: u32, m: u8| {
            assert_eq!(n, MonthYear::new(y, m).unwrap().monthyear());
            assert_eq!(y, MonthYear::new(y, m).unwrap().year());
            assert_eq!(m, MonthYear::new(y, m).unwrap().month());
        };

        let check_yyyymm = |yyyymm: u32| {
            assert_eq!(yyyymm, MonthYear::new_from_yyyymm(yyyymm).unwrap().monthyear());
        };

        let check_invalid = |y: u32, m: u8| { assert!(MonthYear::new(y, m).is_none()); };
        let check_invalid_yyyymm = |n: u32| { assert!(MonthYear::new_from_yyyymm(n).is_none()); };

        check_new(000112, 0001, 12);
        check_new(196112, 1961, 12);
        check_new(201501, 2015, 01);
        check_new(201503, 2015, 03);
        check_new(999911, 9999, 11);

        check_yyyymm(002101);
        check_yyyymm(001104);
        check_yyyymm(201502);
        check_yyyymm(428502);

        check_invalid(2015, 00);
        check_invalid(2015, 13);
        check_invalid(2015, 13);
        check_invalid(293840, 10);

        check_invalid_yyyymm(113);
        check_invalid_yyyymm(201500);
        check_invalid_yyyymm(201513);
        check_invalid_yyyymm(1234110);
        check_invalid_yyyymm(0);
    }

    #[test]
    fn test_time_only() {
        check_decode_encode(b"00:00:00.000", NaiveTime::from_hms_milli(0, 0, 0, 0));
        check_decode_encode(b"12:34:56.000", NaiveTime::from_hms(12, 34, 56));
        check_decode_encode(b"12:34:56.789", NaiveTime::from_hms_milli(12, 34, 56, 789));

        check_no_decode::<NaiveTime>(b"25:11:22");
        check_no_decode::<NaiveTime>(b"13:61:22.000");
    }

    #[test]
    fn test_date_utc() {
        check_decode_encode(b"20160122", UTC.ymd(2016, 1, 22));
        check_decode_encode(b"19011212", UTC.ymd(1901, 12, 12));
        check_decode_encode(b"20501001", UTC.ymd(2050, 10, 1));
    }

    #[test]
    fn test_localmktdate() {
        check_decode_encode(b"20160122", LocalMktDate::from_ymd(2016, 1, 22));
        check_decode_encode(b"19011212", LocalMktDate::from_ymd(1901, 12, 12));
        check_decode_encode(b"20501001", LocalMktDate::from_ymd(2050, 10, 01));
    }

    #[test]
    fn test_datetime_utc() {
    }

    #[test]
    fn test_datetime_local() {
    }
}
