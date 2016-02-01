pub trait FIXField where Self: Sized {
    /// Decode a single byte to a value of a FIX enum field.
    fn from_bytes(bytes: &[u8]) -> Option<Self>;

    /// Encode a value to bytes and append it to a vector.
    fn to_bytes(&self, mut v: &mut Vec<u8>);
}
