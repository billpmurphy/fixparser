/// Preprocess a raw FIX message.
use decoder::decode_u32;
use protocol::{FIXVersion, MsgBody, MsgLen, Checksum, compute_checksum};

/// The minimum possible length of a FIX message.
const MIN_MESSAGE_LEN: usize = 22;

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


fn parse_tag_8(msg: &[u8]) -> Result<(FIXVersion, usize), PreprocessError> {
    // All FIX messages must begin with 8=FIX
    if msg.len() < 10 || &msg[0..5] != b"8=FIX" {
        return Err(PreprocessError::InvalidVersion)
    }

    // Find the protocol verison.
    // For version 4.X, the tag 8 will be 8=FIX.4.X|
    if &msg[5..8] == b".4." && msg[9] == b'\x01' {
        match msg[8] {
            b'0' => Ok((FIXVersion::FIX40, 10)),
            b'1' => Ok((FIXVersion::FIX41, 10)),
            b'2' => Ok((FIXVersion::FIX42, 10)),
            b'3' => Ok((FIXVersion::FIX43, 10)),
            b'4' => Ok((FIXVersion::FIX44, 10)),
            _ => Err(PreprocessError::InvalidVersion)
        }
    }
    // For version 5.0, the tag will be 8=FIXT.1.1|
    else if &msg[5..11] == b"T.1.1\x01" {
        Ok((FIXVersion::FIXT11, 11))
    }
    else {
        Err(PreprocessError::InvalidVersion)
    }
}


fn parse_tag_9(msg: &[u8]) -> Result<(MsgLen, usize), PreprocessError> {
    if msg.len() < 4 || &msg[0..2] != b"9=" {
        Err(PreprocessError::Invalid)
    }
    else {
        let mut msg_len = 0;
        let mut body_start_index = 3;
        for &byte in &msg[2..] {
            if byte == b'\x01' {
                return if body_start_index == 3 {
                    Err(PreprocessError::Invalid)
                }
                else {
                    Ok((msg_len, body_start_index))
                };
            }
            else if byte >= b'0' && byte <= b'9' {
                body_start_index += 1;
                msg_len = (msg_len * 10) + (byte - b'0') as MsgLen;
            }
            else {
                return Err(PreprocessError::Invalid);
            }
        }
        Err(PreprocessError::Invalid)
    }
}

fn parse_tag_10(msg: &[u8]) -> Result<Checksum, PreprocessError> {
    if msg.len() != 7 || &msg[0..3] != b"10=" || msg[6] != b'\x01'
        || msg[3] < b'0' || msg[3] > b'2'
        || msg[4] < b'0' || msg[4] > b'9'
        || msg[5] < b'0' || msg[5] > b'9'  {
         Err(PreprocessError::InvalidChecksum)
    }
    else {
        (100 * (msg[3] - b'0'))
            .checked_add(10 * (msg[4] - b'0'))
            .and_then(|x| x.checked_add(msg[5] - b'0'))
            .ok_or(PreprocessError::InvalidChecksum)
    }
}


/// Preprocess a FIX message.
pub fn preprocess(msg: &[u8]) -> Result<MsgBody, PreprocessError> {
    if msg.len() < MIN_MESSAGE_LEN {
        return Err(PreprocessError::Incomplete)
    }

    let (version, tag_9_start_index) = try!(parse_tag_8(&msg));
    let (body_len, tag_9_len) = try!(parse_tag_9(&msg[tag_9_start_index..]));

    let body_start_index = tag_9_start_index + tag_9_len;
    let tag_10_start_index = body_start_index + body_len;

    let declared_checksum = try!(parse_tag_10(&msg[tag_10_start_index..]));
    let actual_checksum = compute_checksum(&msg[0..tag_10_start_index]);

    if declared_checksum != actual_checksum {
        Err(PreprocessError::BadChecksum)
    }
    else {
        Ok(MsgBody { version: version, body: &msg[body_start_index..tag_10_start_index] })
    }
}


