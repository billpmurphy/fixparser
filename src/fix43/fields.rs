use types::*;
use protocol::FIXValue;
pub use common::AccountType as AccountType;
pub use common::Adjustment as Adjustment;
pub use common::AdvSide as AdvSide;
pub use common::AdvTransType as AdvTransType;
pub use common::AggregatedBook as AggregatedBook;
pub use common::AllocHandlInst as AllocHandlInst;
pub use common::AllocLinkType as AllocLinkType;
pub use common::BasisPxType as BasisPxType;
pub use common::Benchmark as Benchmark;
pub use common::BookingUnit as BookingUnit;
pub use common::CancellationRights as CancellationRights;
pub use common::CashMargin as CashMargin;
pub use common::ContAmtType as ContAmtType;
pub use common::CorporateAction as CorporateAction;
pub use common::CoveredOrUncovered as CoveredOrUncovered;
pub use common::CxlRejResponseTo as CxlRejResponseTo;
pub use common::DeleteReason as DeleteReason;
pub use common::DueToRelated as DueToRelated;
pub use common::EmailType as EmailType;
pub use common::EncryptMethod as EncryptMethod;
pub use common::ExchangeForPhysical as ExchangeForPhysical;
pub use common::ExecPriceType as ExecPriceType;
pub use common::ForexReq as ForexReq;
pub use common::FundRenewWaiv as FundRenewWaiv;
pub use common::GTBookingInst as GTBookingInst;
pub use common::GapFillFlag as GapFillFlag;
pub use common::HaltReasonChar as HaltReasonChar;
pub use common::HandlInst as HandlInst;
pub use common::IOINaturalFlag as IOINaturalFlag;
pub use common::IOIQltyInd as IOIQltyInd;
pub use common::IOIQty as IOIQty;
pub use common::IOITransType as IOITransType;
pub use common::InViewOfCommon as InViewOfCommon;
pub use common::IncTaxInd as IncTaxInd;
pub use common::LastCapacity as LastCapacity;
pub use common::LegalConfirm as LegalConfirm;
pub use common::LocateReqd as LocateReqd;
pub use common::MDImplicitDelete as MDImplicitDelete;
pub use common::MDUpdateAction as MDUpdateAction;
pub use common::MDUpdateType as MDUpdateType;
pub use common::MassCancelRequestType as MassCancelRequestType;
pub use common::MassCancelResponse as MassCancelResponse;
pub use common::MassStatusReqType as MassStatusReqType;
pub use common::MatchStatus as MatchStatus;
pub use common::MoneyLaunderingStatus as MoneyLaunderingStatus;
pub use common::MsgDirection as MsgDirection;
pub use common::MultiLegReportingType as MultiLegReportingType;
pub use common::NetGrossInd as NetGrossInd;
pub use common::NoSides as NoSides;
pub use common::NotifyBrokerOfCredit as NotifyBrokerOfCredit;
pub use common::OrderCapacity as OrderCapacity;
pub use common::OrderRestrictions as OrderRestrictions;
pub use common::OwnerType as OwnerType;
pub use common::PaymentMethod as PaymentMethod;
pub use common::PositionEffect as PositionEffect;
pub use common::PossDupFlag as PossDupFlag;
pub use common::PreallocMethod as PreallocMethod;
pub use common::PreviouslyReported as PreviouslyReported;
pub use common::PriorityIndicator as PriorityIndicator;
pub use common::ProcessCode as ProcessCode;
pub use common::ProgRptReqs as ProgRptReqs;
pub use common::QuoteCondition as QuoteCondition;
pub use common::QuoteRequestType as QuoteRequestType;
pub use common::QuoteResponseLevel as QuoteResponseLevel;
pub use common::RegistTransType as RegistTransType;
pub use common::ReportToExch as ReportToExch;
pub use common::ResetSeqNumFlag as ResetSeqNumFlag;
pub use common::RoundingDirection as RoundingDirection;
pub use common::RoutingType as RoutingType;
pub use common::Scope as Scope;
pub use common::SecurityListRequestType as SecurityListRequestType;
pub use common::SecurityRequestResult as SecurityRequestResult;
pub use common::SecurityRequestType as SecurityRequestType;
pub use common::SettlCurrFxRateCalc as SettlCurrFxRateCalc;
pub use common::SolicitedFlag as SolicitedFlag;
pub use common::SubscriptionRequestType as SubscriptionRequestType;
pub use common::TaxAdvantageType as TaxAdvantageType;
pub use common::TestMessageIndicator as TestMessageIndicator;
pub use common::TickDirection as TickDirection;
pub use common::TradSesMethod as TradSesMethod;
pub use common::TradSesMode as TradSesMode;
pub use common::TradeRequestType as TradeRequestType;
pub use common::TradeType as TradeType;
pub use common::TradedFlatSwitch as TradedFlatSwitch;
pub use common::UnsolicitedIndicator as UnsolicitedIndicator;
pub use common::Urgency as Urgency;
pub use common::WorkingIndicator as WorkingIndicator;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocRejCode {
    UnknownAccount = b'0' as isize,
    UnknownListid = b'6' as isize,
    UnknownExecutingBrokerMnemonic = b'3' as isize,
    UnknownOrderid = b'5' as isize,
    Other = b'7' as isize,
    CommissionDifference = b'4' as isize,
    IncorrectQuantity = b'1' as isize,
    IncorrectAveragePrice = b'2' as isize,
}

