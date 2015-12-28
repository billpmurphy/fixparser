/// Preprocess a raw FIX message.
use decoder::decode_u32;
use protocol::{FIXVersion, MIN_MESSAGE_LEN, SOH, EQ, TagValue, MsgBody};


/// Find and parse the next tag in a slice of bytes representing a raw FIX message.
/// TODO: make more efficient
pub fn parse_next_tag(msg: &[u8]) -> Result<TagValue, PreprocessError>{
    let eq = try!(msg.iter().position(|&n| n == EQ).ok_or(PreprocessError::Invalid));
    let end = try!(msg.iter().position(|&n| n == SOH).ok_or(PreprocessError::Invalid));
    let tag = try!(decode_u32(&msg[0..eq]).map_err(|_| PreprocessError::Invalid));
    let tag = 0;
    Ok(TagValue { tag : tag, value: &msg[eq..end], len: end })
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreprocessError {
    Incomplete,
    Invalid,
    InvalidVersion,
    InvalidMsgLength,
    InvalidChecksum,
    InvalidTerminator,
    BadChecksum,
}


/// Compute a FIX checksum given a slice of the header and body of the message, from the message
/// start to the delimiter before the `10` tag (inclusive).
#[inline]
pub fn compute_checksum(body: &[u8]) -> u8 {
    body.iter().fold(0, |a, &x| a.wrapping_add(x))
}

/// Convert a 3-byte sequence into a u8 checksum value.
fn extract_checksum_from_bytestring(bytes: &[u8]) -> Result<u8, PreprocessError> {
    if bytes.len() != 3
        || bytes[0] < b'0' || bytes[0] > b'2'
        || bytes[1] < b'0' || bytes[1] > b'9'
        || bytes[2] < b'0' || bytes[2] > b'9'  {
            Err(PreprocessError::InvalidChecksum)
    }
    else {
        Ok((bytes[0] - b'0') * 100 + (bytes[1] - b'0') * 10 + (bytes[2] - b'0'))
    }
}

/// Preprocess a FIX message.
pub fn preprocess(msg: &[u8]) -> Result<MsgBody, PreprocessError> {
    // Must be at least as long as 8=FIXT.1.1^9=0^10=000^
    if msg.len() < MIN_MESSAGE_LEN {
        return Err(PreprocessError::Incomplete)
    }

    // All FIX messages must begin with 8=FIX
    if &msg[0..5] != b"8=FIX" {
        return Err(PreprocessError::InvalidVersion)
    }

    let version;
    let mut index;

    // Find the protocol verison.
    // For version 4.X, the tag 8 will be 8=FIX.4.X.
    if &msg[5..8] == b".4." {
        index = 9;
        version = match msg[8] {
            b'0' => FIXVersion::FIX40,
            b'1' => FIXVersion::FIX41,
            b'2' => FIXVersion::FIX42,
            b'3' => FIXVersion::FIX43,
            b'4' => FIXVersion::FIX44,
            _ => return Err(PreprocessError::InvalidVersion)
        }
    }
    // For version 5.0, the tag will be 8=FIXT.1.1
    else if &msg[5..10] == b"T.1.1" {
        index = 10;
        version = FIXVersion::FIXT11;
    }
    else {
        return Err(PreprocessError::InvalidVersion)
    }

    // Next tag after 8=FIX<VERSION> is 9=<BODY_LEN>
    if &msg[index..index+3] != b"\x019=" {
        return Err(PreprocessError::InvalidMsgLength)
    }
    index += 3;

    // Extract the length of the body from the message.
    // Length is the number of characters starting at tag 35, and ending at tag 10 (exclusive).
    // For example:
    //
    // 8=FIX.4.2|9=65|35=A|49=SERVER|56=CLIENT|34=177|52=20090107-18:15:16|98=0|108=30|10=065|
    //      0   + 0  + 5  +   10    +   10    +  7   +        21          + 5  +  7   +   0    = 65
    let mut tag_9_len = 0;
    let mut body_len = 0;
    loop {
        if msg[index] < b'0' || msg[index] > b'9' {
            if msg[index] == SOH {
                if tag_9_len == 0 {
                    return Err(PreprocessError::InvalidMsgLength)
                }
                break;
            }
            else {
                return Err(PreprocessError::InvalidMsgLength)
            }
        }

        body_len = (body_len * 10) + (msg[index] - b'0') as usize;
        index += 1;

        tag_9_len += 1;
        if tag_9_len > 6 {
            return Err(PreprocessError::InvalidMsgLength)
        }
    }
    index += 1; // advance to start of first body tag
    let body_start_index = index;

    if msg.len() < body_len {
        return Err(PreprocessError::InvalidMsgLength)
    }

    index += body_len; // advance to first byte after end of body
    if &msg[index-1..index+3] != b"\x0110=" {
        return Err(PreprocessError::Invalid)
    }

    index += 3; // advance to start of checksum
    let declared_checksum = try!(extract_checksum_from_bytestring(&msg[index..index+3]));
    let actual_checksum = compute_checksum(&msg[0..body_start_index+body_len]);

    if declared_checksum != actual_checksum {
        return Err(PreprocessError::BadChecksum)
    }
    Ok(MsgBody { version: version, body: &msg[body_start_index..body_start_index+body_len] })
}


#[cfg(test)]
mod tests {
    use super::*;
}
