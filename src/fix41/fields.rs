use protocol::FIXValue;
pub use common::AdvSide as AdvSide;
pub use common::AdvTransType as AdvTransType;
pub use common::AllocHandlInst as AllocHandlInst;
pub use common::AllocLinkType as AllocLinkType;
pub use common::CoveredOrUncovered as CoveredOrUncovered;
pub use common::CustomerOrFirm as CustomerOrFirm;
pub use common::EmailType as EmailType;
pub use common::EncryptMethod as EncryptMethod;
pub use common::ExecTransType as ExecTransType;
pub use common::ForexReq as ForexReq;
pub use common::GapFillFlag as GapFillFlag;
pub use common::HandlInst as HandlInst;
pub use common::IOINaturalFlag as IOINaturalFlag;
pub use common::IOIQltyInd as IOIQltyInd;
pub use common::IOIShares as IOIShares;
pub use common::IOITransType as IOITransType;
pub use common::LastCapacity as LastCapacity;
pub use common::LocateReqd as LocateReqd;
pub use common::NotifyBrokerOfCredit as NotifyBrokerOfCredit;
pub use common::PossDupFlag as PossDupFlag;
pub use common::ProcessCode as ProcessCode;
pub use common::PutOrCall as PutOrCall;
pub use common::ReportToExch as ReportToExch;
pub use common::ResetSeqNumFlag as ResetSeqNumFlag;
pub use common::SettlCurrFxRateCalc as SettlCurrFxRateCalc;
pub use common::SettlLocation as SettlLocation;
pub use common::Urgency as Urgency;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocRejCode {
    UnknownAccount = b'0' as isize,
    IncorrectQuantity = b'1' as isize,
    IncorrectAveragePrice = b'2' as isize,
    UnknownExecutingBrokerMnemonic = b'3' as isize,
    CommissionDifference = b'4' as isize,
    UnknownOrderid = b'5' as isize,
    UnknownListid = b'6' as isize,
    Other = b'7' as isize,
}