impl FIXValue for AllocRejCode {
    fn from_bytes(bytes: &[u8]) -> Option<AllocRejCode> {
        match bytes {
            b"0" => Some(AllocRejCode::UnknownAccount),
            b"6" => Some(AllocRejCode::UnknownListid),
            b"3" => Some(AllocRejCode::UnknownExecutingBrokerMnemonic),
            b"5" => Some(AllocRejCode::UnknownOrderid),
            b"7" => Some(AllocRejCode::Other),
            b"4" => Some(AllocRejCode::CommissionDifference),
            b"1" => Some(AllocRejCode::IncorrectQuantity),
            b"2" => Some(AllocRejCode::IncorrectAveragePrice),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocStatus {
    Rejected = b'1' as isize,
    PartialAccept = b'2' as isize,
    Received = b'3' as isize,
    Accepted = b'0' as isize,
}

impl FIXValue for AllocStatus {
    fn from_bytes(bytes: &[u8]) -> Option<AllocStatus> {
        match bytes {
            b"1" => Some(AllocStatus::Rejected),
            b"2" => Some(AllocStatus::PartialAccept),
            b"3" => Some(AllocStatus::Received),
            b"0" => Some(AllocStatus::Accepted),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocTransType {
    CalculatedWithoutPreliminary = b'5' as isize,
    Calculated = b'4' as isize,
    Preliminary = b'3' as isize,
    Cancel = b'2' as isize,
    Replace = b'1' as isize,
    New = b'0' as isize,
}

impl FIXValue for AllocTransType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocTransType> {
        match bytes {
            b"5" => Some(AllocTransType::CalculatedWithoutPreliminary),
            b"4" => Some(AllocTransType::Calculated),
            b"3" => Some(AllocTransType::Preliminary),
            b"2" => Some(AllocTransType::Cancel),
            b"1" => Some(AllocTransType::Replace),
            b"0" => Some(AllocTransType::New),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocType {
    BuysideReadyToBook6 = b'6' as isize,
    BuysidePreliminary = b'2' as isize,
    SellsideCalculatedUsingPreliminary = b'3' as isize,
    BuysideReadyToBook5 = b'5' as isize,
    BuysideCalculated = b'1' as isize,
    SellsideCalculatedWithoutPreliminary = b'4' as isize,
}

impl FIXValue for AllocType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocType> {
        match bytes {
            b"6" => Some(AllocType::BuysideReadyToBook6),
            b"2" => Some(AllocType::BuysidePreliminary),
            b"3" => Some(AllocType::SellsideCalculatedUsingPreliminary),
            b"5" => Some(AllocType::BuysideReadyToBook5),
            b"1" => Some(AllocType::BuysideCalculated),
            b"4" => Some(AllocType::SellsideCalculatedWithoutPreliminary),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchmarkCurveName {
    Euribor,
    Futureswap,
    Libid,
    Libor,
    Muniaaa,
    Other,
    Pfandbriefe,
    Swap,
    Treasury,
}

impl FIXValue for BenchmarkCurveName {
    fn from_bytes(bytes: &[u8]) -> Option<BenchmarkCurveName> {
        match bytes {
            b"Euribor" => Some(BenchmarkCurveName::Euribor),
            b"FutureSWAP" => Some(BenchmarkCurveName::Futureswap),
            b"LIBID" => Some(BenchmarkCurveName::Libid),
            b"LIBOR" => Some(BenchmarkCurveName::Libor),
            b"MuniAAA" => Some(BenchmarkCurveName::Muniaaa),
            b"OTHER" => Some(BenchmarkCurveName::Other),
            b"Pfandbriefe" => Some(BenchmarkCurveName::Pfandbriefe),
            b"SWAP" => Some(BenchmarkCurveName::Swap),
            b"Treasury" => Some(BenchmarkCurveName::Treasury),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            BenchmarkCurveName::Euribor => v.extend(b"Euribor"),
            BenchmarkCurveName::Futureswap => v.extend(b"FutureSWAP"),
            BenchmarkCurveName::Libid => v.extend(b"LIBID"),
            BenchmarkCurveName::Libor => v.extend(b"LIBOR"),
            BenchmarkCurveName::Muniaaa => v.extend(b"MuniAAA"),
            BenchmarkCurveName::Other => v.extend(b"OTHER"),
            BenchmarkCurveName::Pfandbriefe => v.extend(b"Pfandbriefe"),
            BenchmarkCurveName::Swap => v.extend(b"SWAP"),
            BenchmarkCurveName::Treasury => v.extend(b"Treasury"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidDescriptorType {
    Index = b'3' as isize,
    Country = b'2' as isize,
    Sector = b'1' as isize,
}

impl FIXValue for BidDescriptorType {
    fn from_bytes(bytes: &[u8]) -> Option<BidDescriptorType> {
        match bytes {
            b"3" => Some(BidDescriptorType::Index),
            b"2" => Some(BidDescriptorType::Country),
            b"1" => Some(BidDescriptorType::Sector),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidRequestTransType {
    New = b'N' as isize,
    Cancel = b'C' as isize,
}

impl FIXValue for BidRequestTransType {
    fn from_bytes(bytes: &[u8]) -> Option<BidRequestTransType> {
        match bytes {
            b"N" => Some(BidRequestTransType::New),
            b"C" => Some(BidRequestTransType::Cancel),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidType {
    NonDisclosedStyle = b'1' as isize,
    DisclosedStyle = b'2' as isize,
    NoBiddingProcess = b'3' as isize,
}

impl FIXValue for BidType {
    fn from_bytes(bytes: &[u8]) -> Option<BidType> {
        match bytes {
            b"1" => Some(BidType::NonDisclosedStyle),
            b"2" => Some(BidType::DisclosedStyle),
            b"3" => Some(BidType::NoBiddingProcess),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusinessRejectReason {
    UnsupportedMessageType = b'3' as isize,
    DelivertoFirmNotAvailableAtThisTime = b'7' as isize,
    ApplicationNotAvailable = b'4' as isize,
    NotAuthorized = b'6' as isize,
    Other = b'0' as isize,
    ConditionallyRequiredFieldMissing = b'5' as isize,
    UnkownId = b'1' as isize,
    UnknownSecurity = b'2' as isize,
}

impl FIXValue for BusinessRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<BusinessRejectReason> {
        match bytes {
            b"3" => Some(BusinessRejectReason::UnsupportedMessageType),
            b"7" => Some(BusinessRejectReason::DelivertoFirmNotAvailableAtThisTime),
            b"4" => Some(BusinessRejectReason::ApplicationNotAvailable),
            b"6" => Some(BusinessRejectReason::NotAuthorized),
            b"0" => Some(BusinessRejectReason::Other),
            b"5" => Some(BusinessRejectReason::ConditionallyRequiredFieldMissing),
            b"1" => Some(BusinessRejectReason::UnkownId),
            b"2" => Some(BusinessRejectReason::UnknownSecurity),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearingFeeIndicator {
    OnehundredandsixhAnd106JFirms = b'H' as isize,
    FivethYearDelegateTradingForHisOwnAccount = b'5' as isize,
    FourthYearDelegateTradingForHisOwnAccount = b'4' as isize,
    ThreerdYearDelegateTradingForHisOwnAccount = b'3' as isize,
    TwondYearDelegateTradingForHisOwnAccount = b'2' as isize,
    OnestYearDelegateTradingForHisOwnAccount = b'1' as isize,
    AllOtherOwnershipTypes = b'M' as isize,
    GimIdemAndComMembershipInterestHolders = b'I' as isize,
    SixthYearAndBeyondDelegateTradingForHisOwnAccount = b'9' as isize,
    FullAndAssociateMemberTradingForOwnAccountAndAsFloor = b'F' as isize,
    EquityMemberAndClearingMember = b'E' as isize,
    NonMemberAndCustomer = b'C' as isize,
    CboeMember = b'B' as isize,
    LesseeAnd106FEmployees = b'L' as isize,
}

impl FIXValue for ClearingFeeIndicator {
    fn from_bytes(bytes: &[u8]) -> Option<ClearingFeeIndicator> {
        match bytes {
            b"H" => Some(ClearingFeeIndicator::OnehundredandsixhAnd106JFirms),
            b"5" => Some(ClearingFeeIndicator::FivethYearDelegateTradingForHisOwnAccount),
            b"4" => Some(ClearingFeeIndicator::FourthYearDelegateTradingForHisOwnAccount),
            b"3" => Some(ClearingFeeIndicator::ThreerdYearDelegateTradingForHisOwnAccount),
            b"2" => Some(ClearingFeeIndicator::TwondYearDelegateTradingForHisOwnAccount),
            b"1" => Some(ClearingFeeIndicator::OnestYearDelegateTradingForHisOwnAccount),
            b"M" => Some(ClearingFeeIndicator::AllOtherOwnershipTypes),
            b"I" => Some(ClearingFeeIndicator::GimIdemAndComMembershipInterestHolders),
            b"9" => Some(ClearingFeeIndicator::SixthYearAndBeyondDelegateTradingForHisOwnAccount),
            b"F" => Some(ClearingFeeIndicator::FullAndAssociateMemberTradingForOwnAccountAndAsFloor),
            b"E" => Some(ClearingFeeIndicator::EquityMemberAndClearingMember),
            b"C" => Some(ClearingFeeIndicator::NonMemberAndCustomer),
            b"B" => Some(ClearingFeeIndicator::CboeMember),
            b"L" => Some(ClearingFeeIndicator::LesseeAnd106FEmployees),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearingInstruction {
    ProcessNormally = b'0' as isize,
    ExcludeFromAllNetting = b'1' as isize,
    BilateralNettingOnly = b'2' as isize,
    ExClearing = b'3' as isize,
    SpecialTrade = b'4' as isize,
    MultilateralNetting = b'5' as isize,
    ClearAgainstCentralCounterparty = b'6' as isize,
    ExcludeFromCentralCounterparty = b'7' as isize,
    ManualMode = b'8' as isize,
    AutomaticPostingMode = b'9' as isize,
    AutomaticGiveUpMode,
}

impl FIXValue for ClearingInstruction {
    fn from_bytes(bytes: &[u8]) -> Option<ClearingInstruction> {
        match bytes {
            b"0" => Some(ClearingInstruction::ProcessNormally),
            b"1" => Some(ClearingInstruction::ExcludeFromAllNetting),
            b"2" => Some(ClearingInstruction::BilateralNettingOnly),
            b"3" => Some(ClearingInstruction::ExClearing),
            b"4" => Some(ClearingInstruction::SpecialTrade),
            b"5" => Some(ClearingInstruction::MultilateralNetting),
            b"6" => Some(ClearingInstruction::ClearAgainstCentralCounterparty),
            b"7" => Some(ClearingInstruction::ExcludeFromCentralCounterparty),
            b"8" => Some(ClearingInstruction::ManualMode),
            b"9" => Some(ClearingInstruction::AutomaticPostingMode),
            b"10" => Some(ClearingInstruction::AutomaticGiveUpMode),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            ClearingInstruction::AutomaticGiveUpMode => v.extend(b"10"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommType {
    PerBond = b'6' as isize,
    PerShare = b'1' as isize,
    Percentage = b'2' as isize,
    Absolute = b'3' as isize,
    Five = b'5' as isize,
    Four = b'4' as isize,
}

impl FIXValue for CommType {
    fn from_bytes(bytes: &[u8]) -> Option<CommType> {
        match bytes {
            b"6" => Some(CommType::PerBond),
            b"1" => Some(CommType::PerShare),
            b"2" => Some(CommType::Percentage),
            b"3" => Some(CommType::Absolute),
            b"5" => Some(CommType::Five),
            b"4" => Some(CommType::Four),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossPrioritization {
    SellsidePrioritized = b'2' as isize,
    None = b'0' as isize,
    BuysidePrioritized = b'1' as isize,
}

impl FIXValue for CrossPrioritization {
    fn from_bytes(bytes: &[u8]) -> Option<CrossPrioritization> {
        match bytes {
            b"2" => Some(CrossPrioritization::SellsidePrioritized),
            b"0" => Some(CrossPrioritization::None),
            b"1" => Some(CrossPrioritization::BuysidePrioritized),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossType {
    CrossTradeWhichIsExecutedCompletelyOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone = b'1' as isize,
    CrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelledOneSideIsFullyExecutedTheOtherSideIsPartiallyExecutedWithTheRemainderBeingCancelledThisIsEquivalentToAnImmediateOrCancelOnTheOtherSide = b'2' as isize,
    CrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActiveOneSideOfTheCrossIsFullyExecuted = b'3' as isize,
    CrossTradeIsExecutedWithExistingOrdersWithTheSamePrice = b'4' as isize,
}

impl FIXValue for CrossType {
    fn from_bytes(bytes: &[u8]) -> Option<CrossType> {
        match bytes {
            b"1" => Some(CrossType::CrossTradeWhichIsExecutedCompletelyOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone),
            b"2" => Some(CrossType::CrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelledOneSideIsFullyExecutedTheOtherSideIsPartiallyExecutedWithTheRemainderBeingCancelledThisIsEquivalentToAnImmediateOrCancelOnTheOtherSide),
            b"3" => Some(CrossType::CrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActiveOneSideOfTheCrossIsFullyExecuted),
            b"4" => Some(CrossType::CrossTradeIsExecutedWithExistingOrdersWithTheSamePrice),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CxlRejReason {
    UnknownOrder = b'1' as isize,
    TooLateToCancel = b'0' as isize,
    DuplicateClordidReceived = b'6' as isize,
    OrigordmodtimeDidNotMatchLastTransacttimeOfOrder = b'5' as isize,
    UnableToProcessOrderMassCancelRequest = b'4' as isize,
    OrderAlreadyInPendingCancelOrPendingReplaceStatus = b'3' as isize,
    Broker = b'2' as isize,
}

impl FIXValue for CxlRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<CxlRejReason> {
        match bytes {
            b"1" => Some(CxlRejReason::UnknownOrder),
            b"0" => Some(CxlRejReason::TooLateToCancel),
            b"6" => Some(CxlRejReason::DuplicateClordidReceived),
            b"5" => Some(CxlRejReason::OrigordmodtimeDidNotMatchLastTransacttimeOfOrder),
            b"4" => Some(CxlRejReason::UnableToProcessOrderMassCancelRequest),
            b"3" => Some(CxlRejReason::OrderAlreadyInPendingCancelOrPendingReplaceStatus),
            b"2" => Some(CxlRejReason::Broker),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DKReason {
    WrongSide = b'B' as isize,
    QuantityExceedsOrder = b'C' as isize,
    NoMatchingOrder = b'D' as isize,
    PriceExceedsLimit = b'E' as isize,
    Other = b'Z' as isize,
    UnknownSymbol = b'A' as isize,
}

impl FIXValue for DKReason {
    fn from_bytes(bytes: &[u8]) -> Option<DKReason> {
        match bytes {
            b"B" => Some(DKReason::WrongSide),
            b"C" => Some(DKReason::QuantityExceedsOrder),
            b"D" => Some(DKReason::NoMatchingOrder),
            b"E" => Some(DKReason::PriceExceedsLimit),
            b"Z" => Some(DKReason::Other),
            b"A" => Some(DKReason::UnknownSymbol),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayBookingInst {
    CanTriggerBookingWithoutReferenceToTheOrderInitiator = b'0' as isize,
    SpeakWithOrderInitiatorBeforeBooking = b'1' as isize,
}

impl FIXValue for DayBookingInst {
    fn from_bytes(bytes: &[u8]) -> Option<DayBookingInst> {
        match bytes {
            b"0" => Some(DayBookingInst::CanTriggerBookingWithoutReferenceToTheOrderInitiator),
            b"1" => Some(DayBookingInst::SpeakWithOrderInitiatorBeforeBooking),
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
    Trytostop = b'Y' as isize,
    Midprcpeg = b'M' as isize,
    Markpeg = b'P' as isize,
    Cancelonsysfail = b'Q' as isize,
    Primpeg = b'R' as isize,
    Suspend = b'S' as isize,
    Custdispinst = b'U' as isize,
    Netting = b'V' as isize,
    Pegvwap = b'W' as isize,
    Tradealong = b'X' as isize,
    Percvol = b'D' as isize,
    Stayoffer = b'0' as isize,
    Work = b'2' as isize,
    Overday = b'4' as isize,
    Held = b'5' as isize,
    Partnotinit = b'6' as isize,
    Strictscale = b'7' as isize,
    Trytoscale = b'8' as isize,
    Staybid = b'9' as isize,
    Nocross = b'A' as isize,
    Openpeg = b'O' as isize,
    Callfirst = b'C' as isize,
    Nonnego = b'N' as isize,
    Dni = b'E' as isize,
    Dnr = b'F' as isize,
    Aon = b'G' as isize,
    Restateonsysfail = b'H' as isize,
    Institonly = b'I' as isize,
    Restateontradinghalt = b'J' as isize,
    Cancelontradinghalt = b'K' as isize,
    Lastpeg = b'L' as isize,
    Goalong = b'3' as isize,
    Okcross = b'B' as isize,
    Notheld = b'1' as isize,
}

impl FIXValue for ExecInst {
    fn from_bytes(bytes: &[u8]) -> Option<ExecInst> {
        match bytes {
            b"Y" => Some(ExecInst::Trytostop),
            b"M" => Some(ExecInst::Midprcpeg),
            b"P" => Some(ExecInst::Markpeg),
            b"Q" => Some(ExecInst::Cancelonsysfail),
            b"R" => Some(ExecInst::Primpeg),
            b"S" => Some(ExecInst::Suspend),
            b"U" => Some(ExecInst::Custdispinst),
            b"V" => Some(ExecInst::Netting),
            b"W" => Some(ExecInst::Pegvwap),
            b"X" => Some(ExecInst::Tradealong),
            b"D" => Some(ExecInst::Percvol),
            b"0" => Some(ExecInst::Stayoffer),
            b"2" => Some(ExecInst::Work),
            b"4" => Some(ExecInst::Overday),
            b"5" => Some(ExecInst::Held),
            b"6" => Some(ExecInst::Partnotinit),
            b"7" => Some(ExecInst::Strictscale),
            b"8" => Some(ExecInst::Trytoscale),
            b"9" => Some(ExecInst::Staybid),
            b"A" => Some(ExecInst::Nocross),
            b"O" => Some(ExecInst::Openpeg),
            b"C" => Some(ExecInst::Callfirst),
            b"N" => Some(ExecInst::Nonnego),
            b"E" => Some(ExecInst::Dni),
            b"F" => Some(ExecInst::Dnr),
            b"G" => Some(ExecInst::Aon),
            b"H" => Some(ExecInst::Restateonsysfail),
            b"I" => Some(ExecInst::Institonly),
            b"J" => Some(ExecInst::Restateontradinghalt),
            b"K" => Some(ExecInst::Cancelontradinghalt),
            b"L" => Some(ExecInst::Lastpeg),
            b"3" => Some(ExecInst::Goalong),
            b"B" => Some(ExecInst::Okcross),
            b"1" => Some(ExecInst::Notheld),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecRestatementReason {
    CancelOnSystemFailure = b'7' as isize,
    GtCorporateAction = b'0' as isize,
    Market = b'8' as isize,
    CancelOnTradingHalt = b'6' as isize,
    PartialDeclineOfOrderqty = b'5' as isize,
    BrokerOption = b'4' as isize,
    RepricingOfOrder = b'3' as isize,
    GtRenewal = b'1' as isize,
    VerbalChange = b'2' as isize,
}

impl FIXValue for ExecRestatementReason {
    fn from_bytes(bytes: &[u8]) -> Option<ExecRestatementReason> {
        match bytes {
            b"7" => Some(ExecRestatementReason::CancelOnSystemFailure),
            b"0" => Some(ExecRestatementReason::GtCorporateAction),
            b"8" => Some(ExecRestatementReason::Market),
            b"6" => Some(ExecRestatementReason::CancelOnTradingHalt),
            b"5" => Some(ExecRestatementReason::PartialDeclineOfOrderqty),
            b"4" => Some(ExecRestatementReason::BrokerOption),
            b"3" => Some(ExecRestatementReason::RepricingOfOrder),
            b"1" => Some(ExecRestatementReason::GtRenewal),
            b"2" => Some(ExecRestatementReason::VerbalChange),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecType {
    PendingCancel = b'6' as isize,
    New = b'0' as isize,
    PartialFill = b'1' as isize,
    Fill = b'2' as isize,
    Canceled = b'4' as isize,
    Replace = b'5' as isize,
    Rejected = b'8' as isize,
    Suspended = b'9' as isize,
    PendingNew = b'A' as isize,
    Calculated = b'B' as isize,
    Expired = b'C' as isize,
    Restated = b'D' as isize,
    PendingReplace = b'E' as isize,
    Trade = b'F' as isize,
    TradeCorrect = b'G' as isize,
    TradeCancel = b'H' as isize,
    OrderStatus = b'I' as isize,
    DoneForDay = b'3' as isize,
    Stopped = b'7' as isize,
}

impl FIXValue for ExecType {
    fn from_bytes(bytes: &[u8]) -> Option<ExecType> {
        match bytes {
            b"6" => Some(ExecType::PendingCancel),
            b"0" => Some(ExecType::New),
            b"1" => Some(ExecType::PartialFill),
            b"2" => Some(ExecType::Fill),
            b"4" => Some(ExecType::Canceled),
            b"5" => Some(ExecType::Replace),
            b"8" => Some(ExecType::Rejected),
            b"9" => Some(ExecType::Suspended),
            b"A" => Some(ExecType::PendingNew),
            b"B" => Some(ExecType::Calculated),
            b"C" => Some(ExecType::Expired),
            b"D" => Some(ExecType::Restated),
            b"E" => Some(ExecType::PendingReplace),
            b"F" => Some(ExecType::Trade),
            b"G" => Some(ExecType::TradeCorrect),
            b"H" => Some(ExecType::TradeCancel),
            b"I" => Some(ExecType::OrderStatus),
            b"3" => Some(ExecType::DoneForDay),
            b"7" => Some(ExecType::Stopped),
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
    PendingDelisting = b'2' as isize,
}

impl FIXValue for FinancialStatus {
    fn from_bytes(bytes: &[u8]) -> Option<FinancialStatus> {
        match bytes {
            b"1" => Some(FinancialStatus::Bankrupt),
            b"2" => Some(FinancialStatus::PendingDelisting),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOIQualifier {
    AtTheOpen = b'O' as isize,
    CrossingOpportunity = b'X' as isize,
    Indication = b'W' as isize,
    Versus = b'V' as isize,
    ThroughTheDay = b'T' as isize,
    PortfolioShown = b'S' as isize,
    ReadyToTrade = b'R' as isize,
    AllOrNone = b'A' as isize,
    TakingAPosition = b'P' as isize,
    MoreBehind = b'M' as isize,
    Limit = b'L' as isize,
    InTouchWith = b'I' as isize,
    Vwap = b'D' as isize,
    AtTheClose = b'C' as isize,
    MarketOnClose = b'B' as isize,
    AtTheMarket = b'Q' as isize,
    AtTheMidpoint = b'Y' as isize,
    PreOpen = b'Z' as isize,
}

impl FIXValue for IOIQualifier {
    fn from_bytes(bytes: &[u8]) -> Option<IOIQualifier> {
        match bytes {
            b"O" => Some(IOIQualifier::AtTheOpen),
            b"X" => Some(IOIQualifier::CrossingOpportunity),
            b"W" => Some(IOIQualifier::Indication),
            b"V" => Some(IOIQualifier::Versus),
            b"T" => Some(IOIQualifier::ThroughTheDay),
            b"S" => Some(IOIQualifier::PortfolioShown),
            b"R" => Some(IOIQualifier::ReadyToTrade),
            b"A" => Some(IOIQualifier::AllOrNone),
            b"P" => Some(IOIQualifier::TakingAPosition),
            b"M" => Some(IOIQualifier::MoreBehind),
            b"L" => Some(IOIQualifier::Limit),
            b"I" => Some(IOIQualifier::InTouchWith),
            b"D" => Some(IOIQualifier::Vwap),
            b"C" => Some(IOIQualifier::AtTheClose),
            b"B" => Some(IOIQualifier::MarketOnClose),
            b"Q" => Some(IOIQualifier::AtTheMarket),
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
    NormalMarketSize = b'3' as isize,
    Other = b'4' as isize,
    TwentyDayMovingAverage = b'2' as isize,
    FivedayMovingAverage = b'1' as isize,
}

impl FIXValue for LiquidityIndType {
    fn from_bytes(bytes: &[u8]) -> Option<LiquidityIndType> {
        match bytes {
            b"3" => Some(LiquidityIndType::NormalMarketSize),
            b"4" => Some(LiquidityIndType::Other),
            b"2" => Some(LiquidityIndType::TwentyDayMovingAverage),
            b"1" => Some(LiquidityIndType::FivedayMovingAverage),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListExecInstType {
    ExchangeSwitchCivOrderBuyDrivenCashWithdraw = b'5' as isize,
    ExchangeSwitchCivOrderBuyDrivenCashTopUp = b'4' as isize,
    WaitForExecuteInstruction = b'2' as isize,
    Immediate = b'1' as isize,
    ExchangeSwitchCivOrderSellDriven = b'3' as isize,
}

impl FIXValue for ListExecInstType {
    fn from_bytes(bytes: &[u8]) -> Option<ListExecInstType> {
        match bytes {
            b"5" => Some(ListExecInstType::ExchangeSwitchCivOrderBuyDrivenCashWithdraw),
            b"4" => Some(ListExecInstType::ExchangeSwitchCivOrderBuyDrivenCashTopUp),
            b"2" => Some(ListExecInstType::WaitForExecuteInstruction),
            b"1" => Some(ListExecInstType::Immediate),
            b"3" => Some(ListExecInstType::ExchangeSwitchCivOrderSellDriven),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListOrderStatus {
    Canceling = b'4' as isize,
    Executing = b'3' as isize,
    Reject = b'7' as isize,
    AllDone = b'6' as isize,
    Alert = b'5' as isize,
    Receivedforexecution = b'2' as isize,
    Inbiddingprocess = b'1' as isize,
}

impl FIXValue for ListOrderStatus {
    fn from_bytes(bytes: &[u8]) -> Option<ListOrderStatus> {
        match bytes {
            b"4" => Some(ListOrderStatus::Canceling),
            b"3" => Some(ListOrderStatus::Executing),
            b"7" => Some(ListOrderStatus::Reject),
            b"6" => Some(ListOrderStatus::AllDone),
            b"5" => Some(ListOrderStatus::Alert),
            b"2" => Some(ListOrderStatus::Receivedforexecution),
            b"1" => Some(ListOrderStatus::Inbiddingprocess),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListStatusType {
    Alert = b'6' as isize,
    Execstarted = b'4' as isize,
    Timed = b'3' as isize,
    Response = b'2' as isize,
    Ack = b'1' as isize,
    Alldone = b'5' as isize,
}

impl FIXValue for ListStatusType {
    fn from_bytes(bytes: &[u8]) -> Option<ListStatusType> {
        match bytes {
            b"6" => Some(ListStatusType::Alert),
            b"4" => Some(ListStatusType::Execstarted),
            b"3" => Some(ListStatusType::Timed),
            b"2" => Some(ListStatusType::Response),
            b"1" => Some(ListStatusType::Ack),
            b"5" => Some(ListStatusType::Alldone),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MDEntryType {
    TradingSessionHighPrice = b'7' as isize,
    Offer = b'1' as isize,
    Imbalance = b'A' as isize,
    TradingSessionVwapPrice = b'9' as isize,
    TradingSessionLowPrice = b'8' as isize,
    ClosingPrice = b'5' as isize,
    OpeningPrice = b'4' as isize,
    Bid = b'0' as isize,
    Trade = b'2' as isize,
    IndexValue = b'3' as isize,
    SettlementPrice = b'6' as isize,
}

impl FIXValue for MDEntryType {
    fn from_bytes(bytes: &[u8]) -> Option<MDEntryType> {
        match bytes {
            b"7" => Some(MDEntryType::TradingSessionHighPrice),
            b"1" => Some(MDEntryType::Offer),
            b"A" => Some(MDEntryType::Imbalance),
            b"9" => Some(MDEntryType::TradingSessionVwapPrice),
            b"8" => Some(MDEntryType::TradingSessionLowPrice),
            b"5" => Some(MDEntryType::ClosingPrice),
            b"4" => Some(MDEntryType::OpeningPrice),
            b"0" => Some(MDEntryType::Bid),
            b"2" => Some(MDEntryType::Trade),
            b"3" => Some(MDEntryType::IndexValue),
            b"6" => Some(MDEntryType::SettlementPrice),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MDReqRejReason {
    UnsupportedAggregatedbook = b'7' as isize,
    DuplicateMdreqid = b'1' as isize,
    UnsupportedMdimplicitdelete = b'C' as isize,
    UnsupportedOpenclosesettleflag = b'B' as isize,
    UnsupportedScope = b'A' as isize,
    UnsupportedTradingsessionid = b'9' as isize,
    UnsupportedMdentrytype = b'8' as isize,
    UnsupportedMdupdatetype = b'6' as isize,
    UnsupportedMarketdepth = b'5' as isize,
    UnsupportedSubscriptionrequesttype = b'4' as isize,
    InsufficientBandwidth = b'2' as isize,
    UnknownSymbol = b'0' as isize,
    InsufficientPermissions = b'3' as isize,
}

impl FIXValue for MDReqRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<MDReqRejReason> {
        match bytes {
            b"7" => Some(MDReqRejReason::UnsupportedAggregatedbook),
            b"1" => Some(MDReqRejReason::DuplicateMdreqid),
            b"C" => Some(MDReqRejReason::UnsupportedMdimplicitdelete),
            b"B" => Some(MDReqRejReason::UnsupportedOpenclosesettleflag),
            b"A" => Some(MDReqRejReason::UnsupportedScope),
            b"9" => Some(MDReqRejReason::UnsupportedTradingsessionid),
            b"8" => Some(MDReqRejReason::UnsupportedMdentrytype),
            b"6" => Some(MDReqRejReason::UnsupportedMdupdatetype),
            b"5" => Some(MDReqRejReason::UnsupportedMarketdepth),
            b"4" => Some(MDReqRejReason::UnsupportedSubscriptionrequesttype),
            b"2" => Some(MDReqRejReason::InsufficientBandwidth),
            b"0" => Some(MDReqRejReason::UnknownSymbol),
            b"3" => Some(MDReqRejReason::InsufficientPermissions),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassCancelRejectReason {
    InvalidOrUnknownUnderlying = b'2' as isize,
    InvalidOrUnknownTradingSession = b'6' as isize,
    InvalidOrUnknownSecurityType = b'5' as isize,
    InvalidOrUnknownProduct = b'3' as isize,
    InvalidOrUnknownSecurity = b'1' as isize,
    MassCancelNotSupported = b'0' as isize,
    InvalidOrUnknownCficode = b'4' as isize,
}

impl FIXValue for MassCancelRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<MassCancelRejectReason> {
        match bytes {
            b"2" => Some(MassCancelRejectReason::InvalidOrUnknownUnderlying),
            b"6" => Some(MassCancelRejectReason::InvalidOrUnknownTradingSession),
            b"5" => Some(MassCancelRejectReason::InvalidOrUnknownSecurityType),
            b"3" => Some(MassCancelRejectReason::InvalidOrUnknownProduct),
            b"1" => Some(MassCancelRejectReason::InvalidOrUnknownSecurity),
            b"0" => Some(MassCancelRejectReason::MassCancelNotSupported),
            b"4" => Some(MassCancelRejectReason::InvalidOrUnknownCficode),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime,
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges,
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime,
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAnd,
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime,
    ComparedRecordsResultingFromStampedAdvisoriesOrSpecialist,
    ActM1Match,
    ActM2Match,
    ActAcceptedTrade,
    ActDefaultTrade,
    ActDefaultAfterM2,
    ActM6Match,
    NonAct,
    SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS1,
    SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS2,
    SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS3,
    SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS4,
    SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS5,
}

impl FIXValue for MatchType {
    fn from_bytes(bytes: &[u8]) -> Option<MatchType> {
        match bytes {
            b"A1" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime),
            b"A2" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges),
            b"A3" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime),
            b"A4" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAnd),
            b"A5" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime),
            b"AQ" => Some(MatchType::ComparedRecordsResultingFromStampedAdvisoriesOrSpecialist),
            b"M1" => Some(MatchType::ActM1Match),
            b"M2" => Some(MatchType::ActM2Match),
            b"M3" => Some(MatchType::ActAcceptedTrade),
            b"M4" => Some(MatchType::ActDefaultTrade),
            b"M5" => Some(MatchType::ActDefaultAfterM2),
            b"M6" => Some(MatchType::ActM6Match),
            b"MT" => Some(MatchType::NonAct),
            b"S1" => Some(MatchType::SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS1),
            b"S2" => Some(MatchType::SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS2),
            b"S3" => Some(MatchType::SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS3),
            b"S4" => Some(MatchType::SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS4),
            b"S5" => Some(MatchType::SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS5),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime => v.extend(b"A1"),
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges => v.extend(b"A2"),
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime => v.extend(b"A3"),
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAnd => v.extend(b"A4"),
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime => v.extend(b"A5"),
            MatchType::ComparedRecordsResultingFromStampedAdvisoriesOrSpecialist => v.extend(b"AQ"),
            MatchType::ActM1Match => v.extend(b"M1"),
            MatchType::ActM2Match => v.extend(b"M2"),
            MatchType::ActAcceptedTrade => v.extend(b"M3"),
            MatchType::ActDefaultTrade => v.extend(b"M4"),
            MatchType::ActDefaultAfterM2 => v.extend(b"M5"),
            MatchType::ActM6Match => v.extend(b"M6"),
            MatchType::NonAct => v.extend(b"MT"),
            MatchType::SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS1 => v.extend(b"S1"),
            MatchType::SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS2 => v.extend(b"S2"),
            MatchType::SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS3 => v.extend(b"S3"),
            MatchType::SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS4 => v.extend(b"S4"),
            MatchType::SummarizedMatchUsingA1ToA5ExactMatchCriteriaExceptQuantityIsSummarizedS5 => v.extend(b"S5"),
        }
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
    LocalCommission = b'3' as isize,
    ExchangeFees = b'4' as isize,
    Stamp = b'5' as isize,
    Levy = b'6' as isize,
    Other = b'7' as isize,
    Markup = b'8' as isize,
    ConsumptionTax = b'9' as isize,
    Regulatory = b'1' as isize,
    Tax = b'2' as isize,
}

impl FIXValue for MiscFeeType {
    fn from_bytes(bytes: &[u8]) -> Option<MiscFeeType> {
        match bytes {
            b"3" => Some(MiscFeeType::LocalCommission),
            b"4" => Some(MiscFeeType::ExchangeFees),
            b"5" => Some(MiscFeeType::Stamp),
            b"6" => Some(MiscFeeType::Levy),
            b"7" => Some(MiscFeeType::Other),
            b"8" => Some(MiscFeeType::Markup),
            b"9" => Some(MiscFeeType::ConsumptionTax),
            b"1" => Some(MiscFeeType::Regulatory),
            b"2" => Some(MiscFeeType::Tax),
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
    OrderSingle = b'D' as isize,
    OrderList = b'E' as isize,
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
    MarketDataRequest = b'V' as isize,
    MarketDataSnapshotFullRefresh = b'W' as isize,
    MarketDataIncrementalRefresh = b'X' as isize,
    MarketDataRequestReject = b'Y' as isize,
    QuoteCancel = b'Z' as isize,
    QuoteStatusRequest = b'a' as isize,
    MassQuoteAcknowledgement = b'b' as isize,
    SecurityDefinitionRequest = b'c' as isize,
    SecurityDefinition = b'd' as isize,
    SecurityStatusRequest = b'e' as isize,
    SecurityStatus = b'f' as isize,
    TradingSessionStatusRequest = b'g' as isize,
    TradingSessionStatus = b'h' as isize,
    MassQuote = b'i' as isize,
    BusinessMessageReject = b'j' as isize,
    BidRequest = b'k' as isize,
    BidResponse = b'l' as isize,
    ListStrikePrice = b'm' as isize,
    XmlMessage = b'n' as isize,
    RegistrationInstructions = b'o' as isize,
    RegistrationInstructionsResponse = b'p' as isize,
    OrderMassCancelRequest = b'q' as isize,
    OrderMassCancelReport = b'r' as isize,
    NewOrderS = b's' as isize,
    CrossOrderCancelReplaceRequest = b't' as isize,
    CrossOrderCancelRequest = b'u' as isize,
    SecurityTypeRequest = b'v' as isize,
    SecurityTypes = b'w' as isize,
    SecurityListRequest = b'x' as isize,
    SecurityList = b'y' as isize,
    DerivativeSecurityListRequest = b'z' as isize,
    DerivativeSecurityList,
    NewOrderAb,
    MultilegOrderCancelReplace,
    TradeCaptureReportRequest,
    TradeCaptureReport,
    OrderMassStatusRequest,
    QuoteRequestReject,
    RfqRequest,
    QuoteStatusReport,
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
            b"D" => Some(MsgType::OrderSingle),
            b"E" => Some(MsgType::OrderList),
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
            b"V" => Some(MsgType::MarketDataRequest),
            b"W" => Some(MsgType::MarketDataSnapshotFullRefresh),
            b"X" => Some(MsgType::MarketDataIncrementalRefresh),
            b"Y" => Some(MsgType::MarketDataRequestReject),
            b"Z" => Some(MsgType::QuoteCancel),
            b"a" => Some(MsgType::QuoteStatusRequest),
            b"b" => Some(MsgType::MassQuoteAcknowledgement),
            b"c" => Some(MsgType::SecurityDefinitionRequest),
            b"d" => Some(MsgType::SecurityDefinition),
            b"e" => Some(MsgType::SecurityStatusRequest),
            b"f" => Some(MsgType::SecurityStatus),
            b"g" => Some(MsgType::TradingSessionStatusRequest),
            b"h" => Some(MsgType::TradingSessionStatus),
            b"i" => Some(MsgType::MassQuote),
            b"j" => Some(MsgType::BusinessMessageReject),
            b"k" => Some(MsgType::BidRequest),
            b"l" => Some(MsgType::BidResponse),
            b"m" => Some(MsgType::ListStrikePrice),
            b"n" => Some(MsgType::XmlMessage),
            b"o" => Some(MsgType::RegistrationInstructions),
            b"p" => Some(MsgType::RegistrationInstructionsResponse),
            b"q" => Some(MsgType::OrderMassCancelRequest),
            b"r" => Some(MsgType::OrderMassCancelReport),
            b"s" => Some(MsgType::NewOrderS),
            b"t" => Some(MsgType::CrossOrderCancelReplaceRequest),
            b"u" => Some(MsgType::CrossOrderCancelRequest),
            b"v" => Some(MsgType::SecurityTypeRequest),
            b"w" => Some(MsgType::SecurityTypes),
            b"x" => Some(MsgType::SecurityListRequest),
            b"y" => Some(MsgType::SecurityList),
            b"z" => Some(MsgType::DerivativeSecurityListRequest),
            b"AA" => Some(MsgType::DerivativeSecurityList),
            b"AB" => Some(MsgType::NewOrderAb),
            b"AC" => Some(MsgType::MultilegOrderCancelReplace),
            b"AD" => Some(MsgType::TradeCaptureReportRequest),
            b"AE" => Some(MsgType::TradeCaptureReport),
            b"AF" => Some(MsgType::OrderMassStatusRequest),
            b"AG" => Some(MsgType::QuoteRequestReject),
            b"AH" => Some(MsgType::RfqRequest),
            b"AI" => Some(MsgType::QuoteStatusReport),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            MsgType::DerivativeSecurityList => v.extend(b"AA"),
            MsgType::NewOrderAb => v.extend(b"AB"),
            MsgType::MultilegOrderCancelReplace => v.extend(b"AC"),
            MsgType::TradeCaptureReportRequest => v.extend(b"AD"),
            MsgType::TradeCaptureReport => v.extend(b"AE"),
            MsgType::OrderMassStatusRequest => v.extend(b"AF"),
            MsgType::QuoteRequestReject => v.extend(b"AG"),
            MsgType::RfqRequest => v.extend(b"AH"),
            MsgType::QuoteStatusReport => v.extend(b"AI"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenCloseSettleFlag {
    SessionOpen = b'1' as isize,
    DeliverySettlementPrice = b'2' as isize,
    ExpectedPrice = b'3' as isize,
    PriceFromPreviousBusinessDay = b'4' as isize,
    DailyOpen = b'0' as isize,
}

impl FIXValue for OpenCloseSettleFlag {
    fn from_bytes(bytes: &[u8]) -> Option<OpenCloseSettleFlag> {
        match bytes {
            b"1" => Some(OpenCloseSettleFlag::SessionOpen),
            b"2" => Some(OpenCloseSettleFlag::DeliverySettlementPrice),
            b"3" => Some(OpenCloseSettleFlag::ExpectedPrice),
            b"4" => Some(OpenCloseSettleFlag::PriceFromPreviousBusinessDay),
            b"0" => Some(OpenCloseSettleFlag::DailyOpen),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdRejReason {
    Broker = b'0' as isize,
    UnknownSymbol = b'1' as isize,
    ExchangeClosed = b'2' as isize,
    OrderExceedsLimit = b'3' as isize,
    TooLateToEnter = b'4' as isize,
    UnknownOrder = b'5' as isize,
    DuplicateOrder = b'6' as isize,
    DuplicateOfAVerballyCommunicatedOrder = b'7' as isize,
    StaleOrder = b'8' as isize,
    TradeAlongRequired = b'9' as isize,
    InvalidInvestorId,
    UnsupportedOrderCharacteristic,
    SurveillenceOption,
}

impl FIXValue for OrdRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<OrdRejReason> {
        match bytes {
            b"0" => Some(OrdRejReason::Broker),
            b"1" => Some(OrdRejReason::UnknownSymbol),
            b"2" => Some(OrdRejReason::ExchangeClosed),
            b"3" => Some(OrdRejReason::OrderExceedsLimit),
            b"4" => Some(OrdRejReason::TooLateToEnter),
            b"5" => Some(OrdRejReason::UnknownOrder),
            b"6" => Some(OrdRejReason::DuplicateOrder),
            b"7" => Some(OrdRejReason::DuplicateOfAVerballyCommunicatedOrder),
            b"8" => Some(OrdRejReason::StaleOrder),
            b"9" => Some(OrdRejReason::TradeAlongRequired),
            b"10" => Some(OrdRejReason::InvalidInvestorId),
            b"11" => Some(OrdRejReason::UnsupportedOrderCharacteristic),
            b"12" => Some(OrdRejReason::SurveillenceOption),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            OrdRejReason::InvalidInvestorId => v.extend(b"10"),
            OrdRejReason::UnsupportedOrderCharacteristic => v.extend(b"11"),
            OrdRejReason::SurveillenceOption => v.extend(b"12"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdStatus {
    New = b'0' as isize,
    PartiallyFilled = b'1' as isize,
    Replaced = b'5' as isize,
    Filled = b'2' as isize,
    PendingCancel = b'6' as isize,
    Stopped = b'7' as isize,
    Rejected = b'8' as isize,
    Suspended = b'9' as isize,
    PendingNew = b'A' as isize,
    Calculated = b'B' as isize,
    Expired = b'C' as isize,
    AcceptedForBidding = b'D' as isize,
    PendingReplace = b'E' as isize,
    DoneForDay = b'3' as isize,
    Canceled = b'4' as isize,
}

impl FIXValue for OrdStatus {
    fn from_bytes(bytes: &[u8]) -> Option<OrdStatus> {
        match bytes {
            b"0" => Some(OrdStatus::New),
            b"1" => Some(OrdStatus::PartiallyFilled),
            b"5" => Some(OrdStatus::Replaced),
            b"2" => Some(OrdStatus::Filled),
            b"6" => Some(OrdStatus::PendingCancel),
            b"7" => Some(OrdStatus::Stopped),
            b"8" => Some(OrdStatus::Rejected),
            b"9" => Some(OrdStatus::Suspended),
            b"A" => Some(OrdStatus::PendingNew),
            b"B" => Some(OrdStatus::Calculated),
            b"C" => Some(OrdStatus::Expired),
            b"D" => Some(OrdStatus::AcceptedForBidding),
            b"E" => Some(OrdStatus::PendingReplace),
            b"3" => Some(OrdStatus::DoneForDay),
            b"4" => Some(OrdStatus::Canceled),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdType {
    PreviouslyQuoted = b'D' as isize,
    Limit = b'2' as isize,
    Stop = b'3' as isize,
    StopLimit = b'4' as isize,
    MarketOnClose = b'5' as isize,
    WithOrWithout = b'6' as isize,
    LimitOrBetter = b'7' as isize,
    LimitWithOrWithout = b'8' as isize,
    OnBasis = b'9' as isize,
    OnClose = b'A' as isize,
    Market = b'1' as isize,
    ForexC = b'C' as isize,
    ForexF = b'F' as isize,
    PreviouslyIndicated = b'E' as isize,
    ForexG = b'G' as isize,
    Funari = b'I' as isize,
    MarketIfTouched = b'J' as isize,
    MarketWithLeftoverAsLimit = b'K' as isize,
    PreviousFundValuationPoint = b'L' as isize,
    NextFundValuationPoint = b'M' as isize,
    Pegged = b'P' as isize,
    LimitOnClose = b'B' as isize,
    ForexH = b'H' as isize,
}

impl FIXValue for OrdType {
    fn from_bytes(bytes: &[u8]) -> Option<OrdType> {
        match bytes {
            b"D" => Some(OrdType::PreviouslyQuoted),
            b"2" => Some(OrdType::Limit),
            b"3" => Some(OrdType::Stop),
            b"4" => Some(OrdType::StopLimit),
            b"5" => Some(OrdType::MarketOnClose),
            b"6" => Some(OrdType::WithOrWithout),
            b"7" => Some(OrdType::LimitOrBetter),
            b"8" => Some(OrdType::LimitWithOrWithout),
            b"9" => Some(OrdType::OnBasis),
            b"A" => Some(OrdType::OnClose),
            b"1" => Some(OrdType::Market),
            b"C" => Some(OrdType::ForexC),
            b"F" => Some(OrdType::ForexF),
            b"E" => Some(OrdType::PreviouslyIndicated),
            b"G" => Some(OrdType::ForexG),
            b"I" => Some(OrdType::Funari),
            b"J" => Some(OrdType::MarketIfTouched),
            b"K" => Some(OrdType::MarketWithLeftoverAsLimit),
            b"L" => Some(OrdType::PreviousFundValuationPoint),
            b"M" => Some(OrdType::NextFundValuationPoint),
            b"P" => Some(OrdType::Pegged),
            b"B" => Some(OrdType::LimitOnClose),
            b"H" => Some(OrdType::ForexH),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyIDSource {
    ChineseBShare = b'5' as isize,
    UsEmployerIdentificationNumber = b'8' as isize,
    AustralianTaxFileNumber = b'A' as isize,
    AustralianBusinessNumber = b'9' as isize,
    IsoCountryCode = b'E' as isize,
    Bic = b'B' as isize,
    UsSocialSecurityNumber = b'7' as isize,
    ProprietaryCustomCode = b'D' as isize,
    SettlementEntityLocation = b'F' as isize,
    KoreanInvestorId = b'1' as isize,
    TaiwaneseQualifiedForeignInvestorIdQfii = b'2' as isize,
    TaiwaneseTradingAccount = b'3' as isize,
    MalaysianCentralDepository = b'4' as isize,
    UkNationalInsuranceOrPensionNumber = b'6' as isize,
    GenerallyAcceptedMarketParticipantIdentifier = b'C' as isize,
}

impl FIXValue for PartyIDSource {
    fn from_bytes(bytes: &[u8]) -> Option<PartyIDSource> {
        match bytes {
            b"5" => Some(PartyIDSource::ChineseBShare),
            b"8" => Some(PartyIDSource::UsEmployerIdentificationNumber),
            b"A" => Some(PartyIDSource::AustralianTaxFileNumber),
            b"9" => Some(PartyIDSource::AustralianBusinessNumber),
            b"E" => Some(PartyIDSource::IsoCountryCode),
            b"B" => Some(PartyIDSource::Bic),
            b"7" => Some(PartyIDSource::UsSocialSecurityNumber),
            b"D" => Some(PartyIDSource::ProprietaryCustomCode),
            b"F" => Some(PartyIDSource::SettlementEntityLocation),
            b"1" => Some(PartyIDSource::KoreanInvestorId),
            b"2" => Some(PartyIDSource::TaiwaneseQualifiedForeignInvestorIdQfii),
            b"3" => Some(PartyIDSource::TaiwaneseTradingAccount),
            b"4" => Some(PartyIDSource::MalaysianCentralDepository),
            b"6" => Some(PartyIDSource::UkNationalInsuranceOrPensionNumber),
            b"C" => Some(PartyIDSource::GenerallyAcceptedMarketParticipantIdentifier),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyRole {
    ExecutingFirm = b'1' as isize,
    BrokerOfCredit = b'2' as isize,
    ClientId = b'3' as isize,
    ClearingFirm = b'4' as isize,
    InvestorId = b'5' as isize,
    IntroducingFirm = b'6' as isize,
    EnteringFirm = b'7' as isize,
    LocateLendingFirm = b'8' as isize,
    FundManagerClientId = b'9' as isize,
    SettlementLocation,
    OrderOriginationTrader,
    ExecutingTrader,
    OrderOriginationFirm,
    GiveupClearingFirm,
    CorrespondantClearingFirm,
    ExecutingSystem,
    ContraFirm,
    ContraClearingFirm,
    SponsoringFirm,
    UnderlyingContraFirm,
}

impl FIXValue for PartyRole {
    fn from_bytes(bytes: &[u8]) -> Option<PartyRole> {
        match bytes {
            b"1" => Some(PartyRole::ExecutingFirm),
            b"2" => Some(PartyRole::BrokerOfCredit),
            b"3" => Some(PartyRole::ClientId),
            b"4" => Some(PartyRole::ClearingFirm),
            b"5" => Some(PartyRole::InvestorId),
            b"6" => Some(PartyRole::IntroducingFirm),
            b"7" => Some(PartyRole::EnteringFirm),
            b"8" => Some(PartyRole::LocateLendingFirm),
            b"9" => Some(PartyRole::FundManagerClientId),
            b"10" => Some(PartyRole::SettlementLocation),
            b"11" => Some(PartyRole::OrderOriginationTrader),
            b"12" => Some(PartyRole::ExecutingTrader),
            b"13" => Some(PartyRole::OrderOriginationFirm),
            b"14" => Some(PartyRole::GiveupClearingFirm),
            b"15" => Some(PartyRole::CorrespondantClearingFirm),
            b"16" => Some(PartyRole::ExecutingSystem),
            b"17" => Some(PartyRole::ContraFirm),
            b"18" => Some(PartyRole::ContraClearingFirm),
            b"19" => Some(PartyRole::SponsoringFirm),
            b"20" => Some(PartyRole::UnderlyingContraFirm),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            PartyRole::SettlementLocation => v.extend(b"10"),
            PartyRole::OrderOriginationTrader => v.extend(b"11"),
            PartyRole::ExecutingTrader => v.extend(b"12"),
            PartyRole::OrderOriginationFirm => v.extend(b"13"),
            PartyRole::GiveupClearingFirm => v.extend(b"14"),
            PartyRole::CorrespondantClearingFirm => v.extend(b"15"),
            PartyRole::ExecutingSystem => v.extend(b"16"),
            PartyRole::ContraFirm => v.extend(b"17"),
            PartyRole::ContraClearingFirm => v.extend(b"18"),
            PartyRole::SponsoringFirm => v.extend(b"19"),
            PartyRole::UnderlyingContraFirm => v.extend(b"20"),
            _ => v.push(*self as u8)
        }
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
    FixedAmount = b'3' as isize,
    Percentage = b'1' as isize,
    Discount = b'4' as isize,
    BasisPointsRelativeToBenchmark = b'6' as isize,
    TedPrice = b'7' as isize,
    TedYield = b'8' as isize,
    Premium = b'5' as isize,
    PerShare = b'2' as isize,
}

impl FIXValue for PriceType {
    fn from_bytes(bytes: &[u8]) -> Option<PriceType> {
        match bytes {
            b"3" => Some(PriceType::FixedAmount),
            b"1" => Some(PriceType::Percentage),
            b"4" => Some(PriceType::Discount),
            b"6" => Some(PriceType::BasisPointsRelativeToBenchmark),
            b"7" => Some(PriceType::TedPrice),
            b"8" => Some(PriceType::TedYield),
            b"5" => Some(PriceType::Premium),
            b"2" => Some(PriceType::PerShare),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Product {
    Agency = b'1' as isize,
    Commodity = b'2' as isize,
    Corporate = b'3' as isize,
    Currency = b'4' as isize,
    Equity = b'5' as isize,
    Government = b'6' as isize,
    Index = b'7' as isize,
    Loan = b'8' as isize,
    Moneymarket = b'9' as isize,
    Mortgage,
    Municipal,
    Other,
}

impl FIXValue for Product {
    fn from_bytes(bytes: &[u8]) -> Option<Product> {
        match bytes {
            b"1" => Some(Product::Agency),
            b"2" => Some(Product::Commodity),
            b"3" => Some(Product::Corporate),
            b"4" => Some(Product::Currency),
            b"5" => Some(Product::Equity),
            b"6" => Some(Product::Government),
            b"7" => Some(Product::Index),
            b"8" => Some(Product::Loan),
            b"9" => Some(Product::Moneymarket),
            b"10" => Some(Product::Mortgage),
            b"11" => Some(Product::Municipal),
            b"12" => Some(Product::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            Product::Mortgage => v.extend(b"10"),
            Product::Municipal => v.extend(b"11"),
            Product::Other => v.extend(b"12"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityType {
    Contracts = b'6' as isize,
    Other = b'7' as isize,
    Currency = b'5' as isize,
    Originalface = b'4' as isize,
    Currentface = b'3' as isize,
    Bonds = b'2' as isize,
    Shares = b'1' as isize,
    Par = b'8' as isize,
}

impl FIXValue for QuantityType {
    fn from_bytes(bytes: &[u8]) -> Option<QuantityType> {
        match bytes {
            b"6" => Some(QuantityType::Contracts),
            b"7" => Some(QuantityType::Other),
            b"5" => Some(QuantityType::Currency),
            b"4" => Some(QuantityType::Originalface),
            b"3" => Some(QuantityType::Currentface),
            b"2" => Some(QuantityType::Bonds),
            b"1" => Some(QuantityType::Shares),
            b"8" => Some(QuantityType::Par),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteCancelType {
    CancelAllQuotes = b'4' as isize,
    CancelForSecurityType = b'2' as isize,
    CancelForSymbol = b'1' as isize,
    CancelForUnderlyingSymbol = b'3' as isize,
}

impl FIXValue for QuoteCancelType {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteCancelType> {
        match bytes {
            b"4" => Some(QuoteCancelType::CancelAllQuotes),
            b"2" => Some(QuoteCancelType::CancelForSecurityType),
            b"1" => Some(QuoteCancelType::CancelForSymbol),
            b"3" => Some(QuoteCancelType::CancelForUnderlyingSymbol),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteRejectReason {
    NotAuthorizedToQuoteSecurity = b'9' as isize,
    UnknownSymbol = b'1' as isize,
    Exchange = b'2' as isize,
    QuoteRequestExceedsLimit = b'3' as isize,
    TooLateToEnter = b'4' as isize,
    UnknownQuote = b'5' as isize,
    DuplicateQuote = b'6' as isize,
    InvalidBidAskSpread = b'7' as isize,
    InvalidPrice = b'8' as isize,
}

impl FIXValue for QuoteRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteRejectReason> {
        match bytes {
            b"9" => Some(QuoteRejectReason::NotAuthorizedToQuoteSecurity),
            b"1" => Some(QuoteRejectReason::UnknownSymbol),
            b"2" => Some(QuoteRejectReason::Exchange),
            b"3" => Some(QuoteRejectReason::QuoteRequestExceedsLimit),
            b"4" => Some(QuoteRejectReason::TooLateToEnter),
            b"5" => Some(QuoteRejectReason::UnknownQuote),
            b"6" => Some(QuoteRejectReason::DuplicateQuote),
            b"7" => Some(QuoteRejectReason::InvalidBidAskSpread),
            b"8" => Some(QuoteRejectReason::InvalidPrice),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteRequestRejectReason {
    UnknownSymbol = b'1' as isize,
    Exchange = b'2' as isize,
    QuoteRequestExceedsLimit = b'3' as isize,
    TooLateToEnter = b'4' as isize,
    InvalidPrice = b'5' as isize,
    NotAuthorizedToRequestQuote = b'6' as isize,
}

impl FIXValue for QuoteRequestRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteRequestRejectReason> {
        match bytes {
            b"1" => Some(QuoteRequestRejectReason::UnknownSymbol),
            b"2" => Some(QuoteRequestRejectReason::Exchange),
            b"3" => Some(QuoteRequestRejectReason::QuoteRequestExceedsLimit),
            b"4" => Some(QuoteRequestRejectReason::TooLateToEnter),
            b"5" => Some(QuoteRequestRejectReason::InvalidPrice),
            b"6" => Some(QuoteRequestRejectReason::NotAuthorizedToRequestQuote),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteStatus {
    Accepted = b'0' as isize,
    CanceledForSymbol = b'1' as isize,
    CanceledForSecurityType = b'2' as isize,
    CanceledForUnderlying = b'3' as isize,
    CanceledAll = b'4' as isize,
    Rejected = b'5' as isize,
    RemovedFromMarket = b'6' as isize,
    Expired = b'7' as isize,
    Query = b'8' as isize,
    QuoteNotFound = b'9' as isize,
    Pending,
}

impl FIXValue for QuoteStatus {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteStatus> {
        match bytes {
            b"0" => Some(QuoteStatus::Accepted),
            b"1" => Some(QuoteStatus::CanceledForSymbol),
            b"2" => Some(QuoteStatus::CanceledForSecurityType),
            b"3" => Some(QuoteStatus::CanceledForUnderlying),
            b"4" => Some(QuoteStatus::CanceledAll),
            b"5" => Some(QuoteStatus::Rejected),
            b"6" => Some(QuoteStatus::RemovedFromMarket),
            b"7" => Some(QuoteStatus::Expired),
            b"8" => Some(QuoteStatus::Query),
            b"9" => Some(QuoteStatus::QuoteNotFound),
            b"10" => Some(QuoteStatus::Pending),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            QuoteStatus::Pending => v.extend(b"10"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteType {
    Indicative = b'0' as isize,
    Tradeable = b'1' as isize,
    RestrictedTradeable = b'2' as isize,
}

impl FIXValue for QuoteType {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteType> {
        match bytes {
            b"0" => Some(QuoteType::Indicative),
            b"1" => Some(QuoteType::Tradeable),
            b"2" => Some(QuoteType::RestrictedTradeable),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistRejReasonCode {
    InvalidUnacceptableAccountType = b'1' as isize,
    InvalidUnacceptableTaxExemptType = b'2' as isize,
    InvalidUnacceptableOwnershipType = b'3' as isize,
    InvalidUnacceptableNoRegDetls = b'4' as isize,
    InvalidUnacceptableRegSeqNo = b'5' as isize,
    InvalidUnacceptableRegDtls = b'6' as isize,
    InvalidUnacceptableMailingDtls = b'7' as isize,
    InvalidUnacceptableMailingInst = b'8' as isize,
    InvalidUnacceptableInvestorId = b'9' as isize,
    InvalidUnacceptableInvestorIdSource,
    InvalidUnacceptableDateOfBirth,
    InvalidUnacceptableInvestorCountryOfResidence,
    InvalidUnacceptableNodistribinstns,
    InvalidUnacceptableDistribPercentage,
    InvalidUnacceptableDistribPaymentMethod,
    InvalidUnacceptableCashDistribAgentAcctName,
    InvalidUnacceptableCashDistribAgentCode,
    InvalidUnacceptableCashDistribAgentAcctNum,
}

impl FIXValue for RegistRejReasonCode {
    fn from_bytes(bytes: &[u8]) -> Option<RegistRejReasonCode> {
        match bytes {
            b"1" => Some(RegistRejReasonCode::InvalidUnacceptableAccountType),
            b"2" => Some(RegistRejReasonCode::InvalidUnacceptableTaxExemptType),
            b"3" => Some(RegistRejReasonCode::InvalidUnacceptableOwnershipType),
            b"4" => Some(RegistRejReasonCode::InvalidUnacceptableNoRegDetls),
            b"5" => Some(RegistRejReasonCode::InvalidUnacceptableRegSeqNo),
            b"6" => Some(RegistRejReasonCode::InvalidUnacceptableRegDtls),
            b"7" => Some(RegistRejReasonCode::InvalidUnacceptableMailingDtls),
            b"8" => Some(RegistRejReasonCode::InvalidUnacceptableMailingInst),
            b"9" => Some(RegistRejReasonCode::InvalidUnacceptableInvestorId),
            b"10" => Some(RegistRejReasonCode::InvalidUnacceptableInvestorIdSource),
            b"11" => Some(RegistRejReasonCode::InvalidUnacceptableDateOfBirth),
            b"12" => Some(RegistRejReasonCode::InvalidUnacceptableInvestorCountryOfResidence),
            b"13" => Some(RegistRejReasonCode::InvalidUnacceptableNodistribinstns),
            b"14" => Some(RegistRejReasonCode::InvalidUnacceptableDistribPercentage),
            b"15" => Some(RegistRejReasonCode::InvalidUnacceptableDistribPaymentMethod),
            b"16" => Some(RegistRejReasonCode::InvalidUnacceptableCashDistribAgentAcctName),
            b"17" => Some(RegistRejReasonCode::InvalidUnacceptableCashDistribAgentCode),
            b"18" => Some(RegistRejReasonCode::InvalidUnacceptableCashDistribAgentAcctNum),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            RegistRejReasonCode::InvalidUnacceptableInvestorIdSource => v.extend(b"10"),
            RegistRejReasonCode::InvalidUnacceptableDateOfBirth => v.extend(b"11"),
            RegistRejReasonCode::InvalidUnacceptableInvestorCountryOfResidence => v.extend(b"12"),
            RegistRejReasonCode::InvalidUnacceptableNodistribinstns => v.extend(b"13"),
            RegistRejReasonCode::InvalidUnacceptableDistribPercentage => v.extend(b"14"),
            RegistRejReasonCode::InvalidUnacceptableDistribPaymentMethod => v.extend(b"15"),
            RegistRejReasonCode::InvalidUnacceptableCashDistribAgentAcctName => v.extend(b"16"),
            RegistRejReasonCode::InvalidUnacceptableCashDistribAgentCode => v.extend(b"17"),
            RegistRejReasonCode::InvalidUnacceptableCashDistribAgentAcctNum => v.extend(b"18"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistStatus {
    Accept = b'A' as isize,
    Reminder = b'N' as isize,
    Reject = b'R' as isize,
    Held = b'H' as isize,
}

impl FIXValue for RegistStatus {
    fn from_bytes(bytes: &[u8]) -> Option<RegistStatus> {
        match bytes {
            b"A" => Some(RegistStatus::Accept),
            b"N" => Some(RegistStatus::Reminder),
            b"R" => Some(RegistStatus::Reject),
            b"H" => Some(RegistStatus::Held),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule80A {
    ProgramOrderNonIndexArbForOtherMember = b'N' as isize,
    ShortExemptTransactionB = b'B' as isize,
    ProgramOrderIndexArbForMemberFirmOrg = b'D' as isize,
    ShortExemptTransactionForPrincipal = b'E' as isize,
    ShortExemptTransactionF = b'F' as isize,
    ShortExemptTransactionH = b'H' as isize,
    IndividualInvestorSingleOrder = b'I' as isize,
    ProgramOrderIndexArbForIndividualCustomer = b'J' as isize,
    ProgramOrderNonIndexArbForIndividualCustomer = b'K' as isize,
    ProgramOrderIndexArbForOtherMember = b'M' as isize,
    AgencySingleOrder = b'A' as isize,
    ProprietaryTransactionsForCompetingMarketMakerThatIsAffiliatedWithTheClearingMember = b'O' as isize,
    Principal = b'P' as isize,
    TransactionsForTheAccountOfANonMemberCompetingMarketMaker = b'R' as isize,
    SpecialistTrades = b'S' as isize,
    TransactionsForTheAccountOfAnUnaffiliatedMembersCompetingMarketMaker = b'T' as isize,
    ProgramOrderIndexArbForOtherAgency = b'U' as isize,
    AllOtherOrdersAsAgentForOtherMember = b'W' as isize,
    ShortExemptTransactionForMemberCompetingMarketMakerNotAffiliatedWithTheFirmClearingTheTrade = b'X' as isize,
    ProgramOrderNonIndexArbForOtherAgency = b'Y' as isize,
    ShortExemptTransactionForNonMemberCompetingMarketMaker = b'Z' as isize,
    ShortExemptTransactionForMemberCompetingMarketMakerAffiliatedWithTheFirmClearingTheTrade = b'L' as isize,
    ProgramOrderNonIndexArbForMemberFirmOrg = b'C' as isize,
}

impl FIXValue for Rule80A {
    fn from_bytes(bytes: &[u8]) -> Option<Rule80A> {
        match bytes {
            b"N" => Some(Rule80A::ProgramOrderNonIndexArbForOtherMember),
            b"B" => Some(Rule80A::ShortExemptTransactionB),
            b"D" => Some(Rule80A::ProgramOrderIndexArbForMemberFirmOrg),
            b"E" => Some(Rule80A::ShortExemptTransactionForPrincipal),
            b"F" => Some(Rule80A::ShortExemptTransactionF),
            b"H" => Some(Rule80A::ShortExemptTransactionH),
            b"I" => Some(Rule80A::IndividualInvestorSingleOrder),
            b"J" => Some(Rule80A::ProgramOrderIndexArbForIndividualCustomer),
            b"K" => Some(Rule80A::ProgramOrderNonIndexArbForIndividualCustomer),
            b"M" => Some(Rule80A::ProgramOrderIndexArbForOtherMember),
            b"A" => Some(Rule80A::AgencySingleOrder),
            b"O" => Some(Rule80A::ProprietaryTransactionsForCompetingMarketMakerThatIsAffiliatedWithTheClearingMember),
            b"P" => Some(Rule80A::Principal),
            b"R" => Some(Rule80A::TransactionsForTheAccountOfANonMemberCompetingMarketMaker),
            b"S" => Some(Rule80A::SpecialistTrades),
            b"T" => Some(Rule80A::TransactionsForTheAccountOfAnUnaffiliatedMembersCompetingMarketMaker),
            b"U" => Some(Rule80A::ProgramOrderIndexArbForOtherAgency),
            b"W" => Some(Rule80A::AllOtherOrdersAsAgentForOtherMember),
            b"X" => Some(Rule80A::ShortExemptTransactionForMemberCompetingMarketMakerNotAffiliatedWithTheFirmClearingTheTrade),
            b"Y" => Some(Rule80A::ProgramOrderNonIndexArbForOtherAgency),
            b"Z" => Some(Rule80A::ShortExemptTransactionForNonMemberCompetingMarketMaker),
            b"L" => Some(Rule80A::ShortExemptTransactionForMemberCompetingMarketMakerAffiliatedWithTheFirmClearingTheTrade),
            b"C" => Some(Rule80A::ProgramOrderNonIndexArbForMemberFirmOrg),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityIDSource {
    Sicovam = b'E' as isize,
    Sedol = b'2' as isize,
    Cusip = b'1' as isize,
    Quik = b'3' as isize,
    Belgian = b'F' as isize,
    Valoren = b'D' as isize,
    Dutch = b'C' as isize,
    Wertpapier = b'B' as isize,
    BloombergSymbol = b'A' as isize,
    ConsolidatedTapeAssociation = b'9' as isize,
    ExchangeSymbol = b'8' as isize,
    IsoCountryCode = b'7' as isize,
    IsoCurrencyCode = b'6' as isize,
    RicCode = b'5' as isize,
    IsinNumber = b'4' as isize,
    Common = b'G' as isize,
}

impl FIXValue for SecurityIDSource {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityIDSource> {
        match bytes {
            b"E" => Some(SecurityIDSource::Sicovam),
            b"2" => Some(SecurityIDSource::Sedol),
            b"1" => Some(SecurityIDSource::Cusip),
            b"3" => Some(SecurityIDSource::Quik),
            b"F" => Some(SecurityIDSource::Belgian),
            b"D" => Some(SecurityIDSource::Valoren),
            b"C" => Some(SecurityIDSource::Dutch),
            b"B" => Some(SecurityIDSource::Wertpapier),
            b"A" => Some(SecurityIDSource::BloombergSymbol),
            b"9" => Some(SecurityIDSource::ConsolidatedTapeAssociation),
            b"8" => Some(SecurityIDSource::ExchangeSymbol),
            b"7" => Some(SecurityIDSource::IsoCountryCode),
            b"6" => Some(SecurityIDSource::IsoCurrencyCode),
            b"5" => Some(SecurityIDSource::RicCode),
            b"4" => Some(SecurityIDSource::IsinNumber),
            b"G" => Some(SecurityIDSource::Common),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityResponseType {
    RejectSecurityProposal = b'5' as isize,
    AcceptSecurityProposalAsIs = b'1' as isize,
    CanNotMatchSelectionCriteria = b'6' as isize,
    AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage = b'2' as isize,
    ListOfSecuritiesReturnedPerRequest = b'4' as isize,
    ListOfSecurityTypesReturnedPerRequest = b'3' as isize,
}

impl FIXValue for SecurityResponseType {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityResponseType> {
        match bytes {
            b"5" => Some(SecurityResponseType::RejectSecurityProposal),
            b"1" => Some(SecurityResponseType::AcceptSecurityProposalAsIs),
            b"6" => Some(SecurityResponseType::CanNotMatchSelectionCriteria),
            b"2" => Some(SecurityResponseType::AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage),
            b"4" => Some(SecurityResponseType::ListOfSecuritiesReturnedPerRequest),
            b"3" => Some(SecurityResponseType::ListOfSecurityTypesReturnedPerRequest),
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
    PreOpen,
    OpeningRotation,
    FastMarket,
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
            b"21" => Some(SecurityTradingStatus::PreOpen),
            b"22" => Some(SecurityTradingStatus::OpeningRotation),
            b"23" => Some(SecurityTradingStatus::FastMarket),
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
            SecurityTradingStatus::PreOpen => v.extend(b"21"),
            SecurityTradingStatus::OpeningRotation => v.extend(b"22"),
            SecurityTradingStatus::FastMarket => v.extend(b"23"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityType {
    WildcardEntry = b'?' as isize,
    AssetBackedSecurities,
    AmendedRestated,
    OtherAnticipationNotesBanGanEtc,
    BankersAcceptance,
    BankNotes,
    BillOfExchanges,
    BradyBond,
    BridgeLoan,
    ConvertibleBond,
    CertificateOfDeposit,
    CallLoans,
    CorpMortgageBackedSecurities,
    CollateralizedMortgageObligation,
    CertificateOfObligation,
    CertificateOfParticipation,
    CorporateBond,
    CommercialPaper,
    CorporatePrivatePlacement,
    CommonStock,
    Defaulted,
    DebtorInPossession,
    DepositNotes,
    DualCurrency,
    FederalAgencyCoupon,
    FederalAgencyDiscountNote,
    ForeignExchangeContract,
    GeneralObligationBonds,
    IoetteMortgage,
    LetterOfCredit,
    LiquidityNote,
    Matured,
    MortgageBackedSecurities,
    MutualFund,
    MortgageInterestOnly,
    MultiLegInstrument,
    MortgagePrincipalOnly,
    MortgagePrivatePlacement,
    MiscellaneousPassThrough,
    MandatoryTender,
    MediumTermNotes,
    NoSecurityType,
    Overnight,
    PrivateExportFunding,
    PromissoryNote,
    AgencyPools,
    PreferredStock,
    PlazosFijos,
    RevenueAnticipationNote,
    Replaced,
    Retired,
    RevenueBonds,
    RepurchaseAgreement,
    RevolverLoan,
    RevolverTermLoan,
    ReverseRepurchaseAgreement,
    SpecialAssessment,
    SpecialObligation,
    SpecialTax,
    ShortTermLoanNote,
    StructuredNotes,
    SwingLineFacility,
    TaxAnticipationNote,
    TaxAllocation,
    ToBeAnnounced,
    UsTreasuryBond,
    PrincipalStripOfACallableBondOrNote,
    TimeDeposit,
    TaxExemptCommercialPaper,
    TermLoan,
    InterestStripFromAnyBondOrNote,
    TreasuryInflationProtectedSecurities,
    PrincipalStripFromANonCallableBondOrNote,
    TaxRevenueAnticipationNote,
    UsTreasuryNoteBond,
    UsTreasuryBill,
    VariableRateDemandNote,
    Warrant,
    Withdrawn,
    ExtendedCommNote,
    IndexedLinked,
    YankeeCorporateBond,
}

impl FIXValue for SecurityType {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityType> {
        match bytes {
            b"?" => Some(SecurityType::WildcardEntry),
            b"ABS" => Some(SecurityType::AssetBackedSecurities),
            b"AMENDED" => Some(SecurityType::AmendedRestated),
            b"AN" => Some(SecurityType::OtherAnticipationNotesBanGanEtc),
            b"BA" => Some(SecurityType::BankersAcceptance),
            b"BN" => Some(SecurityType::BankNotes),
            b"BOX" => Some(SecurityType::BillOfExchanges),
            b"BRADY" => Some(SecurityType::BradyBond),
            b"BRIDGE" => Some(SecurityType::BridgeLoan),
            b"CB" => Some(SecurityType::ConvertibleBond),
            b"CD" => Some(SecurityType::CertificateOfDeposit),
            b"CL" => Some(SecurityType::CallLoans),
            b"CMBS" => Some(SecurityType::CorpMortgageBackedSecurities),
            b"CMO" => Some(SecurityType::CollateralizedMortgageObligation),
            b"COFO" => Some(SecurityType::CertificateOfObligation),
            b"COFP" => Some(SecurityType::CertificateOfParticipation),
            b"CORP" => Some(SecurityType::CorporateBond),
            b"CP" => Some(SecurityType::CommercialPaper),
            b"CPP" => Some(SecurityType::CorporatePrivatePlacement),
            b"CS" => Some(SecurityType::CommonStock),
            b"DEFLTED" => Some(SecurityType::Defaulted),
            b"DINP" => Some(SecurityType::DebtorInPossession),
            b"DN" => Some(SecurityType::DepositNotes),
            b"DUAL" => Some(SecurityType::DualCurrency),
            b"FAC" => Some(SecurityType::FederalAgencyCoupon),
            b"FADN" => Some(SecurityType::FederalAgencyDiscountNote),
            b"FOR" => Some(SecurityType::ForeignExchangeContract),
            b"GO" => Some(SecurityType::GeneralObligationBonds),
            b"IET" => Some(SecurityType::IoetteMortgage),
            b"LOFC" => Some(SecurityType::LetterOfCredit),
            b"LQN" => Some(SecurityType::LiquidityNote),
            b"MATURED" => Some(SecurityType::Matured),
            b"MBS" => Some(SecurityType::MortgageBackedSecurities),
            b"MF" => Some(SecurityType::MutualFund),
            b"MIO" => Some(SecurityType::MortgageInterestOnly),
            b"MLEG" => Some(SecurityType::MultiLegInstrument),
            b"MPO" => Some(SecurityType::MortgagePrincipalOnly),
            b"MPP" => Some(SecurityType::MortgagePrivatePlacement),
            b"MPT" => Some(SecurityType::MiscellaneousPassThrough),
            b"MT" => Some(SecurityType::MandatoryTender),
            b"MTN" => Some(SecurityType::MediumTermNotes),
            b"NONE" => Some(SecurityType::NoSecurityType),
            b"ONITE" => Some(SecurityType::Overnight),
            b"PEF" => Some(SecurityType::PrivateExportFunding),
            b"PN" => Some(SecurityType::PromissoryNote),
            b"POOL" => Some(SecurityType::AgencyPools),
            b"PS" => Some(SecurityType::PreferredStock),
            b"PZFJ" => Some(SecurityType::PlazosFijos),
            b"RAN" => Some(SecurityType::RevenueAnticipationNote),
            b"REPLACD" => Some(SecurityType::Replaced),
            b"RETIRED" => Some(SecurityType::Retired),
            b"REV" => Some(SecurityType::RevenueBonds),
            b"RP" => Some(SecurityType::RepurchaseAgreement),
            b"RVLV" => Some(SecurityType::RevolverLoan),
            b"RVLVTRM" => Some(SecurityType::RevolverTermLoan),
            b"RVRP" => Some(SecurityType::ReverseRepurchaseAgreement),
            b"SPCLA" => Some(SecurityType::SpecialAssessment),
            b"SPCLO" => Some(SecurityType::SpecialObligation),
            b"SPCLT" => Some(SecurityType::SpecialTax),
            b"STN" => Some(SecurityType::ShortTermLoanNote),
            b"STRUCT" => Some(SecurityType::StructuredNotes),
            b"SWING" => Some(SecurityType::SwingLineFacility),
            b"TAN" => Some(SecurityType::TaxAnticipationNote),
            b"TAXA" => Some(SecurityType::TaxAllocation),
            b"TBA" => Some(SecurityType::ToBeAnnounced),
            b"TBOND" => Some(SecurityType::UsTreasuryBond),
            b"TCAL" => Some(SecurityType::PrincipalStripOfACallableBondOrNote),
            b"TD" => Some(SecurityType::TimeDeposit),
            b"TECP" => Some(SecurityType::TaxExemptCommercialPaper),
            b"TERM" => Some(SecurityType::TermLoan),
            b"TINT" => Some(SecurityType::InterestStripFromAnyBondOrNote),
            b"TIPS" => Some(SecurityType::TreasuryInflationProtectedSecurities),
            b"TPRN" => Some(SecurityType::PrincipalStripFromANonCallableBondOrNote),
            b"TRAN" => Some(SecurityType::TaxRevenueAnticipationNote),
            b"UST" => Some(SecurityType::UsTreasuryNoteBond),
            b"USTB" => Some(SecurityType::UsTreasuryBill),
            b"VRDN" => Some(SecurityType::VariableRateDemandNote),
            b"WAR" => Some(SecurityType::Warrant),
            b"WITHDRN" => Some(SecurityType::Withdrawn),
            b"XCN" => Some(SecurityType::ExtendedCommNote),
            b"XLINKD" => Some(SecurityType::IndexedLinked),
            b"YANK" => Some(SecurityType::YankeeCorporateBond),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            SecurityType::AssetBackedSecurities => v.extend(b"ABS"),
            SecurityType::AmendedRestated => v.extend(b"AMENDED"),
            SecurityType::OtherAnticipationNotesBanGanEtc => v.extend(b"AN"),
            SecurityType::BankersAcceptance => v.extend(b"BA"),
            SecurityType::BankNotes => v.extend(b"BN"),
            SecurityType::BillOfExchanges => v.extend(b"BOX"),
            SecurityType::BradyBond => v.extend(b"BRADY"),
            SecurityType::BridgeLoan => v.extend(b"BRIDGE"),
            SecurityType::ConvertibleBond => v.extend(b"CB"),
            SecurityType::CertificateOfDeposit => v.extend(b"CD"),
            SecurityType::CallLoans => v.extend(b"CL"),
            SecurityType::CorpMortgageBackedSecurities => v.extend(b"CMBS"),
            SecurityType::CollateralizedMortgageObligation => v.extend(b"CMO"),
            SecurityType::CertificateOfObligation => v.extend(b"COFO"),
            SecurityType::CertificateOfParticipation => v.extend(b"COFP"),
            SecurityType::CorporateBond => v.extend(b"CORP"),
            SecurityType::CommercialPaper => v.extend(b"CP"),
            SecurityType::CorporatePrivatePlacement => v.extend(b"CPP"),
            SecurityType::CommonStock => v.extend(b"CS"),
            SecurityType::Defaulted => v.extend(b"DEFLTED"),
            SecurityType::DebtorInPossession => v.extend(b"DINP"),
            SecurityType::DepositNotes => v.extend(b"DN"),
            SecurityType::DualCurrency => v.extend(b"DUAL"),
            SecurityType::FederalAgencyCoupon => v.extend(b"FAC"),
            SecurityType::FederalAgencyDiscountNote => v.extend(b"FADN"),
            SecurityType::ForeignExchangeContract => v.extend(b"FOR"),
            SecurityType::GeneralObligationBonds => v.extend(b"GO"),
            SecurityType::IoetteMortgage => v.extend(b"IET"),
            SecurityType::LetterOfCredit => v.extend(b"LOFC"),
            SecurityType::LiquidityNote => v.extend(b"LQN"),
            SecurityType::Matured => v.extend(b"MATURED"),
            SecurityType::MortgageBackedSecurities => v.extend(b"MBS"),
            SecurityType::MutualFund => v.extend(b"MF"),
            SecurityType::MortgageInterestOnly => v.extend(b"MIO"),
            SecurityType::MultiLegInstrument => v.extend(b"MLEG"),
            SecurityType::MortgagePrincipalOnly => v.extend(b"MPO"),
            SecurityType::MortgagePrivatePlacement => v.extend(b"MPP"),
            SecurityType::MiscellaneousPassThrough => v.extend(b"MPT"),
            SecurityType::MandatoryTender => v.extend(b"MT"),
            SecurityType::MediumTermNotes => v.extend(b"MTN"),
            SecurityType::NoSecurityType => v.extend(b"NONE"),
            SecurityType::Overnight => v.extend(b"ONITE"),
            SecurityType::PrivateExportFunding => v.extend(b"PEF"),
            SecurityType::PromissoryNote => v.extend(b"PN"),
            SecurityType::AgencyPools => v.extend(b"POOL"),
            SecurityType::PreferredStock => v.extend(b"PS"),
            SecurityType::PlazosFijos => v.extend(b"PZFJ"),
            SecurityType::RevenueAnticipationNote => v.extend(b"RAN"),
            SecurityType::Replaced => v.extend(b"REPLACD"),
            SecurityType::Retired => v.extend(b"RETIRED"),
            SecurityType::RevenueBonds => v.extend(b"REV"),
            SecurityType::RepurchaseAgreement => v.extend(b"RP"),
            SecurityType::RevolverLoan => v.extend(b"RVLV"),
            SecurityType::RevolverTermLoan => v.extend(b"RVLVTRM"),
            SecurityType::ReverseRepurchaseAgreement => v.extend(b"RVRP"),
            SecurityType::SpecialAssessment => v.extend(b"SPCLA"),
            SecurityType::SpecialObligation => v.extend(b"SPCLO"),
            SecurityType::SpecialTax => v.extend(b"SPCLT"),
            SecurityType::ShortTermLoanNote => v.extend(b"STN"),
            SecurityType::StructuredNotes => v.extend(b"STRUCT"),
            SecurityType::SwingLineFacility => v.extend(b"SWING"),
            SecurityType::TaxAnticipationNote => v.extend(b"TAN"),
            SecurityType::TaxAllocation => v.extend(b"TAXA"),
            SecurityType::ToBeAnnounced => v.extend(b"TBA"),
            SecurityType::UsTreasuryBond => v.extend(b"TBOND"),
            SecurityType::PrincipalStripOfACallableBondOrNote => v.extend(b"TCAL"),
            SecurityType::TimeDeposit => v.extend(b"TD"),
            SecurityType::TaxExemptCommercialPaper => v.extend(b"TECP"),
            SecurityType::TermLoan => v.extend(b"TERM"),
            SecurityType::InterestStripFromAnyBondOrNote => v.extend(b"TINT"),
            SecurityType::TreasuryInflationProtectedSecurities => v.extend(b"TIPS"),
            SecurityType::PrincipalStripFromANonCallableBondOrNote => v.extend(b"TPRN"),
            SecurityType::TaxRevenueAnticipationNote => v.extend(b"TRAN"),
            SecurityType::UsTreasuryNoteBond => v.extend(b"UST"),
            SecurityType::UsTreasuryBill => v.extend(b"USTB"),
            SecurityType::VariableRateDemandNote => v.extend(b"VRDN"),
            SecurityType::Warrant => v.extend(b"WAR"),
            SecurityType::Withdrawn => v.extend(b"WITHDRN"),
            SecurityType::ExtendedCommNote => v.extend(b"XCN"),
            SecurityType::IndexedLinked => v.extend(b"XLINKD"),
            SecurityType::YankeeCorporateBond => v.extend(b"YANK"),
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
    XmlValidationError,
    TagAppearsMoreThanOnce,
    TagSpecifiedOutOfRequiredOrder,
    RepeatingGroupFieldsOutOfOrder,
    IncorrectNumingroupCountForRepeatingGroup,
    NonDataValueIncludesFieldDelimiter,
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
            b"12" => Some(SessionRejectReason::XmlValidationError),
            b"13" => Some(SessionRejectReason::TagAppearsMoreThanOnce),
            b"14" => Some(SessionRejectReason::TagSpecifiedOutOfRequiredOrder),
            b"15" => Some(SessionRejectReason::RepeatingGroupFieldsOutOfOrder),
            b"16" => Some(SessionRejectReason::IncorrectNumingroupCountForRepeatingGroup),
            b"17" => Some(SessionRejectReason::NonDataValueIncludesFieldDelimiter),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            SessionRejectReason::SendingtimeAccuracyProblem => v.extend(b"10"),
            SessionRejectReason::InvalidMsgtype => v.extend(b"11"),
            SessionRejectReason::XmlValidationError => v.extend(b"12"),
            SessionRejectReason::TagAppearsMoreThanOnce => v.extend(b"13"),
            SessionRejectReason::TagSpecifiedOutOfRequiredOrder => v.extend(b"14"),
            SessionRejectReason::RepeatingGroupFieldsOutOfOrder => v.extend(b"15"),
            SessionRejectReason::IncorrectNumingroupCountForRepeatingGroup => v.extend(b"16"),
            SessionRejectReason::NonDataValueIncludesFieldDelimiter => v.extend(b"17"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlDeliveryType {
    Free = b'1' as isize,
    VersusPayment = b'0' as isize,
}

impl FIXValue for SettlDeliveryType {
    fn from_bytes(bytes: &[u8]) -> Option<SettlDeliveryType> {
        match bytes {
            b"1" => Some(SettlDeliveryType::Free),
            b"0" => Some(SettlDeliveryType::VersusPayment),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstMode {
    Default = b'0' as isize,
    SpecificOrderForASingleAccount = b'4' as isize,
    SpecificAllocationAccountStanding = b'3' as isize,
    StandingInstructionsProvided = b'1' as isize,
    SpecificAllocationAccountOverriding = b'2' as isize,
}

impl FIXValue for SettlInstMode {
    fn from_bytes(bytes: &[u8]) -> Option<SettlInstMode> {
        match bytes {
            b"0" => Some(SettlInstMode::Default),
            b"4" => Some(SettlInstMode::SpecificOrderForASingleAccount),
            b"3" => Some(SettlInstMode::SpecificAllocationAccountStanding),
            b"1" => Some(SettlInstMode::StandingInstructionsProvided),
            b"2" => Some(SettlInstMode::SpecificAllocationAccountOverriding),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstSource {
    InstitutionsInstructions = b'2' as isize,
    Investor = b'3' as isize,
    BrokersInstructions = b'1' as isize,
}

impl FIXValue for SettlInstSource {
    fn from_bytes(bytes: &[u8]) -> Option<SettlInstSource> {
        match bytes {
            b"2" => Some(SettlInstSource::InstitutionsInstructions),
            b"3" => Some(SettlInstSource::Investor),
            b"1" => Some(SettlInstSource::BrokersInstructions),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstTransType {
    New = b'N' as isize,
    Replace = b'R' as isize,
    Cancel = b'C' as isize,
}

impl FIXValue for SettlInstTransType {
    fn from_bytes(bytes: &[u8]) -> Option<SettlInstTransType> {
        match bytes {
            b"N" => Some(SettlInstTransType::New),
            b"R" => Some(SettlInstTransType::Replace),
            b"C" => Some(SettlInstTransType::Cancel),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlmntTyp {
    TPlus4 = b'5' as isize,
    TPlus1 = b'A' as isize,
    Future = b'6' as isize,
    TPlus2 = b'3' as isize,
    NextDay = b'2' as isize,
    SellersOption = b'8' as isize,
    Cash = b'1' as isize,
    WhenAndIfIssued = b'7' as isize,
    Regular = b'0' as isize,
    TPlus5 = b'9' as isize,
    TPlus3 = b'4' as isize,
}

impl FIXValue for SettlmntTyp {
    fn from_bytes(bytes: &[u8]) -> Option<SettlmntTyp> {
        match bytes {
            b"5" => Some(SettlmntTyp::TPlus4),
            b"A" => Some(SettlmntTyp::TPlus1),
            b"6" => Some(SettlmntTyp::Future),
            b"3" => Some(SettlmntTyp::TPlus2),
            b"2" => Some(SettlmntTyp::NextDay),
            b"8" => Some(SettlmntTyp::SellersOption),
            b"1" => Some(SettlmntTyp::Cash),
            b"7" => Some(SettlmntTyp::WhenAndIfIssued),
            b"0" => Some(SettlmntTyp::Regular),
            b"9" => Some(SettlmntTyp::TPlus5),
            b"4" => Some(SettlmntTyp::TPlus3),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    SellShortExempt = b'6' as isize,
    AsDefined = b'B' as isize,
    Opposite = b'C' as isize,
    Cross = b'8' as isize,
    CrossShort = b'9' as isize,
    Buy = b'1' as isize,
    Sell = b'2' as isize,
    BuyMinus = b'3' as isize,
    SellPlus = b'4' as isize,
    CrossShortExempt = b'A' as isize,
    SellShort = b'5' as isize,
    Undisclosed = b'7' as isize,
}

impl FIXValue for Side {
    fn from_bytes(bytes: &[u8]) -> Option<Side> {
        match bytes {
            b"6" => Some(Side::SellShortExempt),
            b"B" => Some(Side::AsDefined),
            b"C" => Some(Side::Opposite),
            b"8" => Some(Side::Cross),
            b"9" => Some(Side::CrossShort),
            b"1" => Some(Side::Buy),
            b"2" => Some(Side::Sell),
            b"3" => Some(Side::BuyMinus),
            b"4" => Some(Side::SellPlus),
            b"A" => Some(Side::CrossShortExempt),
            b"5" => Some(Side::SellShort),
            b"7" => Some(Side::Undisclosed),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideValueInd {
    Sidevalue1 = b'1' as isize,
    Sidevalue2 = b'2' as isize,
}

impl FIXValue for SideValueInd {
    fn from_bytes(bytes: &[u8]) -> Option<SideValueInd> {
        match bytes {
            b"1" => Some(SideValueInd::Sidevalue1),
            b"2" => Some(SideValueInd::Sidevalue2),
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
    AGlobalCustodian = b'3' as isize,
    ThomsonAlert = b'2' as isize,
}

impl FIXValue for StandInstDbType {
    fn from_bytes(bytes: &[u8]) -> Option<StandInstDbType> {
        match bytes {
            b"0" => Some(StandInstDbType::Other),
            b"1" => Some(StandInstDbType::DtcSid),
            b"3" => Some(StandInstDbType::AGlobalCustodian),
            b"2" => Some(StandInstDbType::ThomsonAlert),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StipulationType {
    AbsolutePrepaymentSpeed,
    ConstantPrepaymentPenalty,
    ConstantPrepaymentRate,
    ConstantPrepaymentYield,
    Geographics,
    FinalCprOfHomeEquityPrepaymentCurve,
    YearOfIssue,
    LotVariance,
    MaturityYear,
    OfManufacturedHousingPrepaymentCurve,
    MonthlyPrepaymentRate,
    NumberOfPieces,
    PoolsMaximum,
    OfProspectusPrepaymentCurve,
    PoolsPerLot,
    PoolsPerMillion,
    PoolsPerTrade,
    ProductionYear,
    OfBmaPrepaymentCurve,
    SingleMonthlyMortality,
    TradeVariance,
    WeightedAverageCoupon,
    WeightedAverageLife,
    WeightedAverageLoanAge,
    WeightedAverageMaturity,
}

impl FIXValue for StipulationType {
    fn from_bytes(bytes: &[u8]) -> Option<StipulationType> {
        match bytes {
            b"ABS" => Some(StipulationType::AbsolutePrepaymentSpeed),
            b"CPP" => Some(StipulationType::ConstantPrepaymentPenalty),
            b"CPR" => Some(StipulationType::ConstantPrepaymentRate),
            b"CPY" => Some(StipulationType::ConstantPrepaymentYield),
            b"GEOG" => Some(StipulationType::Geographics),
            b"HEP" => Some(StipulationType::FinalCprOfHomeEquityPrepaymentCurve),
            b"ISSUE" => Some(StipulationType::YearOfIssue),
            b"LOTVAR" => Some(StipulationType::LotVariance),
            b"MAT" => Some(StipulationType::MaturityYear),
            b"MHP" => Some(StipulationType::OfManufacturedHousingPrepaymentCurve),
            b"MPR" => Some(StipulationType::MonthlyPrepaymentRate),
            b"PIECES" => Some(StipulationType::NumberOfPieces),
            b"PMAX" => Some(StipulationType::PoolsMaximum),
            b"PPC" => Some(StipulationType::OfProspectusPrepaymentCurve),
            b"PPL" => Some(StipulationType::PoolsPerLot),
            b"PPM" => Some(StipulationType::PoolsPerMillion),
            b"PPT" => Some(StipulationType::PoolsPerTrade),
            b"PROD" => Some(StipulationType::ProductionYear),
            b"PSA" => Some(StipulationType::OfBmaPrepaymentCurve),
            b"SMM" => Some(StipulationType::SingleMonthlyMortality),
            b"TRDVAR" => Some(StipulationType::TradeVariance),
            b"WAC" => Some(StipulationType::WeightedAverageCoupon),
            b"WAL" => Some(StipulationType::WeightedAverageLife),
            b"WALA" => Some(StipulationType::WeightedAverageLoanAge),
            b"WAM" => Some(StipulationType::WeightedAverageMaturity),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            StipulationType::AbsolutePrepaymentSpeed => v.extend(b"ABS"),
            StipulationType::ConstantPrepaymentPenalty => v.extend(b"CPP"),
            StipulationType::ConstantPrepaymentRate => v.extend(b"CPR"),
            StipulationType::ConstantPrepaymentYield => v.extend(b"CPY"),
            StipulationType::Geographics => v.extend(b"GEOG"),
            StipulationType::FinalCprOfHomeEquityPrepaymentCurve => v.extend(b"HEP"),
            StipulationType::YearOfIssue => v.extend(b"ISSUE"),
            StipulationType::LotVariance => v.extend(b"LOTVAR"),
            StipulationType::MaturityYear => v.extend(b"MAT"),
            StipulationType::OfManufacturedHousingPrepaymentCurve => v.extend(b"MHP"),
            StipulationType::MonthlyPrepaymentRate => v.extend(b"MPR"),
            StipulationType::NumberOfPieces => v.extend(b"PIECES"),
            StipulationType::PoolsMaximum => v.extend(b"PMAX"),
            StipulationType::OfProspectusPrepaymentCurve => v.extend(b"PPC"),
            StipulationType::PoolsPerLot => v.extend(b"PPL"),
            StipulationType::PoolsPerMillion => v.extend(b"PPM"),
            StipulationType::PoolsPerTrade => v.extend(b"PPT"),
            StipulationType::ProductionYear => v.extend(b"PROD"),
            StipulationType::OfBmaPrepaymentCurve => v.extend(b"PSA"),
            StipulationType::SingleMonthlyMortality => v.extend(b"SMM"),
            StipulationType::TradeVariance => v.extend(b"TRDVAR"),
            StipulationType::WeightedAverageCoupon => v.extend(b"WAC"),
            StipulationType::WeightedAverageLife => v.extend(b"WAL"),
            StipulationType::WeightedAverageLoanAge => v.extend(b"WALA"),
            StipulationType::WeightedAverageMaturity => v.extend(b"WAM"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    AtTheClose = b'7' as isize,
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
            b"7" => Some(TimeInForce::AtTheClose),
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
    PreClose = b'5' as isize,
    RequestRejected = b'6' as isize,
    PreOpen = b'4' as isize,
    Closed = b'3' as isize,
    Open = b'2' as isize,
    Halted = b'1' as isize,
    Unknown = b'0' as isize,
}

impl FIXValue for TradSesStatus {
    fn from_bytes(bytes: &[u8]) -> Option<TradSesStatus> {
        match bytes {
            b"5" => Some(TradSesStatus::PreClose),
            b"6" => Some(TradSesStatus::RequestRejected),
            b"4" => Some(TradSesStatus::PreOpen),
            b"3" => Some(TradSesStatus::Closed),
            b"2" => Some(TradSesStatus::Open),
            b"1" => Some(TradSesStatus::Halted),
            b"0" => Some(TradSesStatus::Unknown),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradSesStatusRejReason {
    UnknownOrInvalidTradingsessionid = b'1' as isize,
}

impl FIXValue for TradSesStatusRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<TradSesStatusRejReason> {
        match bytes {
            b"1" => Some(TradSesStatusRejReason::UnknownOrInvalidTradingsessionid),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeCondition {
    NextDayTrade = b'J' as isize,
    Opened = b'K' as isize,
    Seller = b'L' as isize,
    AveragePriceTrade = b'B' as isize,
    Sold = b'M' as isize,
    Rule155Trade = b'H' as isize,
    StoppedStock = b'N' as isize,
    ImbalanceMoreBuyers = b'P' as isize,
    ImbalanceMoreSellers = b'Q' as isize,
    OpeningPrice = b'R' as isize,
    SoldLast = b'I' as isize,
    Cash = b'A' as isize,
    CashTrade = b'C' as isize,
    Opening = b'E' as isize,
    IntradayTradeDetail = b'F' as isize,
    Rule127Trade = b'G' as isize,
    NextDay = b'D' as isize,
}

impl FIXValue for TradeCondition {
    fn from_bytes(bytes: &[u8]) -> Option<TradeCondition> {
        match bytes {
            b"J" => Some(TradeCondition::NextDayTrade),
            b"K" => Some(TradeCondition::Opened),
            b"L" => Some(TradeCondition::Seller),
            b"B" => Some(TradeCondition::AveragePriceTrade),
            b"M" => Some(TradeCondition::Sold),
            b"H" => Some(TradeCondition::Rule155Trade),
            b"N" => Some(TradeCondition::StoppedStock),
            b"P" => Some(TradeCondition::ImbalanceMoreBuyers),
            b"Q" => Some(TradeCondition::ImbalanceMoreSellers),
            b"R" => Some(TradeCondition::OpeningPrice),
            b"I" => Some(TradeCondition::SoldLast),
            b"A" => Some(TradeCondition::Cash),
            b"C" => Some(TradeCondition::CashTrade),
            b"E" => Some(TradeCondition::Opening),
            b"F" => Some(TradeCondition::IntradayTradeDetail),
            b"G" => Some(TradeCondition::Rule127Trade),
            b"D" => Some(TradeCondition::NextDay),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeReportTransType {
    New = b'N' as isize,
    Replace = b'R' as isize,
    Cancel = b'C' as isize,
}

impl FIXValue for TradeReportTransType {
    fn from_bytes(bytes: &[u8]) -> Option<TradeReportTransType> {
        match bytes {
            b"N" => Some(TradeReportTransType::New),
            b"R" => Some(TradeReportTransType::Replace),
            b"C" => Some(TradeReportTransType::Cancel),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YieldType {
    AfterTaxYield,
    AnnualYieldTheAnnualInterestOrDividendIncomeAnInvestmentEarnsExpressedAsAPercentageOfTheInvestmentsTotalValue,
    YieldAtIssue,
    YieldToAverageLifeTheYieldAssumingThatAllSinks,
    YieldToAverageMaturityTheYieldAchievedBySubstitutingABondsAverageMaturityForTheIssuesFinalMaturityDate,
    BookYieldTheYieldOfASecurityCalculatedByUsingItsBookValueInsteadOfTheCurrentMarketPriceThisTermIsTypicallyUsedInTheUsDomesticMarket,
    YieldToNextCallTheYieldOfABondToTheNextPossibleCallDate,
    YieldChangeSinceCloseTheChangeInTheYieldSinceThePreviousDaysClosingYield,
    ClosingYieldTheYieldOfABondBasedOnTheClosingPrice,
    CompoundYieldTheYieldOfCertainJapaneseBondsBasedOnItsPriceCertainJapaneseBondsHaveIrregularFirstOrLastCouponsAndTheYieldIsCalculatedCompoundForTheseIrregularPeriods,
    CurrentYieldAnnualInterestOnABondDividedByTheMarketValueTheActualIncomeRateOfReturnAsOpposedToTheCouponRateExpressedAsAPercentage,
    GovernmentEquivalentYieldAskYieldBasedOnSemiAnnualCouponsCompoundingInAllPeriodsAndActualActualCalendar,
    TrueGrossYieldYieldCalculatedUsingThePriceIncludingAccruedInterestWhereCouponDatesAreMovedFromHolidaysAndWeekendsToTheNextTradingDay,
    YieldWithInflationAssumptionBasedOnPriceTheReturnAnInvestorWouldRequireOnANormalBondThatWouldMakeTheRealReturnEqualToThatOfTheInflationIndexedBondAssumingAConstantInflationRate,
    InverseFloaterBondYieldInverseFloaterSemiAnnualBondEquivalentRate,
    MostRecentClosingYieldTheLastAvailableYieldStoredInHistoryComputedUsingPrice,
    ClosingYieldMostRecentMonthTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentMonthsEnd,
    ClosingYieldMostRecentQuarterTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentQuartersEnd,
    ClosingYieldMostRecentYearTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentYearsEnd,
    YieldToLongestAverageLifeTheYieldAssumingOnlyMandatorySinksAreTakenThisResultsInALowerPaydownOfDebtTheYieldIsThenCalculatedToTheFinalPaymentDate,
    YieldToLongestAverage,
    MarkToMarketYieldAnAdjustmentInTheValuationOfASecuritiesPortfolioToReflectTheCurrentMarketValuesOfTheRespectiveSecuritiesInThePortfolio,
    YieldToMaturityTheYieldOfABondToItsMaturityDate,
    YieldToNextRefund,
    OpenAverageYieldTheAverageYieldOfTheRespectiveSecuritiesInThePortfolio,
    PreviousCloseYieldTheYieldOfABondBasedOnTheClosingPrice1DayAgo,
    ProceedsYieldTheCdEquivalentYieldWhenTheRemainingTimeToMaturityIsLessThanTwoYears,
    YieldToNextPutTheYieldToTheDateAtWhichTheBondHolderCanNextPutTheBondToTheIssuer,
    SemiAnnualYieldTheYieldOfABondWhoseCouponPaymentsAreReinvestedSemiAnnually,
    YieldToShortestAverageLifeSameAsAvglifeAbove,
    YieldToShortestAverage,
    SimpleYieldTheYieldOfABondAssumingNoReinvestmentOfCouponPayments,
    TaxEquivalentYieldTheAfterTaxYieldGrossedUpByTheMaximumFederalTaxRateOf396ForComparisonToTaxableYields,
    YieldToTenderDateTheYieldOnAMunicipalBondToItsMandatoryTenderDate,
    TrueYieldTheYieldCalculatedWithCouponDatesMovedFromAWeekendOrHolidayToTheNextValidSettlementDate,
    YieldValueOf132TheAmountThatTheYieldWillChangeForA132NdChangeInPrice,
    YieldToWorstConventionTheLowestYieldToAllPossibleRedemptionDateScenarios,
}

impl FIXValue for YieldType {
    fn from_bytes(bytes: &[u8]) -> Option<YieldType> {
        match bytes {
            b"AFTERTAX" => Some(YieldType::AfterTaxYield),
            b"ANNUAL" => Some(YieldType::AnnualYieldTheAnnualInterestOrDividendIncomeAnInvestmentEarnsExpressedAsAPercentageOfTheInvestmentsTotalValue),
            b"ATISSUE" => Some(YieldType::YieldAtIssue),
            b"AVGLIFE" => Some(YieldType::YieldToAverageLifeTheYieldAssumingThatAllSinks),
            b"AVGMATURITY" => Some(YieldType::YieldToAverageMaturityTheYieldAchievedBySubstitutingABondsAverageMaturityForTheIssuesFinalMaturityDate),
            b"BOOK" => Some(YieldType::BookYieldTheYieldOfASecurityCalculatedByUsingItsBookValueInsteadOfTheCurrentMarketPriceThisTermIsTypicallyUsedInTheUsDomesticMarket),
            b"CALL" => Some(YieldType::YieldToNextCallTheYieldOfABondToTheNextPossibleCallDate),
            b"CHANGE" => Some(YieldType::YieldChangeSinceCloseTheChangeInTheYieldSinceThePreviousDaysClosingYield),
            b"CLOSE" => Some(YieldType::ClosingYieldTheYieldOfABondBasedOnTheClosingPrice),
            b"COMPOUND" => Some(YieldType::CompoundYieldTheYieldOfCertainJapaneseBondsBasedOnItsPriceCertainJapaneseBondsHaveIrregularFirstOrLastCouponsAndTheYieldIsCalculatedCompoundForTheseIrregularPeriods),
            b"CURRENT" => Some(YieldType::CurrentYieldAnnualInterestOnABondDividedByTheMarketValueTheActualIncomeRateOfReturnAsOpposedToTheCouponRateExpressedAsAPercentage),
            b"GOVTEQUIV" => Some(YieldType::GovernmentEquivalentYieldAskYieldBasedOnSemiAnnualCouponsCompoundingInAllPeriodsAndActualActualCalendar),
            b"GROSS" => Some(YieldType::TrueGrossYieldYieldCalculatedUsingThePriceIncludingAccruedInterestWhereCouponDatesAreMovedFromHolidaysAndWeekendsToTheNextTradingDay),
            b"INFLATION" => Some(YieldType::YieldWithInflationAssumptionBasedOnPriceTheReturnAnInvestorWouldRequireOnANormalBondThatWouldMakeTheRealReturnEqualToThatOfTheInflationIndexedBondAssumingAConstantInflationRate),
            b"INVERSEFLOATER" => Some(YieldType::InverseFloaterBondYieldInverseFloaterSemiAnnualBondEquivalentRate),
            b"LASTCLOSE" => Some(YieldType::MostRecentClosingYieldTheLastAvailableYieldStoredInHistoryComputedUsingPrice),
            b"LASTMONTH" => Some(YieldType::ClosingYieldMostRecentMonthTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentMonthsEnd),
            b"LASTQUARTER" => Some(YieldType::ClosingYieldMostRecentQuarterTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentQuartersEnd),
            b"LASTYEAR" => Some(YieldType::ClosingYieldMostRecentYearTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentYearsEnd),
            b"LONGAVGLIFE" => Some(YieldType::YieldToLongestAverageLifeTheYieldAssumingOnlyMandatorySinksAreTakenThisResultsInALowerPaydownOfDebtTheYieldIsThenCalculatedToTheFinalPaymentDate),
            b"LONGEST" => Some(YieldType::YieldToLongestAverage),
            b"MARK" => Some(YieldType::MarkToMarketYieldAnAdjustmentInTheValuationOfASecuritiesPortfolioToReflectTheCurrentMarketValuesOfTheRespectiveSecuritiesInThePortfolio),
            b"MATURITY" => Some(YieldType::YieldToMaturityTheYieldOfABondToItsMaturityDate),
            b"NEXTREFUND" => Some(YieldType::YieldToNextRefund),
            b"OPENAVG" => Some(YieldType::OpenAverageYieldTheAverageYieldOfTheRespectiveSecuritiesInThePortfolio),
            b"PREVCLOSE" => Some(YieldType::PreviousCloseYieldTheYieldOfABondBasedOnTheClosingPrice1DayAgo),
            b"PROCEEDS" => Some(YieldType::ProceedsYieldTheCdEquivalentYieldWhenTheRemainingTimeToMaturityIsLessThanTwoYears),
            b"PUT" => Some(YieldType::YieldToNextPutTheYieldToTheDateAtWhichTheBondHolderCanNextPutTheBondToTheIssuer),
            b"SEMIANNUAL" => Some(YieldType::SemiAnnualYieldTheYieldOfABondWhoseCouponPaymentsAreReinvestedSemiAnnually),
            b"SHORTAVGLIFE" => Some(YieldType::YieldToShortestAverageLifeSameAsAvglifeAbove),
            b"SHORTEST" => Some(YieldType::YieldToShortestAverage),
            b"SIMPLE" => Some(YieldType::SimpleYieldTheYieldOfABondAssumingNoReinvestmentOfCouponPayments),
            b"TAXEQUIV" => Some(YieldType::TaxEquivalentYieldTheAfterTaxYieldGrossedUpByTheMaximumFederalTaxRateOf396ForComparisonToTaxableYields),
            b"TENDER" => Some(YieldType::YieldToTenderDateTheYieldOnAMunicipalBondToItsMandatoryTenderDate),
            b"TRUE" => Some(YieldType::TrueYieldTheYieldCalculatedWithCouponDatesMovedFromAWeekendOrHolidayToTheNextValidSettlementDate),
            b"VALUE1/32" => Some(YieldType::YieldValueOf132TheAmountThatTheYieldWillChangeForA132NdChangeInPrice),
            b"WORST" => Some(YieldType::YieldToWorstConventionTheLowestYieldToAllPossibleRedemptionDateScenarios),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            YieldType::AfterTaxYield => v.extend(b"AFTERTAX"),
            YieldType::AnnualYieldTheAnnualInterestOrDividendIncomeAnInvestmentEarnsExpressedAsAPercentageOfTheInvestmentsTotalValue => v.extend(b"ANNUAL"),
            YieldType::YieldAtIssue => v.extend(b"ATISSUE"),
            YieldType::YieldToAverageLifeTheYieldAssumingThatAllSinks => v.extend(b"AVGLIFE"),
            YieldType::YieldToAverageMaturityTheYieldAchievedBySubstitutingABondsAverageMaturityForTheIssuesFinalMaturityDate => v.extend(b"AVGMATURITY"),
            YieldType::BookYieldTheYieldOfASecurityCalculatedByUsingItsBookValueInsteadOfTheCurrentMarketPriceThisTermIsTypicallyUsedInTheUsDomesticMarket => v.extend(b"BOOK"),
            YieldType::YieldToNextCallTheYieldOfABondToTheNextPossibleCallDate => v.extend(b"CALL"),
            YieldType::YieldChangeSinceCloseTheChangeInTheYieldSinceThePreviousDaysClosingYield => v.extend(b"CHANGE"),
            YieldType::ClosingYieldTheYieldOfABondBasedOnTheClosingPrice => v.extend(b"CLOSE"),
            YieldType::CompoundYieldTheYieldOfCertainJapaneseBondsBasedOnItsPriceCertainJapaneseBondsHaveIrregularFirstOrLastCouponsAndTheYieldIsCalculatedCompoundForTheseIrregularPeriods => v.extend(b"COMPOUND"),
            YieldType::CurrentYieldAnnualInterestOnABondDividedByTheMarketValueTheActualIncomeRateOfReturnAsOpposedToTheCouponRateExpressedAsAPercentage => v.extend(b"CURRENT"),
            YieldType::GovernmentEquivalentYieldAskYieldBasedOnSemiAnnualCouponsCompoundingInAllPeriodsAndActualActualCalendar => v.extend(b"GOVTEQUIV"),
            YieldType::TrueGrossYieldYieldCalculatedUsingThePriceIncludingAccruedInterestWhereCouponDatesAreMovedFromHolidaysAndWeekendsToTheNextTradingDay => v.extend(b"GROSS"),
            YieldType::YieldWithInflationAssumptionBasedOnPriceTheReturnAnInvestorWouldRequireOnANormalBondThatWouldMakeTheRealReturnEqualToThatOfTheInflationIndexedBondAssumingAConstantInflationRate => v.extend(b"INFLATION"),
            YieldType::InverseFloaterBondYieldInverseFloaterSemiAnnualBondEquivalentRate => v.extend(b"INVERSEFLOATER"),
            YieldType::MostRecentClosingYieldTheLastAvailableYieldStoredInHistoryComputedUsingPrice => v.extend(b"LASTCLOSE"),
            YieldType::ClosingYieldMostRecentMonthTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentMonthsEnd => v.extend(b"LASTMONTH"),
            YieldType::ClosingYieldMostRecentQuarterTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentQuartersEnd => v.extend(b"LASTQUARTER"),
            YieldType::ClosingYieldMostRecentYearTheYieldOfABondBasedOnTheClosingPriceAsOfTheMostRecentYearsEnd => v.extend(b"LASTYEAR"),
            YieldType::YieldToLongestAverageLifeTheYieldAssumingOnlyMandatorySinksAreTakenThisResultsInALowerPaydownOfDebtTheYieldIsThenCalculatedToTheFinalPaymentDate => v.extend(b"LONGAVGLIFE"),
            YieldType::YieldToLongestAverage => v.extend(b"LONGEST"),
            YieldType::MarkToMarketYieldAnAdjustmentInTheValuationOfASecuritiesPortfolioToReflectTheCurrentMarketValuesOfTheRespectiveSecuritiesInThePortfolio => v.extend(b"MARK"),
            YieldType::YieldToMaturityTheYieldOfABondToItsMaturityDate => v.extend(b"MATURITY"),
            YieldType::YieldToNextRefund => v.extend(b"NEXTREFUND"),
            YieldType::OpenAverageYieldTheAverageYieldOfTheRespectiveSecuritiesInThePortfolio => v.extend(b"OPENAVG"),
            YieldType::PreviousCloseYieldTheYieldOfABondBasedOnTheClosingPrice1DayAgo => v.extend(b"PREVCLOSE"),
            YieldType::ProceedsYieldTheCdEquivalentYieldWhenTheRemainingTimeToMaturityIsLessThanTwoYears => v.extend(b"PROCEEDS"),
            YieldType::YieldToNextPutTheYieldToTheDateAtWhichTheBondHolderCanNextPutTheBondToTheIssuer => v.extend(b"PUT"),
            YieldType::SemiAnnualYieldTheYieldOfABondWhoseCouponPaymentsAreReinvestedSemiAnnually => v.extend(b"SEMIANNUAL"),
            YieldType::YieldToShortestAverageLifeSameAsAvglifeAbove => v.extend(b"SHORTAVGLIFE"),
            YieldType::YieldToShortestAverage => v.extend(b"SHORTEST"),
            YieldType::SimpleYieldTheYieldOfABondAssumingNoReinvestmentOfCouponPayments => v.extend(b"SIMPLE"),
            YieldType::TaxEquivalentYieldTheAfterTaxYieldGrossedUpByTheMaximumFederalTaxRateOf396ForComparisonToTaxableYields => v.extend(b"TAXEQUIV"),
            YieldType::YieldToTenderDateTheYieldOnAMunicipalBondToItsMandatoryTenderDate => v.extend(b"TENDER"),
            YieldType::TrueYieldTheYieldCalculatedWithCouponDatesMovedFromAWeekendOrHolidayToTheNextValidSettlementDate => v.extend(b"TRUE"),
            YieldType::YieldValueOf132TheAmountThatTheYieldWillChangeForA132NdChangeInPrice => v.extend(b"VALUE1/32"),
            YieldType::YieldToWorstConventionTheLowestYieldToAllPossibleRedemptionDateScenarios => v.extend(b"WORST"),
        }
    }
}

pub struct CommissionData {
    pub commission: Option<Amt>,
    pub comm_type: Option<CommType>,
    pub comm_currency: Option<Currency>,
    pub fund_renew_waiv: Option<FundRenewWaiv>
}

pub struct Instrument {
    pub symbol: Option<FIXString>,
    pub symbol_sfx: Option<FIXString>,
    pub security_id: Option<FIXString>,
    pub security_id_source: Option<SecurityIDSource>,
    pub security_alt_id: Option<FIXString>,
    pub security_alt_id_source: Option<FIXString>,
    pub product: Option<Product>,
    pub cfi_code: Option<FIXString>,
    pub security_type: Option<SecurityType>,
    pub maturity_month_year: Option<MonthYear>,
    pub maturity_date: Option<LocalMktDate>,
    pub coupon_payment_date: Option<UTCDateOnly>,
    pub issue_date: Option<UTCDateOnly>,
    pub repo_collateral_security_type: Option<FIXString>,
    pub repurchase_term: Option<FIXInt>,
    pub repurchase_rate: Option<Percentage>,
    pub factor: Option<FIXFloat>,
    pub credit_rating: Option<FIXString>,
    pub instr_registry: Option<FIXString>,
    pub country_of_issue: Option<Country>,
    pub state_or_province_of_issue: Option<FIXString>,
    pub locale_of_issue: Option<FIXString>,
    pub redemption_date: Option<UTCDateOnly>,
    pub strike_price: Option<Price>,
    pub opt_attribute: Option<FIXChar>,
    pub contract_multiplier: Option<FIXFloat>,
    pub coupon_rate: Option<Percentage>,
    pub security_exchange: Option<MICExchange>,
    pub issuer: Option<FIXString>,
    pub encoded_issuer: Option<Data>,
    pub security_desc: Option<FIXString>,
    pub encoded_security_desc: Option<Data>
}

pub struct InstrumentLeg {
    pub leg_symbol: Option<FIXString>,
    pub leg_symbol_sfx: Option<FIXString>,
    pub leg_security_id: Option<FIXString>,
    pub leg_security_id_source: Option<FIXString>,
    pub leg_security_alt_id: Option<FIXString>,
    pub leg_security_alt_id_source: Option<FIXString>,
    pub leg_product: Option<FIXInt>,
    pub leg_cfi_code: Option<FIXString>,
    pub leg_security_type: Option<FIXString>,
    pub leg_maturity_month_year: Option<MonthYear>,
    pub leg_maturity_date: Option<LocalMktDate>,
    pub leg_coupon_payment_date: Option<UTCDateOnly>,
    pub leg_issue_date: Option<UTCDateOnly>,
    pub leg_repo_collateral_security_type: Option<FIXString>,
    pub leg_repurchase_term: Option<FIXInt>,
    pub leg_repurchase_rate: Option<Percentage>,
    pub leg_factor: Option<FIXFloat>,
    pub leg_credit_rating: Option<FIXString>,
    pub leg_instr_registry: Option<FIXString>,
    pub leg_country_of_issue: Option<Country>,
    pub leg_state_or_province_of_issue: Option<FIXString>,
    pub leg_locale_of_issue: Option<FIXString>,
    pub leg_redemption_date: Option<UTCDateOnly>,
    pub leg_strike_price: Option<Price>,
    pub leg_opt_attribute: Option<FIXChar>,
    pub leg_contract_multiplier: Option<FIXFloat>,
    pub leg_coupon_rate: Option<Percentage>,
    pub leg_security_exchange: Option<MICExchange>,
    pub leg_issuer: Option<FIXString>,
    pub encoded_leg_issuer: Option<Data>,
    pub leg_security_desc: Option<FIXString>,
    pub encoded_leg_security_desc: Option<Data>,
    pub leg_ratio_qty: Option<FIXFloat>,
    pub leg_side: Option<FIXChar>
}

pub struct NestedParties {
    pub nested_party_id: Option<FIXString>,
    pub nested_party_id_source: Option<FIXChar>,
    pub nested_party_role: Option<FIXInt>,
    pub nested_party_sub_id: Option<FIXString>
}

pub struct OrderQtyData {
    pub order_qty: Option<Qty>,
    pub cash_order_qty: Option<Qty>,
    pub order_percent: Option<Percentage>,
    pub rounding_direction: Option<RoundingDirection>,
    pub rounding_modulus: Option<FIXFloat>
}

pub struct Parties {
    pub party_id: Option<FIXString>,
    pub party_id_source: Option<PartyIDSource>,
    pub party_role: Option<PartyRole>,
    pub party_sub_id: Option<FIXString>
}

pub struct SpreadOrBenchmarkCurveData {
    pub spread: Option<PriceOffset>,
    pub benchmark_curve_currency: Option<Currency>,
    pub benchmark_curve_name: Option<BenchmarkCurveName>,
    pub benchmark_curve_point: Option<FIXString>
}

pub struct Stipulations {
    pub stipulation_type: Option<StipulationType>,
    pub stipulation_value: Option<FIXString>
}

pub struct UnderlyingInstrument {
    pub underlying_symbol: Option<FIXString>,
    pub underlying_symbol_sfx: Option<FIXString>,
    pub underlying_security_id: Option<FIXString>,
    pub underlying_security_id_source: Option<FIXString>,
    pub underlying_security_alt_id: Option<FIXString>,
    pub underlying_security_alt_id_source: Option<FIXString>,
    pub underlying_product: Option<FIXInt>,
    pub underlying_cfi_code: Option<FIXString>,
    pub underlying_security_type: Option<FIXString>,
    pub underlying_maturity_month_year: Option<MonthYear>,
    pub underlying_maturity_date: Option<LocalMktDate>,
    pub underlying_put_or_call: Option<FIXInt>,
    pub underlying_coupon_payment_date: Option<UTCDateOnly>,
    pub underlying_issue_date: Option<UTCDateOnly>,
    pub underlying_repo_collateral_security_type: Option<FIXString>,
    pub underlying_repurchase_term: Option<FIXInt>,
    pub underlying_repurchase_rate: Option<Percentage>,
    pub underlying_factor: Option<FIXFloat>,
    pub underlying_credit_rating: Option<FIXString>,
    pub underlying_instr_registry: Option<FIXString>,
    pub underlying_country_of_issue: Option<Country>,
    pub underlying_state_or_province_of_issue: Option<FIXString>,
    pub underlying_locale_of_issue: Option<FIXString>,
    pub underlying_redemption_date: Option<UTCDateOnly>,
    pub underlying_strike_price: Option<Price>,
    pub underlying_opt_attribute: Option<FIXChar>,
    pub underlying_contract_multiplier: Option<FIXFloat>,
    pub underlying_coupon_rate: Option<Percentage>,
    pub underlying_security_exchange: Option<MICExchange>,
    pub underlying_issuer: Option<FIXString>,
    pub encoded_underlying_issuer: Option<Data>,
    pub underlying_security_desc: Option<FIXString>,
    pub encoded_underlying_security_desc: Option<Data>
}

pub struct YieldData {
    pub yield_type: Option<YieldType>,
    pub yield_: Option<Percentage>
}
