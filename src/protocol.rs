/// The length in bytes of a raw FIX Message.
pub type MsgLen = usize;

/// The checksum of a raw FIX message.
pub type Checksum = u8;


/// Compute a FIX checksum given a slice of the header and body of the message, from the message
/// start to the delimiter before the `10` tag (inclusive).
#[inline]
pub fn compute_checksum(head_and_body: &[u8]) -> u8 {
    head_and_body.iter().fold(0, |a, &x| a.wrapping_add(x))
}


/// FIX Protocol Verison.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FIXVersion {
    /// FIX Protocol Version 4.0.
    FIX40,
    /// FIX Protocol Version 4.1.
    FIX41,
    /// FIX Protocol Version 4.2.
    FIX42,
    /// FIX Protocol Version 4.3.
    FIX43,
    /// FIX Protocol Version 4.4.
    FIX44,
    /// FIX Protocol Version 5.0.
    FIX50,
    /// FIX Protocol Version 5.0, Service Pack 1.
    FIX50SP1,
    /// FIX Protocol Version 5.0, Service Pack 2.
    FIX50SP2,
    /// FIX Transport Protocol 1.1.
    FIXT11,
}


/// A tag/value pair in a FIX message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TagValue<'a> {
    /// The tag number of the tag/value pair.
    pub tag: u32,
    /// The slice of bytes encoding the value of the tag/value pair.
    pub value: &'a[u8],
    /// The total length in bytes of the tag/value pair, including the closing SOH.
    pub len: usize,
}


/// Slice (in bytes) of the body of a message, excluding the header (tags 8, 9) and the checksum
/// (tag 10).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgBody<'a> {
    /// The protocol version of the message.
    pub version: FIXVersion,
    /// Slice of the body of the message.
    pub body: &'a[u8],
}
