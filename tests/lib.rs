extern crate rustfix;
mod messages;

use rustfix::*;
use rustfix::FIXVersion::*;
use messages::*;


#[test]
fn test_preprocess() {
    assert_eq!(preprocess(IEMPTY), Err(PreprocessError::Incomplete));
    assert_eq!(preprocess(ITOO_SHORT), Err(PreprocessError::Incomplete));
    assert_eq!(preprocess(IMISSING_FINAL_SOH), Err(PreprocessError::Invalid));

    assert!(preprocess(FIX41_LOGON).is_ok());
    assert!(preprocess(FIX41_HEARTBEAT).is_ok());
    assert!(preprocess(FIX41_NEWORDERSINGLE_MKT).is_ok());
    assert!(preprocess(FIX41_ER_FILL_NEW).is_ok());
    assert!(preprocess(FIX41_ER_FILL_FILLED).is_ok());
    assert!(preprocess(FIX41_NEWORDERSINGLE_LIMIT).is_ok());
    assert!(preprocess(FIX41_CANCEL).is_ok());
}

