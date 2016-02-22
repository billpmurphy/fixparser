/// Types and traits for the FIX Protocol.

/// The length in bytes of a raw FIX Message.
pub type MsgLen = usize;

/// The checksum of a raw FIX message.
pub type Checksum = u8;


/// Compute a FIX checksum given a slice of the header and body of the message.
///
/// The FIX checksum is the sum of the bytes in the header and body of the message modulo 256.
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
    /// The message checksum.
    pub checksum: u8,
}


/// Trait for value types related to the FIX protocol that can be encoded/decoded to bytearrays.
pub trait FIXValue where Self: Sized {
    /// Decode a value from a slice of bytes.
    fn from_bytes(bytes: &[u8]) -> Option<Self>;

    /// Encode a value and append it to a byte vector.
    fn to_bytes(&self, mut v: &mut Vec<u8>);
}


/// Common interface for all FIX message types.
pub trait FIXMessage where Self: Sized {
    /// Return true if the message is a Heartbeat, and false otherwise.
    //fn is_heartbeat(&self) -> bool;

    /// Return true if the message is a TestRequest, and false otherwise.
    //fn is_test_request(&self) -> bool;

    /// Create a empty Heartbeat message.
    fn make_empty_heartbeat() -> Self;

    /// Create a Heartbeat message to respond to the message.
    ///
    /// If the message is a TestRequest message and contains a TestRequestID, the Heartbeat
    /// response will also contain the TestRequestID. Otherwise, an empty Heartbeat will be
    /// returned.
    fn make_heartbeat_response(&self) -> Self;

    /// Create a TestRequest message with an optional TestRequestID.
    fn make_test_request(test_request_id: Option<&str>) -> Self;
}