impl FIXValue for AllocRejCode {
    fn from_bytes(bytes: &[u8]) -> Option<AllocRejCode> {
        match bytes {
            b"0" => Some(AllocRejCode::UnknownAccount),
            b"1" => Some(AllocRejCode::IncorrectQuantity),
            b"2" => Some(AllocRejCode::IncorrectAveragePrice),
            b"3" => Some(AllocRejCode::UnknownExecutingBrokerMnemonic),
            b"4" => Some(AllocRejCode::CommissionDifference),
            b"5" => Some(AllocRejCode::UnknownOrderid),
            b"6" => Some(AllocRejCode::UnknownListid),
            b"7" => Some(AllocRejCode::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocStatus {
    Accepted = b'0' as isize,
    Rejected = b'1' as isize,
    PartialAccept = b'2' as isize,
    Received = b'3' as isize,
}

impl FIXValue for AllocStatus {
    fn from_bytes(bytes: &[u8]) -> Option<AllocStatus> {
        match bytes {
            b"0" => Some(AllocStatus::Accepted),
            b"1" => Some(AllocStatus::Rejected),
            b"2" => Some(AllocStatus::PartialAccept),
            b"3" => Some(AllocStatus::Received),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocTransType {
    New = b'0' as isize,
    Replace = b'1' as isize,
    Cancel = b'2' as isize,
    Preliminary = b'3' as isize,
    Calculated = b'4' as isize,
}

impl FIXValue for AllocTransType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocTransType> {
        match bytes {
            b"0" => Some(AllocTransType::New),
            b"1" => Some(AllocTransType::Replace),
            b"2" => Some(AllocTransType::Cancel),
            b"3" => Some(AllocTransType::Preliminary),
            b"4" => Some(AllocTransType::Calculated),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommType {
    PerShare = b'1' as isize,
    Percentage = b'2' as isize,
    Absolute = b'3' as isize,
}

impl FIXValue for CommType {
    fn from_bytes(bytes: &[u8]) -> Option<CommType> {
        match bytes {
            b"1" => Some(CommType::PerShare),
            b"2" => Some(CommType::Percentage),
            b"3" => Some(CommType::Absolute),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CxlRejReason {
    TooLateToCancel = b'0' as isize,
    UnknownOrder = b'1' as isize,
}

impl FIXValue for CxlRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<CxlRejReason> {
        match bytes {
            b"0" => Some(CxlRejReason::TooLateToCancel),
            b"1" => Some(CxlRejReason::UnknownOrder),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DKReason {
    UnknownSymbol = b'A' as isize,
    WrongSide = b'B' as isize,
    QuantityExceedsOrder = b'C' as isize,
    NoMatchingOrder = b'D' as isize,
    PriceExceedsLimit = b'E' as isize,
    Other = b'Z' as isize,
}

impl FIXValue for DKReason {
    fn from_bytes(bytes: &[u8]) -> Option<DKReason> {
        match bytes {
            b"A" => Some(DKReason::UnknownSymbol),
            b"B" => Some(DKReason::WrongSide),
            b"C" => Some(DKReason::QuantityExceedsOrder),
            b"D" => Some(DKReason::NoMatchingOrder),
            b"E" => Some(DKReason::PriceExceedsLimit),
            b"Z" => Some(DKReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecInst {
    StayOnOfferside = b'0' as isize,
    NotHeld = b'1' as isize,
    Work = b'2' as isize,
    GoAlong = b'3' as isize,
    OverTheDay = b'4' as isize,
    Held = b'5' as isize,
    ParticipateDontInitiate = b'6' as isize,
    StrictScale = b'7' as isize,
    TryToScale = b'8' as isize,
    StayOnBidside = b'9' as isize,
    NoCross = b'A' as isize,
    OkToCross = b'B' as isize,
    CallFirst = b'C' as isize,
    PercentOfVolume = b'D' as isize,
    DoNotIncrease = b'E' as isize,
    DoNotReduce = b'F' as isize,
    AllOrNone = b'G' as isize,
    InstitutionsOnly = b'I' as isize,
    LastPeg = b'L' as isize,
    MidPricePeg = b'M' as isize,
    NonNegotiable = b'N' as isize,
    OpeningPeg = b'O' as isize,
    MarketPeg = b'P' as isize,
    PrimaryPeg = b'R' as isize,
    Suspend = b'S' as isize,
    CustomerDisplayInstruction = b'U' as isize,
    Netting = b'V' as isize,
}

impl FIXValue for ExecInst {
    fn from_bytes(bytes: &[u8]) -> Option<ExecInst> {
        match bytes {
            b"0" => Some(ExecInst::StayOnOfferside),
            b"1" => Some(ExecInst::NotHeld),
            b"2" => Some(ExecInst::Work),
            b"3" => Some(ExecInst::GoAlong),
            b"4" => Some(ExecInst::OverTheDay),
            b"5" => Some(ExecInst::Held),
            b"6" => Some(ExecInst::ParticipateDontInitiate),
            b"7" => Some(ExecInst::StrictScale),
            b"8" => Some(ExecInst::TryToScale),
            b"9" => Some(ExecInst::StayOnBidside),
            b"A" => Some(ExecInst::NoCross),
            b"B" => Some(ExecInst::OkToCross),
            b"C" => Some(ExecInst::CallFirst),
            b"D" => Some(ExecInst::PercentOfVolume),
            b"E" => Some(ExecInst::DoNotIncrease),
            b"F" => Some(ExecInst::DoNotReduce),
            b"G" => Some(ExecInst::AllOrNone),
            b"I" => Some(ExecInst::InstitutionsOnly),
            b"L" => Some(ExecInst::LastPeg),
            b"M" => Some(ExecInst::MidPricePeg),
            b"N" => Some(ExecInst::NonNegotiable),
            b"O" => Some(ExecInst::OpeningPeg),
            b"P" => Some(ExecInst::MarketPeg),
            b"R" => Some(ExecInst::PrimaryPeg),
            b"S" => Some(ExecInst::Suspend),
            b"U" => Some(ExecInst::CustomerDisplayInstruction),
            b"V" => Some(ExecInst::Netting),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecType {
    New = b'0' as isize,
    PartialFill = b'1' as isize,
    Fill = b'2' as isize,
    DoneForDay = b'3' as isize,
    Cancelled = b'4' as isize,
    Replace = b'5' as isize,
    PendingCancelReplace = b'6' as isize,
    Stopped = b'7' as isize,
    Rejected = b'8' as isize,
    Suspended = b'9' as isize,
    PendingNew = b'A' as isize,
    Calculated = b'B' as isize,
    Expired = b'C' as isize,
}

impl FIXValue for ExecType {
    fn from_bytes(bytes: &[u8]) -> Option<ExecType> {
        match bytes {
            b"0" => Some(ExecType::New),
            b"1" => Some(ExecType::PartialFill),
            b"2" => Some(ExecType::Fill),
            b"3" => Some(ExecType::DoneForDay),
            b"4" => Some(ExecType::Cancelled),
            b"5" => Some(ExecType::Replace),
            b"6" => Some(ExecType::PendingCancelReplace),
            b"7" => Some(ExecType::Stopped),
            b"8" => Some(ExecType::Rejected),
            b"9" => Some(ExecType::Suspended),
            b"A" => Some(ExecType::PendingNew),
            b"B" => Some(ExecType::Calculated),
            b"C" => Some(ExecType::Expired),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IDSource {
    Cusip = b'1' as isize,
    Sedol = b'2' as isize,
    Quik = b'3' as isize,
    IsinNumber = b'4' as isize,
    RicCode = b'5' as isize,
    IsoCurrencyCode = b'6' as isize,
    IsoCountryCode = b'7' as isize,
}

impl FIXValue for IDSource {
    fn from_bytes(bytes: &[u8]) -> Option<IDSource> {
        match bytes {
            b"1" => Some(IDSource::Cusip),
            b"2" => Some(IDSource::Sedol),
            b"3" => Some(IDSource::Quik),
            b"4" => Some(IDSource::IsinNumber),
            b"5" => Some(IDSource::RicCode),
            b"6" => Some(IDSource::IsoCurrencyCode),
            b"7" => Some(IDSource::IsoCountryCode),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOIOthSvc {
    Autex = b'A' as isize,
    Bridge = b'B' as isize,
}

impl FIXValue for IOIOthSvc {
    fn from_bytes(bytes: &[u8]) -> Option<IOIOthSvc> {
        match bytes {
            b"A" => Some(IOIOthSvc::Autex),
            b"B" => Some(IOIOthSvc::Bridge),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOIQualifier {
    AllOrNone = b'A' as isize,
    AtTheClose = b'C' as isize,
    InTouchWith = b'I' as isize,
    Limit = b'L' as isize,
    MoreBehind = b'M' as isize,
    AtTheOpen = b'O' as isize,
    TakingAPosition = b'P' as isize,
    AtTheMarket = b'Q' as isize,
    PortfolioShowN = b'S' as isize,
    ThroughTheDay = b'T' as isize,
    Versus = b'V' as isize,
    Indication = b'W' as isize,
    CrossingOpportunity = b'X' as isize,
    AtTheMidpoint = b'Y' as isize,
    PreOpen = b'Z' as isize,
}

impl FIXValue for IOIQualifier {
    fn from_bytes(bytes: &[u8]) -> Option<IOIQualifier> {
        match bytes {
            b"A" => Some(IOIQualifier::AllOrNone),
            b"C" => Some(IOIQualifier::AtTheClose),
            b"I" => Some(IOIQualifier::InTouchWith),
            b"L" => Some(IOIQualifier::Limit),
            b"M" => Some(IOIQualifier::MoreBehind),
            b"O" => Some(IOIQualifier::AtTheOpen),
            b"P" => Some(IOIQualifier::TakingAPosition),
            b"Q" => Some(IOIQualifier::AtTheMarket),
            b"S" => Some(IOIQualifier::PortfolioShowN),
            b"T" => Some(IOIQualifier::ThroughTheDay),
            b"V" => Some(IOIQualifier::Versus),
            b"W" => Some(IOIQualifier::Indication),
            b"X" => Some(IOIQualifier::CrossingOpportunity),
            b"Y" => Some(IOIQualifier::AtTheMidpoint),
            b"Z" => Some(IOIQualifier::PreOpen),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiscFeeType {
    Regulatory = b'1' as isize,
    Tax = b'2' as isize,
    LocalCommission = b'3' as isize,
    ExchangeFees = b'4' as isize,
    Stamp = b'5' as isize,
    Levy = b'6' as isize,
    Other = b'7' as isize,
    Markup = b'8' as isize,
}

impl FIXValue for MiscFeeType {
    fn from_bytes(bytes: &[u8]) -> Option<MiscFeeType> {
        match bytes {
            b"1" => Some(MiscFeeType::Regulatory),
            b"2" => Some(MiscFeeType::Tax),
            b"3" => Some(MiscFeeType::LocalCommission),
            b"4" => Some(MiscFeeType::ExchangeFees),
            b"5" => Some(MiscFeeType::Stamp),
            b"6" => Some(MiscFeeType::Levy),
            b"7" => Some(MiscFeeType::Other),
            b"8" => Some(MiscFeeType::Markup),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgType {
    Heartbeat = b'0' as isize,
    TestRequest = b'1' as isize,
    ResendRequest = b'2' as isize,
    Reject = b'3' as isize,
    SequenceReset = b'4' as isize,
    Logout = b'5' as isize,
    IndicationOfInterest = b'6' as isize,
    Advertisement = b'7' as isize,
    ExecutionReport = b'8' as isize,
    OrderCancelReject = b'9' as isize,
    Logon = b'A' as isize,
    News = b'B' as isize,
    Email = b'C' as isize,
    OrderD = b'D' as isize,
    OrderE = b'E' as isize,
    OrderCancelRequest = b'F' as isize,
    OrderCancelReplaceRequest = b'G' as isize,
    OrderStatusRequest = b'H' as isize,
    Allocation = b'J' as isize,
    ListCancelRequest = b'K' as isize,
    ListExecute = b'L' as isize,
    ListStatusRequest = b'M' as isize,
    ListStatus = b'N' as isize,
    AllocationAck = b'P' as isize,
    DontKnowTrade = b'Q' as isize,
    QuoteRequest = b'R' as isize,
    Quote = b'S' as isize,
    SettlementInstructions = b'T' as isize,
}

impl FIXValue for MsgType {
    fn from_bytes(bytes: &[u8]) -> Option<MsgType> {
        match bytes {
            b"0" => Some(MsgType::Heartbeat),
            b"1" => Some(MsgType::TestRequest),
            b"2" => Some(MsgType::ResendRequest),
            b"3" => Some(MsgType::Reject),
            b"4" => Some(MsgType::SequenceReset),
            b"5" => Some(MsgType::Logout),
            b"6" => Some(MsgType::IndicationOfInterest),
            b"7" => Some(MsgType::Advertisement),
            b"8" => Some(MsgType::ExecutionReport),
            b"9" => Some(MsgType::OrderCancelReject),
            b"A" => Some(MsgType::Logon),
            b"B" => Some(MsgType::News),
            b"C" => Some(MsgType::Email),
            b"D" => Some(MsgType::OrderD),
            b"E" => Some(MsgType::OrderE),
            b"F" => Some(MsgType::OrderCancelRequest),
            b"G" => Some(MsgType::OrderCancelReplaceRequest),
            b"H" => Some(MsgType::OrderStatusRequest),
            b"J" => Some(MsgType::Allocation),
            b"K" => Some(MsgType::ListCancelRequest),
            b"L" => Some(MsgType::ListExecute),
            b"M" => Some(MsgType::ListStatusRequest),
            b"N" => Some(MsgType::ListStatus),
            b"P" => Some(MsgType::AllocationAck),
            b"Q" => Some(MsgType::DontKnowTrade),
            b"R" => Some(MsgType::QuoteRequest),
            b"S" => Some(MsgType::Quote),
            b"T" => Some(MsgType::SettlementInstructions),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenClose {
    Close = b'C' as isize,
    Open = b'O' as isize,
}

impl FIXValue for OpenClose {
    fn from_bytes(bytes: &[u8]) -> Option<OpenClose> {
        match bytes {
            b"C" => Some(OpenClose::Close),
            b"O" => Some(OpenClose::Open),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdRejReason {
    BrokerOption = b'0' as isize,
    UnknownSymbol = b'1' as isize,
    ExchangeClosed = b'2' as isize,
    OrderExceedsLimit = b'3' as isize,
    TooLateToEnter = b'4' as isize,
    UnknownOrder = b'5' as isize,
    DuplicateOrder = b'6' as isize,
}

impl FIXValue for OrdRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<OrdRejReason> {
        match bytes {
            b"0" => Some(OrdRejReason::BrokerOption),
            b"1" => Some(OrdRejReason::UnknownSymbol),
            b"2" => Some(OrdRejReason::ExchangeClosed),
            b"3" => Some(OrdRejReason::OrderExceedsLimit),
            b"4" => Some(OrdRejReason::TooLateToEnter),
            b"5" => Some(OrdRejReason::UnknownOrder),
            b"6" => Some(OrdRejReason::DuplicateOrder),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdStatus {
    New = b'0' as isize,
    PartiallyFilled = b'1' as isize,
    Filled = b'2' as isize,
    DoneForDay = b'3' as isize,
    Canceled = b'4' as isize,
    Replaced = b'5' as isize,
    PendingCancelReplace = b'6' as isize,
    Stopped = b'7' as isize,
    Rejected = b'8' as isize,
    Suspended = b'9' as isize,
    PendingNew = b'A' as isize,
    Calculated = b'B' as isize,
    Expired = b'C' as isize,
}

impl FIXValue for OrdStatus {
    fn from_bytes(bytes: &[u8]) -> Option<OrdStatus> {
        match bytes {
            b"0" => Some(OrdStatus::New),
            b"1" => Some(OrdStatus::PartiallyFilled),
            b"2" => Some(OrdStatus::Filled),
            b"3" => Some(OrdStatus::DoneForDay),
            b"4" => Some(OrdStatus::Canceled),
            b"5" => Some(OrdStatus::Replaced),
            b"6" => Some(OrdStatus::PendingCancelReplace),
            b"7" => Some(OrdStatus::Stopped),
            b"8" => Some(OrdStatus::Rejected),
            b"9" => Some(OrdStatus::Suspended),
            b"A" => Some(OrdStatus::PendingNew),
            b"B" => Some(OrdStatus::Calculated),
            b"C" => Some(OrdStatus::Expired),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdType {
    Market = b'1' as isize,
    Limit = b'2' as isize,
    Stop = b'3' as isize,
    StopLimit = b'4' as isize,
    MarketOnClose = b'5' as isize,
    WithOrWithout = b'6' as isize,
    LimitOrBetter = b'7' as isize,
    LimitWithOrWithout = b'8' as isize,
    OnBasis = b'9' as isize,
    OnClose = b'A' as isize,
    LimitOnClose = b'B' as isize,
    ForexC = b'C' as isize,
    PreviouslyQuoted = b'D' as isize,
    PreviouslyIndicated = b'E' as isize,
    ForexF = b'F' as isize,
    ForexG = b'G' as isize,
    ForexH = b'H' as isize,
    Pegged = b'P' as isize,
}

impl FIXValue for OrdType {
    fn from_bytes(bytes: &[u8]) -> Option<OrdType> {
        match bytes {
            b"1" => Some(OrdType::Market),
            b"2" => Some(OrdType::Limit),
            b"3" => Some(OrdType::Stop),
            b"4" => Some(OrdType::StopLimit),
            b"5" => Some(OrdType::MarketOnClose),
            b"6" => Some(OrdType::WithOrWithout),
            b"7" => Some(OrdType::LimitOrBetter),
            b"8" => Some(OrdType::LimitWithOrWithout),
            b"9" => Some(OrdType::OnBasis),
            b"A" => Some(OrdType::OnClose),
            b"B" => Some(OrdType::LimitOnClose),
            b"C" => Some(OrdType::ForexC),
            b"D" => Some(OrdType::PreviouslyQuoted),
            b"E" => Some(OrdType::PreviouslyIndicated),
            b"F" => Some(OrdType::ForexF),
            b"G" => Some(OrdType::ForexG),
            b"H" => Some(OrdType::ForexH),
            b"P" => Some(OrdType::Pegged),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PossResend {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for PossResend {
    fn from_bytes(bytes: &[u8]) -> Option<PossResend> {
        match bytes {
            b"N" => Some(PossResend::No),
            b"Y" => Some(PossResend::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule80A {
    AgencySingleOrder = b'A' as isize,
    ShortExemptTransactionB = b'B' as isize,
    ProgramOrderNonIndexArbForMemberFirmOrg = b'C' as isize,
    ProgramOrderIndexArbForMemberFirmOrg = b'D' as isize,
    RegisteredEquityMarketMakerTrades = b'E' as isize,
    ShortExemptTransactionF = b'F' as isize,
    ShortExemptTransactionH = b'H' as isize,
    IndividualInvestorSingleOrder = b'I' as isize,
    ProgramOrderIndexArbForIndividualCustomer = b'J' as isize,
    ProgramOrderNonIndexArbForIndividualCustomer = b'K' as isize,
    ShortExemptTransactionForMemberCompetingMarketMakerAffiliatedWithTheFirmClearingTheTrade = b'L' as isize,
    ProgramOrderIndexArbForOtherMember = b'M' as isize,
    ProgramOrderNonIndexArbForOtherMember = b'N' as isize,
    CompetingDealerTradesO = b'O' as isize,
    Principal = b'P' as isize,
    CompetingDealerTradesR = b'R' as isize,
    SpecialistTrades = b'S' as isize,
    CompetingDealerTradesT = b'T' as isize,
    ProgramOrderIndexArbForOtherAgency = b'U' as isize,
    AllOtherOrdersAsAgentForOtherMember = b'W' as isize,
    ShortExemptTransactionForMemberCompetingMarketMakerNotAffiliatedWithTheFirmClearingTheTrade = b'X' as isize,
    ProgramOrderNonIndexArbForOtherAgency = b'Y' as isize,
    ShortExemptTransactionForNonMemberCompetingMarketMaker = b'Z' as isize,
}

impl FIXValue for Rule80A {
    fn from_bytes(bytes: &[u8]) -> Option<Rule80A> {
        match bytes {
            b"A" => Some(Rule80A::AgencySingleOrder),
            b"B" => Some(Rule80A::ShortExemptTransactionB),
            b"C" => Some(Rule80A::ProgramOrderNonIndexArbForMemberFirmOrg),
            b"D" => Some(Rule80A::ProgramOrderIndexArbForMemberFirmOrg),
            b"E" => Some(Rule80A::RegisteredEquityMarketMakerTrades),
            b"F" => Some(Rule80A::ShortExemptTransactionF),
            b"H" => Some(Rule80A::ShortExemptTransactionH),
            b"I" => Some(Rule80A::IndividualInvestorSingleOrder),
            b"J" => Some(Rule80A::ProgramOrderIndexArbForIndividualCustomer),
            b"K" => Some(Rule80A::ProgramOrderNonIndexArbForIndividualCustomer),
            b"L" => Some(Rule80A::ShortExemptTransactionForMemberCompetingMarketMakerAffiliatedWithTheFirmClearingTheTrade),
            b"M" => Some(Rule80A::ProgramOrderIndexArbForOtherMember),
            b"N" => Some(Rule80A::ProgramOrderNonIndexArbForOtherMember),
            b"O" => Some(Rule80A::CompetingDealerTradesO),
            b"P" => Some(Rule80A::Principal),
            b"R" => Some(Rule80A::CompetingDealerTradesR),
            b"S" => Some(Rule80A::SpecialistTrades),
            b"T" => Some(Rule80A::CompetingDealerTradesT),
            b"U" => Some(Rule80A::ProgramOrderIndexArbForOtherAgency),
            b"W" => Some(Rule80A::AllOtherOrdersAsAgentForOtherMember),
            b"X" => Some(Rule80A::ShortExemptTransactionForMemberCompetingMarketMakerNotAffiliatedWithTheFirmClearingTheTrade),
            b"Y" => Some(Rule80A::ProgramOrderNonIndexArbForOtherAgency),
            b"Z" => Some(Rule80A::ShortExemptTransactionForNonMemberCompetingMarketMaker),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityType {
    BankersAcceptance,
    CertificateOfDeposit,
    CollateralizeMortgageObligation,
    CorporateBond,
    CommercialPaper,
    CorporatePrivatePlacement,
    CommonStock,
    FederalHousingAuthority,
    FederalHomeLoan,
    FederalNationalMortgageAssociation,
    ForeignExchangeContract,
    Future,
    GovernmentNationalMortgageAssociation,
    TreasuriesPlusAgencyDebenture,
    MutualFund,
    MortgageInterestOnly,
    MortgagePrincipleOnly,
    MortgagePrivatePlacement,
    MiscellaneousPassThru,
    MunicipalBond,
    NoIsitcSecurityType,
    Option,
    PreferredStock,
    RepurchaseAgreement,
    ReverseRepurchaseAgreement,
    StudentLoanMarketingAssociation,
    TimeDeposit,
    UsTreasuryBill,
    Warrant,
    CatsTigersLions,
}

impl FIXValue for SecurityType {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityType> {
        match bytes {
            b"BA" => Some(SecurityType::BankersAcceptance),
            b"CD" => Some(SecurityType::CertificateOfDeposit),
            b"CMO" => Some(SecurityType::CollateralizeMortgageObligation),
            b"CORP" => Some(SecurityType::CorporateBond),
            b"CP" => Some(SecurityType::CommercialPaper),
            b"CPP" => Some(SecurityType::CorporatePrivatePlacement),
            b"CS" => Some(SecurityType::CommonStock),
            b"FHA" => Some(SecurityType::FederalHousingAuthority),
            b"FHL" => Some(SecurityType::FederalHomeLoan),
            b"FN" => Some(SecurityType::FederalNationalMortgageAssociation),
            b"FOR" => Some(SecurityType::ForeignExchangeContract),
            b"FUT" => Some(SecurityType::Future),
            b"GN" => Some(SecurityType::GovernmentNationalMortgageAssociation),
            b"GOVT" => Some(SecurityType::TreasuriesPlusAgencyDebenture),
            b"MF" => Some(SecurityType::MutualFund),
            b"MIO" => Some(SecurityType::MortgageInterestOnly),
            b"MPO" => Some(SecurityType::MortgagePrincipleOnly),
            b"MPP" => Some(SecurityType::MortgagePrivatePlacement),
            b"MPT" => Some(SecurityType::MiscellaneousPassThru),
            b"MUNI" => Some(SecurityType::MunicipalBond),
            b"NONE" => Some(SecurityType::NoIsitcSecurityType),
            b"OPT" => Some(SecurityType::Option),
            b"PS" => Some(SecurityType::PreferredStock),
            b"RP" => Some(SecurityType::RepurchaseAgreement),
            b"RVRP" => Some(SecurityType::ReverseRepurchaseAgreement),
            b"SL" => Some(SecurityType::StudentLoanMarketingAssociation),
            b"TD" => Some(SecurityType::TimeDeposit),
            b"USTB" => Some(SecurityType::UsTreasuryBill),
            b"WAR" => Some(SecurityType::Warrant),
            b"ZOO" => Some(SecurityType::CatsTigersLions),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            SecurityType::BankersAcceptance => v.extend(b"BA"),
            SecurityType::CertificateOfDeposit => v.extend(b"CD"),
            SecurityType::CollateralizeMortgageObligation => v.extend(b"CMO"),
            SecurityType::CorporateBond => v.extend(b"CORP"),
            SecurityType::CommercialPaper => v.extend(b"CP"),
            SecurityType::CorporatePrivatePlacement => v.extend(b"CPP"),
            SecurityType::CommonStock => v.extend(b"CS"),
            SecurityType::FederalHousingAuthority => v.extend(b"FHA"),
            SecurityType::FederalHomeLoan => v.extend(b"FHL"),
            SecurityType::FederalNationalMortgageAssociation => v.extend(b"FN"),
            SecurityType::ForeignExchangeContract => v.extend(b"FOR"),
            SecurityType::Future => v.extend(b"FUT"),
            SecurityType::GovernmentNationalMortgageAssociation => v.extend(b"GN"),
            SecurityType::TreasuriesPlusAgencyDebenture => v.extend(b"GOVT"),
            SecurityType::MutualFund => v.extend(b"MF"),
            SecurityType::MortgageInterestOnly => v.extend(b"MIO"),
            SecurityType::MortgagePrincipleOnly => v.extend(b"MPO"),
            SecurityType::MortgagePrivatePlacement => v.extend(b"MPP"),
            SecurityType::MiscellaneousPassThru => v.extend(b"MPT"),
            SecurityType::MunicipalBond => v.extend(b"MUNI"),
            SecurityType::NoIsitcSecurityType => v.extend(b"NONE"),
            SecurityType::Option => v.extend(b"OPT"),
            SecurityType::PreferredStock => v.extend(b"PS"),
            SecurityType::RepurchaseAgreement => v.extend(b"RP"),
            SecurityType::ReverseRepurchaseAgreement => v.extend(b"RVRP"),
            SecurityType::StudentLoanMarketingAssociation => v.extend(b"SL"),
            SecurityType::TimeDeposit => v.extend(b"TD"),
            SecurityType::UsTreasuryBill => v.extend(b"USTB"),
            SecurityType::Warrant => v.extend(b"WAR"),
            SecurityType::CatsTigersLions => v.extend(b"ZOO"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstMode {
    Default = b'0' as isize,
    StandingInstructionsProvided = b'1' as isize,
    SpecificAllocationAccountOverriding = b'2' as isize,
    SpecificAllocationAccountStanding = b'3' as isize,
}

impl FIXValue for SettlInstMode {
    fn from_bytes(bytes: &[u8]) -> Option<SettlInstMode> {
        match bytes {
            b"0" => Some(SettlInstMode::Default),
            b"1" => Some(SettlInstMode::StandingInstructionsProvided),
            b"2" => Some(SettlInstMode::SpecificAllocationAccountOverriding),
            b"3" => Some(SettlInstMode::SpecificAllocationAccountStanding),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstSource {
    BrokersInstructions = b'1' as isize,
    InstitutionsInstructions = b'2' as isize,
}

impl FIXValue for SettlInstSource {
    fn from_bytes(bytes: &[u8]) -> Option<SettlInstSource> {
        match bytes {
            b"1" => Some(SettlInstSource::BrokersInstructions),
            b"2" => Some(SettlInstSource::InstitutionsInstructions),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstTransType {
    Cancel = b'C' as isize,
    New = b'N' as isize,
    Replace = b'R' as isize,
}

impl FIXValue for SettlInstTransType {
    fn from_bytes(bytes: &[u8]) -> Option<SettlInstTransType> {
        match bytes {
            b"C" => Some(SettlInstTransType::Cancel),
            b"N" => Some(SettlInstTransType::New),
            b"R" => Some(SettlInstTransType::Replace),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlmntTyp {
    Regular = b'0' as isize,
    Cash = b'1' as isize,
    NextDay = b'2' as isize,
    TPlus2 = b'3' as isize,
    TPlus3 = b'4' as isize,
    TPlus4 = b'5' as isize,
    Future = b'6' as isize,
    WhenIssued = b'7' as isize,
    SellersOption = b'8' as isize,
    TPlus5 = b'9' as isize,
}

impl FIXValue for SettlmntTyp {
    fn from_bytes(bytes: &[u8]) -> Option<SettlmntTyp> {
        match bytes {
            b"0" => Some(SettlmntTyp::Regular),
            b"1" => Some(SettlmntTyp::Cash),
            b"2" => Some(SettlmntTyp::NextDay),
            b"3" => Some(SettlmntTyp::TPlus2),
            b"4" => Some(SettlmntTyp::TPlus3),
            b"5" => Some(SettlmntTyp::TPlus4),
            b"6" => Some(SettlmntTyp::Future),
            b"7" => Some(SettlmntTyp::WhenIssued),
            b"8" => Some(SettlmntTyp::SellersOption),
            b"9" => Some(SettlmntTyp::TPlus5),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy = b'1' as isize,
    Sell = b'2' as isize,
    BuyMinus = b'3' as isize,
    SellPlus = b'4' as isize,
    SellShort = b'5' as isize,
    SellShortExempt = b'6' as isize,
    Undisclosed = b'7' as isize,
    Cross = b'8' as isize,
}

impl FIXValue for Side {
    fn from_bytes(bytes: &[u8]) -> Option<Side> {
        match bytes {
            b"1" => Some(Side::Buy),
            b"2" => Some(Side::Sell),
            b"3" => Some(Side::BuyMinus),
            b"4" => Some(Side::SellPlus),
            b"5" => Some(Side::SellShort),
            b"6" => Some(Side::SellShortExempt),
            b"7" => Some(Side::Undisclosed),
            b"8" => Some(Side::Cross),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandInstDbType {
    Other = b'0' as isize,
    DtcSid = b'1' as isize,
    ThomsonAlert = b'2' as isize,
    AGlobalCustodian = b'3' as isize,
}

impl FIXValue for StandInstDbType {
    fn from_bytes(bytes: &[u8]) -> Option<StandInstDbType> {
        match bytes {
            b"0" => Some(StandInstDbType::Other),
            b"1" => Some(StandInstDbType::DtcSid),
            b"2" => Some(StandInstDbType::ThomsonAlert),
            b"3" => Some(StandInstDbType::AGlobalCustodian),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    Day = b'0' as isize,
    GoodTillCancel = b'1' as isize,
    AtTheOpening = b'2' as isize,
    ImmediateOrCancel = b'3' as isize,
    FillOrKill = b'4' as isize,
    GoodTillCrossing = b'5' as isize,
    GoodTillDate = b'6' as isize,
}

impl FIXValue for TimeInForce {
    fn from_bytes(bytes: &[u8]) -> Option<TimeInForce> {
        match bytes {
            b"0" => Some(TimeInForce::Day),
            b"1" => Some(TimeInForce::GoodTillCancel),
            b"2" => Some(TimeInForce::AtTheOpening),
            b"3" => Some(TimeInForce::ImmediateOrCancel),
            b"4" => Some(TimeInForce::FillOrKill),
            b"5" => Some(TimeInForce::GoodTillCrossing),
            b"6" => Some(TimeInForce::GoodTillDate),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

