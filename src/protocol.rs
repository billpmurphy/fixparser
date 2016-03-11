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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        // Sanity checks: handle mod 256, \x00 properly
        assert_eq!(0, compute_checksum(&Vec::new()));
        assert_eq!(0, compute_checksum(b"\x00\x00\x00\x00\x00"));
        assert_eq!(0, compute_checksum(b"~~\x01\x01\x01\x01"));
        assert_eq!(45, compute_checksum(b"\x01\x01(\x01\x01\x01"));
        assert_eq!(84, compute_checksum(b"dd(d\x00"));
        assert_eq!(185, compute_checksum(b"dddd\x01dddddddddd\x00dddddddddddddddd"));

        // Test case FIX messages
        assert_eq!(3, compute_checksum(b"8=FIX.4.1\x019=61\x0135=A\x0134=1\x0149=EXEC\x0152=\
                                       20121105-23:24:06\x0156=BANZAI\x0198=0\x01108=30\x01"));
        assert_eq!(228, compute_checksum(b"8=FIX.4.1\x019=49\x0135=0\x0134=2\x0149=BANZAI\x01\
                                        52=20121105-23:24:37\x0156=EXEC\x01"));
        assert_eq!(62, compute_checksum(b"8=FIX.4.1\x019=103\x0135=D\x0134=3\x0149=BANZAI\x01\
                                        52=20121105-23:24:42\x0156=EXEC\x0111=1352157882577\x01\
                                        21=1\x0138=10000\x0140=1\x0154=1\x0155=MSFT\x0159=0\x01"));
        assert_eq!(59, compute_checksum(b"8=FIX.4.1\x019=139\x0135=8\x0134=3\x0149=EXEC\x01\
                                        52=20121105-23:24:42\x0156=BANZAI\x016=0\x0111=\
                                        1352157882577\x0114=0\x0117=1\x0120=0\x0131=0\x0132=0\
                                        \x0137=1\x0138=10000\x0139=0\x0154=1\x0155=MSFT\x01150=2\
                                        \x01151=0\x01"));
        assert_eq!(230, compute_checksum(b"8=FIX.4.1\x019=153\x0135=8\x0134=4\x0149=EXEC\x01\
                                         52=20121105-23:24:42\x0156=BANZAI\x016=12.3\x0111=\
                                         1352157882577\x0114=10000\x0117=2\x0120=0\x0131=12.3\
                                         \x0132=10000\x0137=2\x0138=10000\x0139=2\x0154=1\x01\
                                         55=MSFT\x01150=2\x01151=0\x01"));
        assert_eq!(03, compute_checksum(b"8=FIX.4.1\x019=108\x0135=D\x0134=5\x0149=BANZAI\x01\
                                        52=20121105-23:25:12\x0156=EXEC\x0111=1352157912357\
                                        \x0121=1\x0138=10000\x0140=2\x0144=10\x0154=1\x0155=SPY\
                                        \x0159=0\x01"));
        assert_eq!(198, compute_checksum(b"8=FIX.4.1\x019=104\x0135=F\x0134=6\x0149=BANZAI\x01\
                                         52=20121105-23:25:16\x0156=EXEC\x0111=1352157916437\
                                         \x0138=10000\x0141=1352157912357\x0154=1\x0155=SPY\x01"));
    }
}