#[cfg(test)]
mod tests {
    use super::{parse_tag_8, parse_tag_9, parse_tag_10};
    use protocol::*;

    #[test]
    fn test_parse_tag_8() {
        assert_eq!(parse_tag_8(b"8=FIX.4.0\x019=000").unwrap(), (FIXVersion::FIX40, 10));
        assert_eq!(parse_tag_8(b"8=FIX.4.1\x019=000").unwrap(), (FIXVersion::FIX41, 10));
        assert_eq!(parse_tag_8(b"8=FIX.4.2\x019=000").unwrap(), (FIXVersion::FIX42, 10));
        assert_eq!(parse_tag_8(b"8=FIX.4.3\x019=000").unwrap(), (FIXVersion::FIX43, 10));
        assert_eq!(parse_tag_8(b"8=FIX.4.4\x019=000").unwrap(), (FIXVersion::FIX44, 10));
        assert_eq!(parse_tag_8(b"8=FIXT.1.1\x019=00").unwrap(), (FIXVersion::FIXT11, 11));

        assert!(parse_tag_8(b"").is_err());
        assert!(parse_tag_8(b"8=").is_err());
        assert!(parse_tag_8(b"8=FIY.4.1\x019=00").is_err());
        assert!(parse_tag_8(b"8=FIXT.4.1\x019=00").is_err());
        assert!(parse_tag_8(b"8=FIX4.1\x019=00").is_err());
        assert!(parse_tag_8(b"8=FIX.4.19=00").is_err());
        assert!(parse_tag_8(b"8=FIX.4.5\x019=00").is_err());
        assert!(parse_tag_8(b"9=FIX.4.1\x019=00").is_err());
        assert!(parse_tag_8(b"88=FIX.4.1\x019=00").is_err());
    }

    #[test]
    fn test_parse_tag_9() {
        assert_eq!(parse_tag_9(b"9=8\x0135=G").unwrap(), (8, 4));
        assert_eq!(parse_tag_9(b"9=2\x0135=G").unwrap(), (2, 4));
        assert_eq!(parse_tag_9(b"9=200\x0135=G").unwrap(), (200, 6));
        assert_eq!(parse_tag_9(b"9=002\x0135=G").unwrap(), (2, 6));
        assert_eq!(parse_tag_9(b"9=90900\x0135=G").unwrap(), (90900, 8));

        assert!(parse_tag_8(b"").is_err());
        assert!(parse_tag_8(b"9=").is_err());
        assert!(parse_tag_8(b"9=a0").is_err());
        assert!(parse_tag_8(b"9=a0\x0135=G").is_err());
        assert!(parse_tag_8(b"8=00\x0135=G").is_err());
        assert!(parse_tag_8(b"9\x0100\x0135=G").is_err());
        assert!(parse_tag_8(b"99=00\x0135=G").is_err());
    }

    #[test]
    fn test_parse_tag_10() {
        assert_eq!(parse_tag_10(b"10=000\x01").unwrap(), 0);
        assert_eq!(parse_tag_10(b"10=002\x01").unwrap(), 2);
        assert_eq!(parse_tag_10(b"10=020\x01").unwrap(), 20);
        assert_eq!(parse_tag_10(b"10=123\x01").unwrap(), 123);
        assert_eq!(parse_tag_10(b"10=255\x01").unwrap(), 255);

        assert!(parse_tag_10(b"").is_err());
        assert!(parse_tag_10(b"10=").is_err());
        assert!(parse_tag_10(b"10=006").is_err());
        assert!(parse_tag_10(b"10=006|").is_err());
        assert!(parse_tag_10(b"10=6\x01").is_err());
        assert!(parse_tag_10(b"10=56\x01").is_err());
        assert!(parse_tag_10(b"10=256\x01").is_err());
        assert!(parse_tag_10(b"10=600\x01").is_err());
        assert!(parse_tag_10(b"10=0011\x01").is_err());
        assert!(parse_tag_10(b"10=aa0\x01").is_err());
        assert!(parse_tag_10(b"11=010\x01").is_err());
    }
}
