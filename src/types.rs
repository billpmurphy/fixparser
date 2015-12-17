#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FIXVersion {
    FIX40,
    FIX41,
    FIX42,
    FIX43,
    FIX44,
    FIX50,
    FIX50SP1,
    FIX50SP2,
    FIXT11,
}

pub type Float = f64;
pub type Percentage = f64;
pub type Qty = f64;
pub type Amt = f64;
pub type Price = f64;
pub type PriceOffset = f64;

pub type Int = f64;
pub type SeqNum = f64;
pub type NumInGroup = f64;
pub type Length = f64;

pub type Boolean = bool;

pub type Char = u8;

pub type Currency = String;
pub type Country = String;
pub type Exchange = String;
pub type Language = String;

pub type DayOfMonth = i32;
