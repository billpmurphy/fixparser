/// Preprocess a raw FIX message.

use std::result;
use types::FIXVersion;

const SOH: u8 = 1; // SOH control character
const EQ: u8 = 61; // = character


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreprocessInfo {
    /// The version of the FIX protocol declared in the message header (tag 8).
    pub version: FIXVersion,
    /// The length of the body of the message in bytes (tag 9).
    pub body_len: usize,
    /// Index of the beginning of the body, i.e. tag 35
    body_start_index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreprocessError {
    Incomplete,
    Invalid,
    InvalidMsgType,
    InvalidMsgLength,
    InvalidChecksum,
    InvalidTerminator,
    BadChecksum,
}

///
pub type Result = result::Result<PreprocessInfo, PreprocessError>;

///
///
pub fn preprocess(msg: &[u8]) -> Result {
    // Must be at least as long as 8=FIXT.1.1^9=0^10=000^
    if msg.len() < 22 {
        return Err(PreprocessError::Incomplete)
    }

    // All FIX messages must begin with 8=FIX
    if msg[0] != b'8' || msg[1] != b'=' || msg[2] != b'F' || msg[3] != b'I' || msg[4] != b'X' {
        return Err(PreprocessError::InvalidMsgType)
    }

    let version;
    let mut index;

    // Find the protocol verison.
    // For version 4.X, the tag 8 will be 8=FIX.4.X.
    if msg[5] == b'.' && msg[6] == b'4' && msg[7] == b'.' {
        index = 9;
        version = match msg[8] {
            b'0' => FIXVersion::FIX40,
            b'1' => FIXVersion::FIX41,
            b'2' => FIXVersion::FIX42,
            b'3' => FIXVersion::FIX43,
            b'4' => FIXVersion::FIX44,
            _ => return Err(PreprocessError::InvalidMsgType)
        }
    }
    // For version 5.0, the tag will be 8=FIXT.1.1
    else if msg[5] == b'T' && msg[6] == b'.' && msg[7] == b'1' && msg[8] == b'.' && msg[9] == b'1' {
        index = 10;
        version = FIXVersion::FIXT11;
    }
    else {
        return Err(PreprocessError::InvalidMsgType)
    }

    // Next tag after 8=FIX<VERSION> is 9=<BODY_LEN>
    if msg[index] != SOH || msg[index+1] != b'9' || msg[index+2] != EQ {
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
    if msg[index-1] != SOH || msg[index] != b'1' || msg[index+1] != b'0' || msg[index+2] != EQ {
        return Err(PreprocessError::Invalid)
    }

    let mut checksum: u32 = 0;
    index += 3; // advance to start of checksum

    // Extract the checksum from the 10=XXX tag.
    for _ in 0..3 {
        if msg[index] < b'0' || msg[index] > b'9' {
            return Err(PreprocessError::InvalidChecksum)
        }
        checksum = (checksum * 10) + (msg[index] - b'0') as u32;
        index += 1;
    }

    if checksum > 255 {
        return Err(PreprocessError::InvalidChecksum)
    }

    let byte_sum: u32 = msg[0..body_start_index+body_len].iter().fold(0, |a, &x| a + x as u32);
    if checksum != (byte_sum % 256) {
        return Err(PreprocessError::BadChecksum)
    }

    Ok(PreprocessInfo { version: version, body_len: body_len, body_start_index: body_start_index })
}
