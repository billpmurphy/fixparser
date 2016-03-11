use protocol::FIXValue;
pub use common::Adjustment as Adjustment;
pub use common::AdvSide as AdvSide;
pub use common::AdvTransType as AdvTransType;
pub use common::AggregatedBook as AggregatedBook;
pub use common::AllocHandlInst as AllocHandlInst;
pub use common::AllocLinkType as AllocLinkType;
pub use common::BasisPxType as BasisPxType;
pub use common::Benchmark as Benchmark;
pub use common::CorporateAction as CorporateAction;
pub use common::CoveredOrUncovered as CoveredOrUncovered;
pub use common::CustomerOrFirm as CustomerOrFirm;
pub use common::CxlRejResponseTo as CxlRejResponseTo;
pub use common::DeleteReason as DeleteReason;
pub use common::DueToRelated as DueToRelated;
pub use common::EmailType as EmailType;
pub use common::EncryptMethod as EncryptMethod;
pub use common::ExchangeForPhysical as ExchangeForPhysical;
pub use common::ExecTransType as ExecTransType;
pub use common::ForexReq as ForexReq;
pub use common::GTBookingInst as GTBookingInst;
pub use common::GapFillFlag as GapFillFlag;
pub use common::HaltReasonChar as HaltReasonChar;
pub use common::HandlInst as HandlInst;
pub use common::IOINaturalFlag as IOINaturalFlag;
pub use common::IOIQltyInd as IOIQltyInd;
pub use common::IOIShares as IOIShares;
pub use common::IOITransType as IOITransType;
pub use common::InViewOfCommon as InViewOfCommon;
pub use common::IncTaxInd as IncTaxInd;
pub use common::LastCapacity as LastCapacity;
pub use common::LocateReqd as LocateReqd;
pub use common::MDUpdateAction as MDUpdateAction;
pub use common::MDUpdateType as MDUpdateType;
pub use common::MsgDirection as MsgDirection;
pub use common::MultiLegReportingType as MultiLegReportingType;
pub use common::NetGrossInd as NetGrossInd;
pub use common::NotifyBrokerOfCredit as NotifyBrokerOfCredit;
pub use common::PossDupFlag as PossDupFlag;
pub use common::ProcessCode as ProcessCode;
pub use common::ProgRptReqs as ProgRptReqs;
pub use common::PutOrCall as PutOrCall;
pub use common::QuoteCondition as QuoteCondition;
pub use common::QuoteRequestType as QuoteRequestType;
pub use common::QuoteResponseLevel as QuoteResponseLevel;
pub use common::ReportToExch as ReportToExch;
pub use common::ResetSeqNumFlag as ResetSeqNumFlag;
pub use common::RoutingType as RoutingType;
pub use common::SecurityRequestType as SecurityRequestType;
pub use common::SettlCurrFxRateCalc as SettlCurrFxRateCalc;
pub use common::SettlLocation as SettlLocation;
pub use common::SolicitedFlag as SolicitedFlag;
pub use common::SubscriptionRequestType as SubscriptionRequestType;
pub use common::TickDirection as TickDirection;
pub use common::TradSesMethod as TradSesMethod;
pub use common::TradSesMode as TradSesMode;
pub use common::TradeType as TradeType;
pub use common::UnsolicitedIndicator as UnsolicitedIndicator;
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
    CalculatedWithoutPreliminary = b'5' as isize,
}

