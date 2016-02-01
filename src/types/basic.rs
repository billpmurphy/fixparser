/// Raw byte array datatype.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type Data = Vec<u8>;

/// Floating point datatype.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type FIXFloat = f64;

/// Float field representing a Price times a Qty.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type Amt = f64;

/// Represents a percentage value as a decimal.
///
/// Used in FIX Protocol Versions 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type Percentage = f64;

/// Represents a price as a floating-point value.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type Price = f64;

/// Representing a price offset, which can be mathematically added to a Price.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type PriceOffset = f64;

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
    fn as_float(f: Qty) -> f64 {
        match f {
            Qty::Whole(w) => w as f64,
            Qty::Fraction(f) => f
        }
    }
}

/// Integer value datatype.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type FIXInt = i32;

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

/// One-byte character datatype. Cannot contain the separator character (SOH).
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type FIXChar = u8;

/// Multi-byte string. Cannot contain the separator character (SOH).
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0 SP1, and 5.0 SP2
pub type FIXString = String;

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

        if month < 13 && month > 0 && year >= 0 && year <= 9999 {
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

/// Date of Local Market (vs. UTC).
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type LocalMktDate = String;

/// UTC date.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type UTCDateOnly = String;

/// UTC time.
///
/// Used in FIX Protocol Versions 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
pub type UTCTimeOnly = String;

/// UTC date and time.
///
/// Used in FIX Protocol Versions 4.2, 4.3, 4.4, 5.0 SP1, and 5.0 SP2
pub type UTCTimestamp = String;

/// ISO 8601 local time zone time.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
pub type TZTimeOnly = String;

/// Local time zone date and time.
///
/// Used in FIX Protocol Versions 5.0, 5.0 SP1, and 5.0 SP2
pub type TZTimestamp = String;

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
pub type XMLData = String;

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