impl FIXValue for AllocTransType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocTransType> {
        match bytes {
            b"0" => Some(AllocTransType::New),
            b"1" => Some(AllocTransType::Replace),
            b"2" => Some(AllocTransType::Cancel),
            b"3" => Some(AllocTransType::Preliminary),
            b"4" => Some(AllocTransType::Calculated),
            b"5" => Some(AllocTransType::CalculatedWithoutPreliminary),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidRequestTransType {
    Cancel = b'C' as isize,
    No = b'N' as isize,
}

impl FIXValue for BidRequestTransType {
    fn from_bytes(bytes: &[u8]) -> Option<BidRequestTransType> {
        match bytes {
            b"C" => Some(BidRequestTransType::Cancel),
            b"N" => Some(BidRequestTransType::No),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusinessRejectReason {
    Other = b'0' as isize,
    UnkownId = b'1' as isize,
    UnknownSecurity = b'2' as isize,
    UnsupportedMessageType = b'3' as isize,
    ApplicationNotAvailable = b'4' as isize,
    ConditionallyRequiredFieldMissing = b'5' as isize,
}

impl FIXValue for BusinessRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<BusinessRejectReason> {
        match bytes {
            b"0" => Some(BusinessRejectReason::Other),
            b"1" => Some(BusinessRejectReason::UnkownId),
            b"2" => Some(BusinessRejectReason::UnknownSecurity),
            b"3" => Some(BusinessRejectReason::UnsupportedMessageType),
            b"4" => Some(BusinessRejectReason::ApplicationNotAvailable),
            b"5" => Some(BusinessRejectReason::ConditionallyRequiredFieldMissing),
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
    BrokerOption = b'2' as isize,
    OrderAlreadyInPendingCancelOrPendingReplaceStatus = b'3' as isize,
}

impl FIXValue for CxlRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<CxlRejReason> {
        match bytes {
            b"0" => Some(CxlRejReason::TooLateToCancel),
            b"1" => Some(CxlRejReason::UnknownOrder),
            b"2" => Some(CxlRejReason::BrokerOption),
            b"3" => Some(CxlRejReason::OrderAlreadyInPendingCancelOrPendingReplaceStatus),
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
pub enum DiscretionInst {
    RelatedToDisplayedPrice = b'0' as isize,
    RelatedToMarketPrice = b'1' as isize,
    RelatedToPrimaryPrice = b'2' as isize,
    RelatedToLocalPrimaryPrice = b'3' as isize,
    RelatedToMidpointPrice = b'4' as isize,
    RelatedToLastTradePrice = b'5' as isize,
}

impl FIXValue for DiscretionInst {
    fn from_bytes(bytes: &[u8]) -> Option<DiscretionInst> {
        match bytes {
            b"0" => Some(DiscretionInst::RelatedToDisplayedPrice),
            b"1" => Some(DiscretionInst::RelatedToMarketPrice),
            b"2" => Some(DiscretionInst::RelatedToPrimaryPrice),
            b"3" => Some(DiscretionInst::RelatedToLocalPrimaryPrice),
            b"4" => Some(DiscretionInst::RelatedToMidpointPrice),
            b"5" => Some(DiscretionInst::RelatedToLastTradePrice),
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
    FixedPegToLocalBestBidOrOfferAtTimeOfOrder = b'T' as isize,
    CustomerDisplayInstruction = b'U' as isize,
    Netting = b'V' as isize,
    PegToVwap = b'W' as isize,
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
            b"T" => Some(ExecInst::FixedPegToLocalBestBidOrOfferAtTimeOfOrder),
            b"U" => Some(ExecInst::CustomerDisplayInstruction),
            b"V" => Some(ExecInst::Netting),
            b"W" => Some(ExecInst::PegToVwap),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecRestatementReason {
    GtCorporateAction = b'0' as isize,
    GtRenewal = b'1' as isize,
    VerbalChange = b'2' as isize,
    RepricingOfOrder = b'3' as isize,
    BrokerOption = b'4' as isize,
    PartialDeclineOfOrderqty = b'5' as isize,
}

impl FIXValue for ExecRestatementReason {
    fn from_bytes(bytes: &[u8]) -> Option<ExecRestatementReason> {
        match bytes {
            b"0" => Some(ExecRestatementReason::GtCorporateAction),
            b"1" => Some(ExecRestatementReason::GtRenewal),
            b"2" => Some(ExecRestatementReason::VerbalChange),
            b"3" => Some(ExecRestatementReason::RepricingOfOrder),
            b"4" => Some(ExecRestatementReason::BrokerOption),
            b"5" => Some(ExecRestatementReason::PartialDeclineOfOrderqty),
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
    Canceled = b'4' as isize,
    Replace = b'5' as isize,
    PendingCancel = b'6' as isize,
    Stopped = b'7' as isize,
    Rejected = b'8' as isize,
    Suspended = b'9' as isize,
    PendingNew = b'A' as isize,
    Calculated = b'B' as isize,
    Expired = b'C' as isize,
    Restated = b'D' as isize,
    PendingReplace = b'E' as isize,
}

impl FIXValue for ExecType {
    fn from_bytes(bytes: &[u8]) -> Option<ExecType> {
        match bytes {
            b"0" => Some(ExecType::New),
            b"1" => Some(ExecType::PartialFill),
            b"2" => Some(ExecType::Fill),
            b"3" => Some(ExecType::DoneForDay),
            b"4" => Some(ExecType::Canceled),
            b"5" => Some(ExecType::Replace),
            b"6" => Some(ExecType::PendingCancel),
            b"7" => Some(ExecType::Stopped),
            b"8" => Some(ExecType::Rejected),
            b"9" => Some(ExecType::Suspended),
            b"A" => Some(ExecType::PendingNew),
            b"B" => Some(ExecType::Calculated),
            b"C" => Some(ExecType::Expired),
            b"D" => Some(ExecType::Restated),
            b"E" => Some(ExecType::PendingReplace),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinancialStatus {
    Bankrupt = b'1' as isize,
}

impl FIXValue for FinancialStatus {
    fn from_bytes(bytes: &[u8]) -> Option<FinancialStatus> {
        match bytes {
            b"1" => Some(FinancialStatus::Bankrupt),
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
    ExchangeSymbol = b'8' as isize,
    ConsolidatedTapeAssociation = b'9' as isize,
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
            b"8" => Some(IDSource::ExchangeSymbol),
            b"9" => Some(IDSource::ConsolidatedTapeAssociation),
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
    ReadyToTrade = b'R' as isize,
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
            b"R" => Some(IOIQualifier::ReadyToTrade),
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
pub enum LiquidityIndType {
    FiveDayMovingAverage = b'1' as isize,
    TwentyDayMovingAverage = b'2' as isize,
    NormalMarketSize = b'3' as isize,
    Other = b'4' as isize,
}

impl FIXValue for LiquidityIndType {
    fn from_bytes(bytes: &[u8]) -> Option<LiquidityIndType> {
        match bytes {
            b"1" => Some(LiquidityIndType::FiveDayMovingAverage),
            b"2" => Some(LiquidityIndType::TwentyDayMovingAverage),
            b"3" => Some(LiquidityIndType::NormalMarketSize),
            b"4" => Some(LiquidityIndType::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListExecInstType {
    Immediate = b'1' as isize,
    WaitForExecuteInstruction = b'2' as isize,
}

impl FIXValue for ListExecInstType {
    fn from_bytes(bytes: &[u8]) -> Option<ListExecInstType> {
        match bytes {
            b"1" => Some(ListExecInstType::Immediate),
            b"2" => Some(ListExecInstType::WaitForExecuteInstruction),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MDEntryType {
    Bid = b'0' as isize,
    Offer = b'1' as isize,
    Trade = b'2' as isize,
    IndexValue = b'3' as isize,
    OpeningPrice = b'4' as isize,
    ClosingPrice = b'5' as isize,
    SettlementPrice = b'6' as isize,
    TradingSessionHighPrice = b'7' as isize,
    TradingSessionLowPrice = b'8' as isize,
    TradingSessionVwapPrice = b'9' as isize,
}

impl FIXValue for MDEntryType {
    fn from_bytes(bytes: &[u8]) -> Option<MDEntryType> {
        match bytes {
            b"0" => Some(MDEntryType::Bid),
            b"1" => Some(MDEntryType::Offer),
            b"2" => Some(MDEntryType::Trade),
            b"3" => Some(MDEntryType::IndexValue),
            b"4" => Some(MDEntryType::OpeningPrice),
            b"5" => Some(MDEntryType::ClosingPrice),
            b"6" => Some(MDEntryType::SettlementPrice),
            b"7" => Some(MDEntryType::TradingSessionHighPrice),
            b"8" => Some(MDEntryType::TradingSessionLowPrice),
            b"9" => Some(MDEntryType::TradingSessionVwapPrice),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MDReqRejReason {
    UnknownSymbol = b'0' as isize,
    DuplicateMdreqid = b'1' as isize,
    InsufficientBandwidth = b'2' as isize,
    InsufficientPermissions = b'3' as isize,
    UnsupportedSubscriptionrequesttype = b'4' as isize,
    UnsupportedMarketdepth = b'5' as isize,
    UnsupportedMdupdatetype = b'6' as isize,
    UnsupportedAggregatedbook = b'7' as isize,
    UnsupportedMdentrytype = b'8' as isize,
}

impl FIXValue for MDReqRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<MDReqRejReason> {
        match bytes {
            b"0" => Some(MDReqRejReason::UnknownSymbol),
            b"1" => Some(MDReqRejReason::DuplicateMdreqid),
            b"2" => Some(MDReqRejReason::InsufficientBandwidth),
            b"3" => Some(MDReqRejReason::InsufficientPermissions),
            b"4" => Some(MDReqRejReason::UnsupportedSubscriptionrequesttype),
            b"5" => Some(MDReqRejReason::UnsupportedMarketdepth),
            b"6" => Some(MDReqRejReason::UnsupportedMdupdatetype),
            b"7" => Some(MDReqRejReason::UnsupportedAggregatedbook),
            b"8" => Some(MDReqRejReason::UnsupportedMdentrytype),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageEncoding {
    EucJp,
    Iso2022Jp,
    ShiftJis,
    Utf8,
}

impl FIXValue for MessageEncoding {
    fn from_bytes(bytes: &[u8]) -> Option<MessageEncoding> {
        match bytes {
            b"EUC-JP" => Some(MessageEncoding::EucJp),
            b"ISO-2022-JP" => Some(MessageEncoding::Iso2022Jp),
            b"SHIFT_JIS" => Some(MessageEncoding::ShiftJis),
            b"UTF-8" => Some(MessageEncoding::Utf8),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            MessageEncoding::EucJp => v.extend(b"EUC-JP"),
            MessageEncoding::Iso2022Jp => v.extend(b"ISO-2022-JP"),
            MessageEncoding::ShiftJis => v.extend(b"SHIFT_JIS"),
            MessageEncoding::Utf8 => v.extend(b"UTF-8"),
        }
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
    ConsumptionTax = b'9' as isize,
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
            b"9" => Some(MiscFeeType::ConsumptionTax),
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
    QuoteStatusRequest = b'a' as isize,
    Logon = b'A' as isize,
    News = b'B' as isize,
    QuoteAcknowledgement = b'b' as isize,
    Email = b'C' as isize,
    SecurityDefinitionRequest = b'c' as isize,
    OrderSingle = b'D' as isize,
    SecurityDefinition = b'd' as isize,
    OrderList = b'E' as isize,
    SecurityStatusRequest = b'e' as isize,
    SecurityStatus = b'f' as isize,
    OrderCancelRequest = b'F' as isize,
    OrderCancelReplaceRequest = b'G' as isize,
    TradingSessionStatusRequest = b'g' as isize,
    OrderStatusRequest = b'H' as isize,
    TradingSessionStatus = b'h' as isize,
    MassQuote = b'i' as isize,
    BusinessMessageReject = b'j' as isize,
    Allocation = b'J' as isize,
    ListCancelRequest = b'K' as isize,
    BidRequest = b'k' as isize,
    BidResponse = b'l' as isize,
    ListExecute = b'L' as isize,
    ListStrikePrice = b'm' as isize,
    ListStatusRequest = b'M' as isize,
    ListStatus = b'N' as isize,
    AllocationAck = b'P' as isize,
    DontKnowTrade = b'Q' as isize,
    QuoteRequest = b'R' as isize,
    Quote = b'S' as isize,
    SettlementInstructions = b'T' as isize,
    MarketDataRequest = b'V' as isize,
    MarketDataSnapshotFullRefresh = b'W' as isize,
    MarketDataIncrementalRefresh = b'X' as isize,
    MarketDataRequestReject = b'Y' as isize,
    QuoteCancel = b'Z' as isize,
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
            b"a" => Some(MsgType::QuoteStatusRequest),
            b"A" => Some(MsgType::Logon),
            b"B" => Some(MsgType::News),
            b"b" => Some(MsgType::QuoteAcknowledgement),
            b"C" => Some(MsgType::Email),
            b"c" => Some(MsgType::SecurityDefinitionRequest),
            b"D" => Some(MsgType::OrderSingle),
            b"d" => Some(MsgType::SecurityDefinition),
            b"E" => Some(MsgType::OrderList),
            b"e" => Some(MsgType::SecurityStatusRequest),
            b"f" => Some(MsgType::SecurityStatus),
            b"F" => Some(MsgType::OrderCancelRequest),
            b"G" => Some(MsgType::OrderCancelReplaceRequest),
            b"g" => Some(MsgType::TradingSessionStatusRequest),
            b"H" => Some(MsgType::OrderStatusRequest),
            b"h" => Some(MsgType::TradingSessionStatus),
            b"i" => Some(MsgType::MassQuote),
            b"j" => Some(MsgType::BusinessMessageReject),
            b"J" => Some(MsgType::Allocation),
            b"K" => Some(MsgType::ListCancelRequest),
            b"k" => Some(MsgType::BidRequest),
            b"l" => Some(MsgType::BidResponse),
            b"L" => Some(MsgType::ListExecute),
            b"m" => Some(MsgType::ListStrikePrice),
            b"M" => Some(MsgType::ListStatusRequest),
            b"N" => Some(MsgType::ListStatus),
            b"P" => Some(MsgType::AllocationAck),
            b"Q" => Some(MsgType::DontKnowTrade),
            b"R" => Some(MsgType::QuoteRequest),
            b"S" => Some(MsgType::Quote),
            b"T" => Some(MsgType::SettlementInstructions),
            b"V" => Some(MsgType::MarketDataRequest),
            b"W" => Some(MsgType::MarketDataSnapshotFullRefresh),
            b"X" => Some(MsgType::MarketDataIncrementalRefresh),
            b"Y" => Some(MsgType::MarketDataRequestReject),
            b"Z" => Some(MsgType::QuoteCancel),
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
pub enum OpenCloseSettleFlag {
    DailyOpen = b'0' as isize,
    SessionOpen = b'1' as isize,
    DeliverySettlementPrice = b'2' as isize,
}

impl FIXValue for OpenCloseSettleFlag {
    fn from_bytes(bytes: &[u8]) -> Option<OpenCloseSettleFlag> {
        match bytes {
            b"0" => Some(OpenCloseSettleFlag::DailyOpen),
            b"1" => Some(OpenCloseSettleFlag::SessionOpen),
            b"2" => Some(OpenCloseSettleFlag::DeliverySettlementPrice),
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
    DuplicateOfAVerballyCommunicatedOrder = b'7' as isize,
    StaleOrder = b'8' as isize,
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
            b"7" => Some(OrdRejReason::DuplicateOfAVerballyCommunicatedOrder),
            b"8" => Some(OrdRejReason::StaleOrder),
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
    PendingCancel = b'6' as isize,
    Stopped = b'7' as isize,
    Rejected = b'8' as isize,
    Suspended = b'9' as isize,
    PendingNew = b'A' as isize,
    Calculated = b'B' as isize,
    Expired = b'C' as isize,
    AcceptedForBidding = b'D' as isize,
    PendingReplace = b'E' as isize,
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
            b"6" => Some(OrdStatus::PendingCancel),
            b"7" => Some(OrdStatus::Stopped),
            b"8" => Some(OrdStatus::Rejected),
            b"9" => Some(OrdStatus::Suspended),
            b"A" => Some(OrdStatus::PendingNew),
            b"B" => Some(OrdStatus::Calculated),
            b"C" => Some(OrdStatus::Expired),
            b"D" => Some(OrdStatus::AcceptedForBidding),
            b"E" => Some(OrdStatus::PendingReplace),
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
    Funari = b'I' as isize,
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
            b"I" => Some(OrdType::Funari),
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
pub enum PriceType {
    Percentage = b'1' as isize,
    PerShare = b'2' as isize,
    FixedAmount = b'3' as isize,
}

impl FIXValue for PriceType {
    fn from_bytes(bytes: &[u8]) -> Option<PriceType> {
        match bytes {
            b"1" => Some(PriceType::Percentage),
            b"2" => Some(PriceType::PerShare),
            b"3" => Some(PriceType::FixedAmount),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteAckStatus {
    Accepted = b'0' as isize,
    CanceledForSymbol = b'1' as isize,
    CanceledForSecurityType = b'2' as isize,
    CanceledForUnderlying = b'3' as isize,
    CanceledAll = b'4' as isize,
    Rejected = b'5' as isize,
}

impl FIXValue for QuoteAckStatus {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteAckStatus> {
        match bytes {
            b"0" => Some(QuoteAckStatus::Accepted),
            b"1" => Some(QuoteAckStatus::CanceledForSymbol),
            b"2" => Some(QuoteAckStatus::CanceledForSecurityType),
            b"3" => Some(QuoteAckStatus::CanceledForUnderlying),
            b"4" => Some(QuoteAckStatus::CanceledAll),
            b"5" => Some(QuoteAckStatus::Rejected),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteCancelType {
    CancelForSymbol = b'1' as isize,
    CancelForSecurityType = b'2' as isize,
    CancelForUnderlyingSymbol = b'3' as isize,
    CancelForAllQuotes = b'4' as isize,
}

impl FIXValue for QuoteCancelType {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteCancelType> {
        match bytes {
            b"1" => Some(QuoteCancelType::CancelForSymbol),
            b"2" => Some(QuoteCancelType::CancelForSecurityType),
            b"3" => Some(QuoteCancelType::CancelForUnderlyingSymbol),
            b"4" => Some(QuoteCancelType::CancelForAllQuotes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteEntryRejectReason {
    UnknownSymbol = b'1' as isize,
    Exchange = b'2' as isize,
    QuoteExceedsLimit = b'3' as isize,
    TooLateToEnter = b'4' as isize,
    UnknownQuote = b'5' as isize,
    DuplicateQuote = b'6' as isize,
    InvalidBidAskSpread = b'7' as isize,
    InvalidPrice = b'8' as isize,
    NotAuthorizedToQuoteSecurity = b'9' as isize,
}

impl FIXValue for QuoteEntryRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteEntryRejectReason> {
        match bytes {
            b"1" => Some(QuoteEntryRejectReason::UnknownSymbol),
            b"2" => Some(QuoteEntryRejectReason::Exchange),
            b"3" => Some(QuoteEntryRejectReason::QuoteExceedsLimit),
            b"4" => Some(QuoteEntryRejectReason::TooLateToEnter),
            b"5" => Some(QuoteEntryRejectReason::UnknownQuote),
            b"6" => Some(QuoteEntryRejectReason::DuplicateQuote),
            b"7" => Some(QuoteEntryRejectReason::InvalidBidAskSpread),
            b"8" => Some(QuoteEntryRejectReason::InvalidPrice),
            b"9" => Some(QuoteEntryRejectReason::NotAuthorizedToQuoteSecurity),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteRejectReason {
    UnknownSymbol = b'1' as isize,
    Exchange = b'2' as isize,
    QuoteRequestExceedsLimit = b'3' as isize,
    TooLateToEnter = b'4' as isize,
    UnknownQuote = b'5' as isize,
    DuplicateQuote = b'6' as isize,
    InvalidBidAskSpread = b'7' as isize,
    InvalidPrice = b'8' as isize,
    NotAuthorizedToQuoteSecurity = b'9' as isize,
}

impl FIXValue for QuoteRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteRejectReason> {
        match bytes {
            b"1" => Some(QuoteRejectReason::UnknownSymbol),
            b"2" => Some(QuoteRejectReason::Exchange),
            b"3" => Some(QuoteRejectReason::QuoteRequestExceedsLimit),
            b"4" => Some(QuoteRejectReason::TooLateToEnter),
            b"5" => Some(QuoteRejectReason::UnknownQuote),
            b"6" => Some(QuoteRejectReason::DuplicateQuote),
            b"7" => Some(QuoteRejectReason::InvalidBidAskSpread),
            b"8" => Some(QuoteRejectReason::InvalidPrice),
            b"9" => Some(QuoteRejectReason::NotAuthorizedToQuoteSecurity),
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
pub enum SecurityResponseType {
    AcceptSecurityProposalAsIs = b'1' as isize,
    AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage = b'2' as isize,
    ListOfSecurityTypesReturnedPerRequest = b'3' as isize,
    ListOfSecuritiesReturnedPerRequest = b'4' as isize,
    RejectSecurityProposal = b'5' as isize,
    CanNotMatchSelectionCriteria = b'6' as isize,
}

impl FIXValue for SecurityResponseType {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityResponseType> {
        match bytes {
            b"1" => Some(SecurityResponseType::AcceptSecurityProposalAsIs),
            b"2" => Some(SecurityResponseType::AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage),
            b"3" => Some(SecurityResponseType::ListOfSecurityTypesReturnedPerRequest),
            b"4" => Some(SecurityResponseType::ListOfSecuritiesReturnedPerRequest),
            b"5" => Some(SecurityResponseType::RejectSecurityProposal),
            b"6" => Some(SecurityResponseType::CanNotMatchSelectionCriteria),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityTradingStatus {
    OpeningDelay = b'1' as isize,
    TradingHalt = b'2' as isize,
    Resume = b'3' as isize,
    NoOpenNoResume = b'4' as isize,
    PriceIndication = b'5' as isize,
    TradingRangeIndication = b'6' as isize,
    MarketImbalanceBuy = b'7' as isize,
    MarketImbalanceSell = b'8' as isize,
    MarketOnCloseImbalanceBuy = b'9' as isize,
    MarketOnCloseImbalanceSell,
    NoMarketImbalance,
    NoMarketOnCloseImbalance,
    ItsPreOpening,
    NewPriceIndication,
    TradeDisseminationTime,
    ReadyToTrade,
    NotAvailableForTrading,
    NotTradedOnThisMarket,
    UnknownOrInvalid,
}

impl FIXValue for SecurityTradingStatus {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityTradingStatus> {
        match bytes {
            b"1" => Some(SecurityTradingStatus::OpeningDelay),
            b"2" => Some(SecurityTradingStatus::TradingHalt),
            b"3" => Some(SecurityTradingStatus::Resume),
            b"4" => Some(SecurityTradingStatus::NoOpenNoResume),
            b"5" => Some(SecurityTradingStatus::PriceIndication),
            b"6" => Some(SecurityTradingStatus::TradingRangeIndication),
            b"7" => Some(SecurityTradingStatus::MarketImbalanceBuy),
            b"8" => Some(SecurityTradingStatus::MarketImbalanceSell),
            b"9" => Some(SecurityTradingStatus::MarketOnCloseImbalanceBuy),
            b"10" => Some(SecurityTradingStatus::MarketOnCloseImbalanceSell),
            b"12" => Some(SecurityTradingStatus::NoMarketImbalance),
            b"13" => Some(SecurityTradingStatus::NoMarketOnCloseImbalance),
            b"14" => Some(SecurityTradingStatus::ItsPreOpening),
            b"15" => Some(SecurityTradingStatus::NewPriceIndication),
            b"16" => Some(SecurityTradingStatus::TradeDisseminationTime),
            b"17" => Some(SecurityTradingStatus::ReadyToTrade),
            b"18" => Some(SecurityTradingStatus::NotAvailableForTrading),
            b"19" => Some(SecurityTradingStatus::NotTradedOnThisMarket),
            b"20" => Some(SecurityTradingStatus::UnknownOrInvalid),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            SecurityTradingStatus::MarketOnCloseImbalanceSell => v.extend(b"10"),
            SecurityTradingStatus::NoMarketImbalance => v.extend(b"12"),
            SecurityTradingStatus::NoMarketOnCloseImbalance => v.extend(b"13"),
            SecurityTradingStatus::ItsPreOpening => v.extend(b"14"),
            SecurityTradingStatus::NewPriceIndication => v.extend(b"15"),
            SecurityTradingStatus::TradeDisseminationTime => v.extend(b"16"),
            SecurityTradingStatus::ReadyToTrade => v.extend(b"17"),
            SecurityTradingStatus::NotAvailableForTrading => v.extend(b"18"),
            SecurityTradingStatus::NotTradedOnThisMarket => v.extend(b"19"),
            SecurityTradingStatus::UnknownOrInvalid => v.extend(b"20"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityType {
    WildcardEntry = b'?' as isize,
    BankersAcceptance,
    ConvertibleBond,
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
    MortgageIoette,
    MutualFund,
    MortgageInterestOnly,
    MortgagePrincipalOnly,
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
            b"?" => Some(SecurityType::WildcardEntry),
            b"BA" => Some(SecurityType::BankersAcceptance),
            b"CB" => Some(SecurityType::ConvertibleBond),
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
            b"IET" => Some(SecurityType::MortgageIoette),
            b"MF" => Some(SecurityType::MutualFund),
            b"MIO" => Some(SecurityType::MortgageInterestOnly),
            b"MPO" => Some(SecurityType::MortgagePrincipalOnly),
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
            SecurityType::ConvertibleBond => v.extend(b"CB"),
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
            SecurityType::MortgageIoette => v.extend(b"IET"),
            SecurityType::MutualFund => v.extend(b"MF"),
            SecurityType::MortgageInterestOnly => v.extend(b"MIO"),
            SecurityType::MortgagePrincipalOnly => v.extend(b"MPO"),
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
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRejectReason {
    InvalidTagNumber = b'0' as isize,
    RequiredTagMissing = b'1' as isize,
    TagNotDefinedForThisMessageType = b'2' as isize,
    UndefinedTag = b'3' as isize,
    TagSpecifiedWithoutAValue = b'4' as isize,
    ValueIsIncorrect = b'5' as isize,
    IncorrectDataFormatForValue = b'6' as isize,
    DecryptionProblem = b'7' as isize,
    SignatureProblem = b'8' as isize,
    CompidProblem = b'9' as isize,
    SendingtimeAccuracyProblem,
    InvalidMsgtype,
}

impl FIXValue for SessionRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<SessionRejectReason> {
        match bytes {
            b"0" => Some(SessionRejectReason::InvalidTagNumber),
            b"1" => Some(SessionRejectReason::RequiredTagMissing),
            b"2" => Some(SessionRejectReason::TagNotDefinedForThisMessageType),
            b"3" => Some(SessionRejectReason::UndefinedTag),
            b"4" => Some(SessionRejectReason::TagSpecifiedWithoutAValue),
            b"5" => Some(SessionRejectReason::ValueIsIncorrect),
            b"6" => Some(SessionRejectReason::IncorrectDataFormatForValue),
            b"7" => Some(SessionRejectReason::DecryptionProblem),
            b"8" => Some(SessionRejectReason::SignatureProblem),
            b"9" => Some(SessionRejectReason::CompidProblem),
            b"10" => Some(SessionRejectReason::SendingtimeAccuracyProblem),
            b"11" => Some(SessionRejectReason::InvalidMsgtype),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            SessionRejectReason::SendingtimeAccuracyProblem => v.extend(b"10"),
            SessionRejectReason::InvalidMsgtype => v.extend(b"11"),
            _ => v.push(*self as u8)
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
    CrossShort = b'9' as isize,
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
            b"9" => Some(Side::CrossShort),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradSesStatus {
    Halted = b'1' as isize,
    Open = b'2' as isize,
    Closed = b'3' as isize,
    PreOpen = b'4' as isize,
    PreClose = b'5' as isize,
}

impl FIXValue for TradSesStatus {
    fn from_bytes(bytes: &[u8]) -> Option<TradSesStatus> {
        match bytes {
            b"1" => Some(TradSesStatus::Halted),
            b"2" => Some(TradSesStatus::Open),
            b"3" => Some(TradSesStatus::Closed),
            b"4" => Some(TradSesStatus::PreOpen),
            b"5" => Some(TradSesStatus::PreClose),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeCondition {
    Cash = b'A' as isize,
    AveragePriceTrade = b'B' as isize,
    CashTrade = b'C' as isize,
    NextDay = b'D' as isize,
    Opening = b'E' as isize,
    IntradayTradeDetail = b'F' as isize,
    Rule127Trade = b'G' as isize,
    Rule155Trade = b'H' as isize,
    SoldLast = b'I' as isize,
    NextDayTrade = b'J' as isize,
    Opened = b'K' as isize,
    Seller = b'L' as isize,
    Sold = b'M' as isize,
    StoppedStock = b'N' as isize,
}

impl FIXValue for TradeCondition {
    fn from_bytes(bytes: &[u8]) -> Option<TradeCondition> {
        match bytes {
            b"A" => Some(TradeCondition::Cash),
            b"B" => Some(TradeCondition::AveragePriceTrade),
            b"C" => Some(TradeCondition::CashTrade),
            b"D" => Some(TradeCondition::NextDay),
            b"E" => Some(TradeCondition::Opening),
            b"F" => Some(TradeCondition::IntradayTradeDetail),
            b"G" => Some(TradeCondition::Rule127Trade),
            b"H" => Some(TradeCondition::Rule155Trade),
            b"I" => Some(TradeCondition::SoldLast),
            b"J" => Some(TradeCondition::NextDayTrade),
            b"K" => Some(TradeCondition::Opened),
            b"L" => Some(TradeCondition::Seller),
            b"M" => Some(TradeCondition::Sold),
            b"N" => Some(TradeCondition::StoppedStock),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

