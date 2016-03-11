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
pub use common::PutOrCall as PutOrCall;
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
pub use common::TradedFlatSwitch as TradedFlatSwitch;
pub use common::UnsolicitedIndicator as UnsolicitedIndicator;
pub use common::Urgency as Urgency;
pub use common::WorkingIndicator as WorkingIndicator;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcctIDSource {
    Bic = b'1' as isize,
    SidCode = b'2' as isize,
    Tfm = b'3' as isize,
    Omgeo = b'4' as isize,
    DtccCode = b'5' as isize,
    Other,
}

impl FIXValue for AcctIDSource {
    fn from_bytes(bytes: &[u8]) -> Option<AcctIDSource> {
        match bytes {
            b"1" => Some(AcctIDSource::Bic),
            b"2" => Some(AcctIDSource::SidCode),
            b"3" => Some(AcctIDSource::Tfm),
            b"4" => Some(AcctIDSource::Omgeo),
            b"5" => Some(AcctIDSource::DtccCode),
            b"99" => Some(AcctIDSource::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            AcctIDSource::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjustmentType {
    ProcessRequestAsMarginDisposition = b'0' as isize,
    DeltaPlus = b'1' as isize,
    DeltaMinus = b'2' as isize,
    Final = b'3' as isize,
}

impl FIXValue for AdjustmentType {
    fn from_bytes(bytes: &[u8]) -> Option<AdjustmentType> {
        match bytes {
            b"0" => Some(AdjustmentType::ProcessRequestAsMarginDisposition),
            b"1" => Some(AdjustmentType::DeltaPlus),
            b"2" => Some(AdjustmentType::DeltaMinus),
            b"3" => Some(AdjustmentType::Final),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AffirmStatus {
    Received = b'1' as isize,
    ConfirmRejectedIeNotAffirmed = b'2' as isize,
    Affirmed = b'3' as isize,
}

impl FIXValue for AffirmStatus {
    fn from_bytes(bytes: &[u8]) -> Option<AffirmStatus> {
        match bytes {
            b"1" => Some(AffirmStatus::Received),
            b"2" => Some(AffirmStatus::ConfirmRejectedIeNotAffirmed),
            b"3" => Some(AffirmStatus::Affirmed),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocAccountType {
    AccountIsCarriedOnCustomerSideOfBooks = b'1' as isize,
    AccountIsCarriedOnNonCustomerSideOfBooks = b'2' as isize,
    HouseTrader = b'3' as isize,
    FloorTrader = b'4' as isize,
    AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined = b'6' as isize,
    AccountIsHouseTraderAndIsCrossMargined = b'7' as isize,
    JointBackofficeAccount = b'8' as isize,
}

impl FIXValue for AllocAccountType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocAccountType> {
        match bytes {
            b"1" => Some(AllocAccountType::AccountIsCarriedOnCustomerSideOfBooks),
            b"2" => Some(AllocAccountType::AccountIsCarriedOnNonCustomerSideOfBooks),
            b"3" => Some(AllocAccountType::HouseTrader),
            b"4" => Some(AllocAccountType::FloorTrader),
            b"6" => Some(AllocAccountType::AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined),
            b"7" => Some(AllocAccountType::AccountIsHouseTraderAndIsCrossMargined),
            b"8" => Some(AllocAccountType::JointBackofficeAccount),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocCancReplaceReason {
    OriginalDetailsIncompleteIncorrect = b'1' as isize,
    ChangeInUnderlyingOrderDetails = b'2' as isize,
    Other,
}

impl FIXValue for AllocCancReplaceReason {
    fn from_bytes(bytes: &[u8]) -> Option<AllocCancReplaceReason> {
        match bytes {
            b"1" => Some(AllocCancReplaceReason::OriginalDetailsIncompleteIncorrect),
            b"2" => Some(AllocCancReplaceReason::ChangeInUnderlyingOrderDetails),
            b"99" => Some(AllocCancReplaceReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            AllocCancReplaceReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocIntermedReqType {
    PendingAccept = b'1' as isize,
    PendingRelease = b'2' as isize,
    PendingReversal = b'3' as isize,
    Accept = b'4' as isize,
    BlockLevelReject = b'5' as isize,
    AccountLevelReject = b'6' as isize,
}

impl FIXValue for AllocIntermedReqType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocIntermedReqType> {
        match bytes {
            b"1" => Some(AllocIntermedReqType::PendingAccept),
            b"2" => Some(AllocIntermedReqType::PendingRelease),
            b"3" => Some(AllocIntermedReqType::PendingReversal),
            b"4" => Some(AllocIntermedReqType::Accept),
            b"5" => Some(AllocIntermedReqType::BlockLevelReject),
            b"6" => Some(AllocIntermedReqType::AccountLevelReject),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocNoOrdersType {
    NotSpecified = b'0' as isize,
    ExplicitListProvided = b'1' as isize,
}

impl FIXValue for AllocNoOrdersType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocNoOrdersType> {
        match bytes {
            b"0" => Some(AllocNoOrdersType::NotSpecified),
            b"1" => Some(AllocNoOrdersType::ExplicitListProvided),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

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
    IncorrectAllocatedQuantity = b'8' as isize,
    CalculationDifference = b'9' as isize,
    UnknownOrStaleExecid,
    MismatchedDataValue,
    UnknownClordid,
    WarehouseRequestRejected,
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
            b"8" => Some(AllocRejCode::IncorrectAllocatedQuantity),
            b"9" => Some(AllocRejCode::CalculationDifference),
            b"10" => Some(AllocRejCode::UnknownOrStaleExecid),
            b"11" => Some(AllocRejCode::MismatchedDataValue),
            b"12" => Some(AllocRejCode::UnknownClordid),
            b"13" => Some(AllocRejCode::WarehouseRequestRejected),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            AllocRejCode::UnknownOrStaleExecid => v.extend(b"10"),
            AllocRejCode::MismatchedDataValue => v.extend(b"11"),
            AllocRejCode::UnknownClordid => v.extend(b"12"),
            AllocRejCode::WarehouseRequestRejected => v.extend(b"13"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocReportType {
    SellsideCalculatedUsingPreliminary = b'3' as isize,
    SellsideCalculatedWithoutPreliminary = b'4' as isize,
    WarehouseRecap = b'5' as isize,
    RequestToIntermediary = b'8' as isize,
}

impl FIXValue for AllocReportType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocReportType> {
        match bytes {
            b"3" => Some(AllocReportType::SellsideCalculatedUsingPreliminary),
            b"4" => Some(AllocReportType::SellsideCalculatedWithoutPreliminary),
            b"5" => Some(AllocReportType::WarehouseRecap),
            b"8" => Some(AllocReportType::RequestToIntermediary),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocSettlInstType {
    UseDefaultInstructions = b'0' as isize,
    DeriveFromParametersProvided = b'1' as isize,
    FullDetailsProvided = b'2' as isize,
    SsiDbIdsProvided = b'3' as isize,
    PhoneForInstructions = b'4' as isize,
}

impl FIXValue for AllocSettlInstType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocSettlInstType> {
        match bytes {
            b"0" => Some(AllocSettlInstType::UseDefaultInstructions),
            b"1" => Some(AllocSettlInstType::DeriveFromParametersProvided),
            b"2" => Some(AllocSettlInstType::FullDetailsProvided),
            b"3" => Some(AllocSettlInstType::SsiDbIdsProvided),
            b"4" => Some(AllocSettlInstType::PhoneForInstructions),
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
    BlockLevelReject = b'1' as isize,
    AccountLevelReject = b'2' as isize,
    Received = b'3' as isize,
    Incomplete = b'4' as isize,
    RejectedByIntermediary = b'5' as isize,
}

impl FIXValue for AllocStatus {
    fn from_bytes(bytes: &[u8]) -> Option<AllocStatus> {
        match bytes {
            b"0" => Some(AllocStatus::Accepted),
            b"1" => Some(AllocStatus::BlockLevelReject),
            b"2" => Some(AllocStatus::AccountLevelReject),
            b"3" => Some(AllocStatus::Received),
            b"4" => Some(AllocStatus::Incomplete),
            b"5" => Some(AllocStatus::RejectedByIntermediary),
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
}

impl FIXValue for AllocTransType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocTransType> {
        match bytes {
            b"0" => Some(AllocTransType::New),
            b"1" => Some(AllocTransType::Replace),
            b"2" => Some(AllocTransType::Cancel),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocType {
    Calculated = b'1' as isize,
    Preliminary = b'2' as isize,
    ReadyToBook = b'5' as isize,
    WarehouseInstruction = b'7' as isize,
    RequestToIntermediary = b'8' as isize,
}

impl FIXValue for AllocType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocType> {
        match bytes {
            b"1" => Some(AllocType::Calculated),
            b"2" => Some(AllocType::Preliminary),
            b"5" => Some(AllocType::ReadyToBook),
            b"7" => Some(AllocType::WarehouseInstruction),
            b"8" => Some(AllocType::RequestToIntermediary),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplQueueAction {
    NoActionTaken = b'0' as isize,
    QueueFlushed = b'1' as isize,
    OverlayLast = b'2' as isize,
    EndSession = b'3' as isize,
}

impl FIXValue for ApplQueueAction {
    fn from_bytes(bytes: &[u8]) -> Option<ApplQueueAction> {
        match bytes {
            b"0" => Some(ApplQueueAction::NoActionTaken),
            b"1" => Some(ApplQueueAction::QueueFlushed),
            b"2" => Some(ApplQueueAction::OverlayLast),
            b"3" => Some(ApplQueueAction::EndSession),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplQueueResolution {
    NoActionTaken = b'0' as isize,
    QueueFlushed = b'1' as isize,
    OverlayLast = b'2' as isize,
    EndSession = b'3' as isize,
}

impl FIXValue for ApplQueueResolution {
    fn from_bytes(bytes: &[u8]) -> Option<ApplQueueResolution> {
        match bytes {
            b"0" => Some(ApplQueueResolution::NoActionTaken),
            b"1" => Some(ApplQueueResolution::QueueFlushed),
            b"2" => Some(ApplQueueResolution::OverlayLast),
            b"3" => Some(ApplQueueResolution::EndSession),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentMethod {
    Random = b'R' as isize,
    Prorata = b'P' as isize,
}

impl FIXValue for AssignmentMethod {
    fn from_bytes(bytes: &[u8]) -> Option<AssignmentMethod> {
        match bytes {
            b"R" => Some(AssignmentMethod::Random),
            b"P" => Some(AssignmentMethod::Prorata),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvgPxIndicator {
    NoAveragePricing = b'0' as isize,
    TradeIsPartOfAnAveragePriceGroupIdentifiedByTheTradelinkid = b'1' as isize,
    LastTradeInTheAveragePriceGroupIdentifiedByTheTradelinkid = b'2' as isize,
}

impl FIXValue for AvgPxIndicator {
    fn from_bytes(bytes: &[u8]) -> Option<AvgPxIndicator> {
        match bytes {
            b"0" => Some(AvgPxIndicator::NoAveragePricing),
            b"1" => Some(AvgPxIndicator::TradeIsPartOfAnAveragePriceGroupIdentifiedByTheTradelinkid),
            b"2" => Some(AvgPxIndicator::LastTradeInTheAveragePriceGroupIdentifiedByTheTradelinkid),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidDescriptorType {
    Sector = b'1' as isize,
    Country = b'2' as isize,
    Index = b'3' as isize,
}

impl FIXValue for BidDescriptorType {
    fn from_bytes(bytes: &[u8]) -> Option<BidDescriptorType> {
        match bytes {
            b"1" => Some(BidDescriptorType::Sector),
            b"2" => Some(BidDescriptorType::Country),
            b"3" => Some(BidDescriptorType::Index),
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
pub enum BidTradeType {
    RiskTrade = b'R' as isize,
    VwapGuarantee = b'G' as isize,
    Agency = b'A' as isize,
    GuaranteedClose = b'J' as isize,
}

impl FIXValue for BidTradeType {
    fn from_bytes(bytes: &[u8]) -> Option<BidTradeType> {
        match bytes {
            b"R" => Some(BidTradeType::RiskTrade),
            b"G" => Some(BidTradeType::VwapGuarantee),
            b"A" => Some(BidTradeType::Agency),
            b"J" => Some(BidTradeType::GuaranteedClose),
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
pub enum BookingType {
    RegularBooking = b'0' as isize,
    Cfd = b'1' as isize,
    TotalReturnSwap = b'2' as isize,
}

impl FIXValue for BookingType {
    fn from_bytes(bytes: &[u8]) -> Option<BookingType> {
        match bytes {
            b"0" => Some(BookingType::RegularBooking),
            b"1" => Some(BookingType::Cfd),
            b"2" => Some(BookingType::TotalReturnSwap),
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
    NotAuthorized = b'6' as isize,
    DelivertoFirmNotAvailableAtThisTime = b'7' as isize,
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
            b"6" => Some(BusinessRejectReason::NotAuthorized),
            b"7" => Some(BusinessRejectReason::DelivertoFirmNotAvailableAtThisTime),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CPProgram {
    Three = b'1' as isize,
    Four = b'2' as isize,
    Other,
}

impl FIXValue for CPProgram {
    fn from_bytes(bytes: &[u8]) -> Option<CPProgram> {
        match bytes {
            b"1" => Some(CPProgram::Three),
            b"2" => Some(CPProgram::Four),
            b"99" => Some(CPProgram::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            CPProgram::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearingFeeIndicator {
    CboeMember = b'B' as isize,
    NonMemberAndCustomer = b'C' as isize,
    EquityMemberAndClearingMember = b'E' as isize,
    FullAndAssociateMemberTradingForOwnAccountAndAsFloorBrokers = b'F' as isize,
    OnehundredandsixhAnd106JFirms = b'H' as isize,
    GimIdemAndComMembershipInterestHolders = b'I' as isize,
    LesseeAnd106FEmployees = b'L' as isize,
    AllOtherOwnershipTypes = b'M' as isize,
    OnestYearDelegateTradingForHisOwnAccount = b'1' as isize,
    TwondYearDelegateTradingForHisOwnAccount = b'2' as isize,
    ThreerdYearDelegateTradingForHisOwnAccount = b'3' as isize,
    FourthYearDelegateTradingForHisOwnAccount = b'4' as isize,
    FivethYearDelegateTradingForHisOwnAccount = b'5' as isize,
    SixthYearAndBeyondDelegateTradingForHisOwnAccount = b'9' as isize,
}

impl FIXValue for ClearingFeeIndicator {
    fn from_bytes(bytes: &[u8]) -> Option<ClearingFeeIndicator> {
        match bytes {
            b"B" => Some(ClearingFeeIndicator::CboeMember),
            b"C" => Some(ClearingFeeIndicator::NonMemberAndCustomer),
            b"E" => Some(ClearingFeeIndicator::EquityMemberAndClearingMember),
            b"F" => Some(ClearingFeeIndicator::FullAndAssociateMemberTradingForOwnAccountAndAsFloorBrokers),
            b"H" => Some(ClearingFeeIndicator::OnehundredandsixhAnd106JFirms),
            b"I" => Some(ClearingFeeIndicator::GimIdemAndComMembershipInterestHolders),
            b"L" => Some(ClearingFeeIndicator::LesseeAnd106FEmployees),
            b"M" => Some(ClearingFeeIndicator::AllOtherOwnershipTypes),
            b"1" => Some(ClearingFeeIndicator::OnestYearDelegateTradingForHisOwnAccount),
            b"2" => Some(ClearingFeeIndicator::TwondYearDelegateTradingForHisOwnAccount),
            b"3" => Some(ClearingFeeIndicator::ThreerdYearDelegateTradingForHisOwnAccount),
            b"4" => Some(ClearingFeeIndicator::FourthYearDelegateTradingForHisOwnAccount),
            b"5" => Some(ClearingFeeIndicator::FivethYearDelegateTradingForHisOwnAccount),
            b"9" => Some(ClearingFeeIndicator::SixthYearAndBeyondDelegateTradingForHisOwnAccount),
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
    QualifiedServiceRepresentative,
    CustomerTrade,
    SelfClearing,
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
            b"11" => Some(ClearingInstruction::QualifiedServiceRepresentative),
            b"12" => Some(ClearingInstruction::CustomerTrade),
            b"13" => Some(ClearingInstruction::SelfClearing),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            ClearingInstruction::AutomaticGiveUpMode => v.extend(b"10"),
            ClearingInstruction::QualifiedServiceRepresentative => v.extend(b"11"),
            ClearingInstruction::CustomerTrade => v.extend(b"12"),
            ClearingInstruction::SelfClearing => v.extend(b"13"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollAction {
    Retain = b'0' as isize,
    Add = b'1' as isize,
    Remove = b'2' as isize,
}

impl FIXValue for CollAction {
    fn from_bytes(bytes: &[u8]) -> Option<CollAction> {
        match bytes {
            b"0" => Some(CollAction::Retain),
            b"1" => Some(CollAction::Add),
            b"2" => Some(CollAction::Remove),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollAsgnReason {
    Initial = b'0' as isize,
    Scheduled = b'1' as isize,
    TimeWarning = b'2' as isize,
    MarginDeficiency = b'3' as isize,
    MarginExcess = b'4' as isize,
    ForwardCollateralDemand = b'5' as isize,
    EventOfDefault = b'6' as isize,
    AdverseTaxEvent = b'7' as isize,
}

impl FIXValue for CollAsgnReason {
    fn from_bytes(bytes: &[u8]) -> Option<CollAsgnReason> {
        match bytes {
            b"0" => Some(CollAsgnReason::Initial),
            b"1" => Some(CollAsgnReason::Scheduled),
            b"2" => Some(CollAsgnReason::TimeWarning),
            b"3" => Some(CollAsgnReason::MarginDeficiency),
            b"4" => Some(CollAsgnReason::MarginExcess),
            b"5" => Some(CollAsgnReason::ForwardCollateralDemand),
            b"6" => Some(CollAsgnReason::EventOfDefault),
            b"7" => Some(CollAsgnReason::AdverseTaxEvent),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollAsgnRejectReason {
    UnknownDeal = b'0' as isize,
    UnknownOrInvalidInstrument = b'1' as isize,
    UnauthorizedTransaction = b'2' as isize,
    InsufficientCollateral = b'3' as isize,
    InvalidTypeOfCollateral = b'4' as isize,
    ExcessiveSubstitution = b'5' as isize,
    Other,
}

impl FIXValue for CollAsgnRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<CollAsgnRejectReason> {
        match bytes {
            b"0" => Some(CollAsgnRejectReason::UnknownDeal),
            b"1" => Some(CollAsgnRejectReason::UnknownOrInvalidInstrument),
            b"2" => Some(CollAsgnRejectReason::UnauthorizedTransaction),
            b"3" => Some(CollAsgnRejectReason::InsufficientCollateral),
            b"4" => Some(CollAsgnRejectReason::InvalidTypeOfCollateral),
            b"5" => Some(CollAsgnRejectReason::ExcessiveSubstitution),
            b"99" => Some(CollAsgnRejectReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            CollAsgnRejectReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollAsgnRespType {
    Received = b'0' as isize,
    Accepted = b'1' as isize,
    Declined = b'2' as isize,
    Rejected = b'3' as isize,
}

impl FIXValue for CollAsgnRespType {
    fn from_bytes(bytes: &[u8]) -> Option<CollAsgnRespType> {
        match bytes {
            b"0" => Some(CollAsgnRespType::Received),
            b"1" => Some(CollAsgnRespType::Accepted),
            b"2" => Some(CollAsgnRespType::Declined),
            b"3" => Some(CollAsgnRespType::Rejected),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollAsgnTransType {
    New = b'0' as isize,
    Replace = b'1' as isize,
    Cancel = b'2' as isize,
    Release = b'3' as isize,
    Reverse = b'4' as isize,
}

impl FIXValue for CollAsgnTransType {
    fn from_bytes(bytes: &[u8]) -> Option<CollAsgnTransType> {
        match bytes {
            b"0" => Some(CollAsgnTransType::New),
            b"1" => Some(CollAsgnTransType::Replace),
            b"2" => Some(CollAsgnTransType::Cancel),
            b"3" => Some(CollAsgnTransType::Release),
            b"4" => Some(CollAsgnTransType::Reverse),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollInquiryQualifier {
    Tradedate = b'0' as isize,
    GcInstrument = b'1' as isize,
    Collateralinstrument = b'2' as isize,
    SubstitutionEligible = b'3' as isize,
    NotAssigned = b'4' as isize,
    PartiallyAssigned = b'5' as isize,
    FullyAssigned = b'6' as isize,
    OutstandingTrades = b'7' as isize,
}

impl FIXValue for CollInquiryQualifier {
    fn from_bytes(bytes: &[u8]) -> Option<CollInquiryQualifier> {
        match bytes {
            b"0" => Some(CollInquiryQualifier::Tradedate),
            b"1" => Some(CollInquiryQualifier::GcInstrument),
            b"2" => Some(CollInquiryQualifier::Collateralinstrument),
            b"3" => Some(CollInquiryQualifier::SubstitutionEligible),
            b"4" => Some(CollInquiryQualifier::NotAssigned),
            b"5" => Some(CollInquiryQualifier::PartiallyAssigned),
            b"6" => Some(CollInquiryQualifier::FullyAssigned),
            b"7" => Some(CollInquiryQualifier::OutstandingTrades),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollInquiryResult {
    Successful = b'0' as isize,
    InvalidOrUnknownInstrument = b'1' as isize,
    InvalidOrUnknownCollateralType = b'2' as isize,
    InvalidParties = b'3' as isize,
    InvalidTransportTypeRequested = b'4' as isize,
    InvalidDestinationRequested = b'5' as isize,
    NoCollateralFoundForTheTradeSpecified = b'6' as isize,
    NoCollateralFoundForTheOrderSpecified = b'7' as isize,
    CollateralInquiryTypeNotSupported = b'8' as isize,
    UnauthorizedForCollateralInquiry = b'9' as isize,
    Other,
}

impl FIXValue for CollInquiryResult {
    fn from_bytes(bytes: &[u8]) -> Option<CollInquiryResult> {
        match bytes {
            b"0" => Some(CollInquiryResult::Successful),
            b"1" => Some(CollInquiryResult::InvalidOrUnknownInstrument),
            b"2" => Some(CollInquiryResult::InvalidOrUnknownCollateralType),
            b"3" => Some(CollInquiryResult::InvalidParties),
            b"4" => Some(CollInquiryResult::InvalidTransportTypeRequested),
            b"5" => Some(CollInquiryResult::InvalidDestinationRequested),
            b"6" => Some(CollInquiryResult::NoCollateralFoundForTheTradeSpecified),
            b"7" => Some(CollInquiryResult::NoCollateralFoundForTheOrderSpecified),
            b"8" => Some(CollInquiryResult::CollateralInquiryTypeNotSupported),
            b"9" => Some(CollInquiryResult::UnauthorizedForCollateralInquiry),
            b"99" => Some(CollInquiryResult::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            CollInquiryResult::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollInquiryStatus {
    Accepted = b'0' as isize,
    AcceptedWithWarnings = b'1' as isize,
    Completed = b'2' as isize,
    CompletedWithWarnings = b'3' as isize,
    Rejected = b'4' as isize,
}

impl FIXValue for CollInquiryStatus {
    fn from_bytes(bytes: &[u8]) -> Option<CollInquiryStatus> {
        match bytes {
            b"0" => Some(CollInquiryStatus::Accepted),
            b"1" => Some(CollInquiryStatus::AcceptedWithWarnings),
            b"2" => Some(CollInquiryStatus::Completed),
            b"3" => Some(CollInquiryStatus::CompletedWithWarnings),
            b"4" => Some(CollInquiryStatus::Rejected),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollStatus {
    Unassigned = b'0' as isize,
    PartiallyAssigned = b'1' as isize,
    AssignmentProposed = b'2' as isize,
    Assigned = b'3' as isize,
    Challenged = b'4' as isize,
}

impl FIXValue for CollStatus {
    fn from_bytes(bytes: &[u8]) -> Option<CollStatus> {
        match bytes {
            b"0" => Some(CollStatus::Unassigned),
            b"1" => Some(CollStatus::PartiallyAssigned),
            b"2" => Some(CollStatus::AssignmentProposed),
            b"3" => Some(CollStatus::Assigned),
            b"4" => Some(CollStatus::Challenged),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommType {
    PerUnit = b'1' as isize,
    Percentage = b'2' as isize,
    Absolute = b'3' as isize,
    Four = b'4' as isize,
    Five = b'5' as isize,
    PointsPerBondOrContractSupplyContractmultiplier = b'6' as isize,
}

impl FIXValue for CommType {
    fn from_bytes(bytes: &[u8]) -> Option<CommType> {
        match bytes {
            b"1" => Some(CommType::PerUnit),
            b"2" => Some(CommType::Percentage),
            b"3" => Some(CommType::Absolute),
            b"4" => Some(CommType::Four),
            b"5" => Some(CommType::Five),
            b"6" => Some(CommType::PointsPerBondOrContractSupplyContractmultiplier),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmRejReason {
    MismatchedAccount = b'1' as isize,
    MissingSettlementInstructions = b'2' as isize,
    Other,
}

impl FIXValue for ConfirmRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<ConfirmRejReason> {
        match bytes {
            b"1" => Some(ConfirmRejReason::MismatchedAccount),
            b"2" => Some(ConfirmRejReason::MissingSettlementInstructions),
            b"99" => Some(ConfirmRejReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            ConfirmRejReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmStatus {
    Received = b'1' as isize,
    MismatchedAccount = b'2' as isize,
    MissingSettlementInstructions = b'3' as isize,
    Confirmed = b'4' as isize,
    RequestRejected = b'5' as isize,
}

impl FIXValue for ConfirmStatus {
    fn from_bytes(bytes: &[u8]) -> Option<ConfirmStatus> {
        match bytes {
            b"1" => Some(ConfirmStatus::Received),
            b"2" => Some(ConfirmStatus::MismatchedAccount),
            b"3" => Some(ConfirmStatus::MissingSettlementInstructions),
            b"4" => Some(ConfirmStatus::Confirmed),
            b"5" => Some(ConfirmStatus::RequestRejected),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmTransType {
    New = b'0' as isize,
    Replace = b'1' as isize,
    Cancel = b'2' as isize,
}

impl FIXValue for ConfirmTransType {
    fn from_bytes(bytes: &[u8]) -> Option<ConfirmTransType> {
        match bytes {
            b"0" => Some(ConfirmTransType::New),
            b"1" => Some(ConfirmTransType::Replace),
            b"2" => Some(ConfirmTransType::Cancel),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmType {
    Status = b'1' as isize,
    Confirmation = b'2' as isize,
    ConfirmationRequestRejected = b'3' as isize,
}

impl FIXValue for ConfirmType {
    fn from_bytes(bytes: &[u8]) -> Option<ConfirmType> {
        match bytes {
            b"1" => Some(ConfirmType::Status),
            b"2" => Some(ConfirmType::Confirmation),
            b"3" => Some(ConfirmType::ConfirmationRequestRejected),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossPrioritization {
    None = b'0' as isize,
    BuySideIsPrioritized = b'1' as isize,
    SellSideIsPrioritized = b'2' as isize,
}

impl FIXValue for CrossPrioritization {
    fn from_bytes(bytes: &[u8]) -> Option<CrossPrioritization> {
        match bytes {
            b"0" => Some(CrossPrioritization::None),
            b"1" => Some(CrossPrioritization::BuySideIsPrioritized),
            b"2" => Some(CrossPrioritization::SellSideIsPrioritized),
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
    CrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelledOneSideIsFullyExecutedTheOtherSideIsPartiallyExecutedWithTheRemainderBeingCancelledThisIsEquivalentToAnImmediateOrCancelOnTheOtherSideNoteTheCrossprioritzation = b'2' as isize,
    CrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActiveOneSideOfTheCrossIsFullyExecuted = b'3' as isize,
    CrossTradeIsExecutedWithExistingOrdersWithTheSamePriceInTheCaseOtherOrdersExistWithTheSamePriceTheQuantityOfTheCrossIsExecutedAgainstTheExistingOrdersAndQuotesTheRemainderOfTheCrossIsExecutedAgainstTheOtherSideOfTheCrossTheTwoSidesPotentiallyHaveDifferentQuantities = b'4' as isize,
}

impl FIXValue for CrossType {
    fn from_bytes(bytes: &[u8]) -> Option<CrossType> {
        match bytes {
            b"1" => Some(CrossType::CrossTradeWhichIsExecutedCompletelyOrNotBothSidesAreTreatedInTheSameMannerThisIsEquivalentToAnAllOrNone),
            b"2" => Some(CrossType::CrossTradeWhichIsExecutedPartiallyAndTheRestIsCancelledOneSideIsFullyExecutedTheOtherSideIsPartiallyExecutedWithTheRemainderBeingCancelledThisIsEquivalentToAnImmediateOrCancelOnTheOtherSideNoteTheCrossprioritzation),
            b"3" => Some(CrossType::CrossTradeWhichIsPartiallyExecutedWithTheUnfilledPortionsRemainingActiveOneSideOfTheCrossIsFullyExecuted),
            b"4" => Some(CrossType::CrossTradeIsExecutedWithExistingOrdersWithTheSamePriceInTheCaseOtherOrdersExistWithTheSamePriceTheQuantityOfTheCrossIsExecutedAgainstTheExistingOrdersAndQuotesTheRemainderOfTheCrossIsExecutedAgainstTheOtherSideOfTheCrossTheTwoSidesPotentiallyHaveDifferentQuantities),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustOrderCapacity {
    MemberTradingForTheirOwnAccount = b'1' as isize,
    ClearingFirmTradingForItsProprietaryAccount = b'2' as isize,
    MemberTradingForAnotherMember = b'3' as isize,
    AllOther = b'4' as isize,
}

impl FIXValue for CustOrderCapacity {
    fn from_bytes(bytes: &[u8]) -> Option<CustOrderCapacity> {
        match bytes {
            b"1" => Some(CustOrderCapacity::MemberTradingForTheirOwnAccount),
            b"2" => Some(CustOrderCapacity::ClearingFirmTradingForItsProprietaryAccount),
            b"3" => Some(CustOrderCapacity::MemberTradingForAnotherMember),
            b"4" => Some(CustOrderCapacity::AllOther),
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
    Broker = b'2' as isize,
    OrderAlreadyInPendingCancelOrPendingReplaceStatus = b'3' as isize,
    UnableToProcessOrderMassCancelRequest = b'4' as isize,
    Origordmodtime = b'5' as isize,
    DuplicateClordid = b'6' as isize,
    Other,
}

impl FIXValue for CxlRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<CxlRejReason> {
        match bytes {
            b"0" => Some(CxlRejReason::TooLateToCancel),
            b"1" => Some(CxlRejReason::UnknownOrder),
            b"2" => Some(CxlRejReason::Broker),
            b"3" => Some(CxlRejReason::OrderAlreadyInPendingCancelOrPendingReplaceStatus),
            b"4" => Some(CxlRejReason::UnableToProcessOrderMassCancelRequest),
            b"5" => Some(CxlRejReason::Origordmodtime),
            b"6" => Some(CxlRejReason::DuplicateClordid),
            b"99" => Some(CxlRejReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            CxlRejReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DKReason {
    UnknownSymbol = b'A' as isize,
    WrongSide = b'B' as isize,
    QuantityExceedsOrder = b'C' as isize,
    NoMatchingOrder = b'D' as isize,
    PriceExceedsLimit = b'E' as isize,
    CalculationDifference = b'F' as isize,
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
            b"F" => Some(DKReason::CalculationDifference),
            b"Z" => Some(DKReason::Other),
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
    Accumulate = b'2' as isize,
}

impl FIXValue for DayBookingInst {
    fn from_bytes(bytes: &[u8]) -> Option<DayBookingInst> {
        match bytes {
            b"0" => Some(DayBookingInst::CanTriggerBookingWithoutReferenceToTheOrderInitiator),
            b"1" => Some(DayBookingInst::SpeakWithOrderInitiatorBeforeBooking),
            b"2" => Some(DayBookingInst::Accumulate),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryForm {
    Bookentry = b'1' as isize,
    Bearer = b'2' as isize,
}

impl FIXValue for DeliveryForm {
    fn from_bytes(bytes: &[u8]) -> Option<DeliveryForm> {
        match bytes {
            b"1" => Some(DeliveryForm::Bookentry),
            b"2" => Some(DeliveryForm::Bearer),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryType {
    VersusPaymentDeliver = b'0' as isize,
    FreeDeliver = b'1' as isize,
    TriParty = b'2' as isize,
    HoldInCustody = b'3' as isize,
}

impl FIXValue for DeliveryType {
    fn from_bytes(bytes: &[u8]) -> Option<DeliveryType> {
        match bytes {
            b"0" => Some(DeliveryType::VersusPaymentDeliver),
            b"1" => Some(DeliveryType::FreeDeliver),
            b"2" => Some(DeliveryType::TriParty),
            b"3" => Some(DeliveryType::HoldInCustody),
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
    RelatedToVwap = b'6' as isize,
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
            b"6" => Some(DiscretionInst::RelatedToVwap),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscretionLimitType {
    OrBetter = b'0' as isize,
    StrictLimitIsAStrictLimit = b'1' as isize,
    OrWorseForABuyTheDiscretionPriceIsAMinimumAndForASellTheDiscretionPriceIsAMaximum = b'2' as isize,
}

impl FIXValue for DiscretionLimitType {
    fn from_bytes(bytes: &[u8]) -> Option<DiscretionLimitType> {
        match bytes {
            b"0" => Some(DiscretionLimitType::OrBetter),
            b"1" => Some(DiscretionLimitType::StrictLimitIsAStrictLimit),
            b"2" => Some(DiscretionLimitType::OrWorseForABuyTheDiscretionPriceIsAMinimumAndForASellTheDiscretionPriceIsAMaximum),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscretionMoveType {
    Floating = b'0' as isize,
    Fixed = b'1' as isize,
}

impl FIXValue for DiscretionMoveType {
    fn from_bytes(bytes: &[u8]) -> Option<DiscretionMoveType> {
        match bytes {
            b"0" => Some(DiscretionMoveType::Floating),
            b"1" => Some(DiscretionMoveType::Fixed),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscretionOffsetType {
    Price = b'0' as isize,
    BasisPoints = b'1' as isize,
    Ticks = b'2' as isize,
    PriceTier = b'3' as isize,
}

impl FIXValue for DiscretionOffsetType {
    fn from_bytes(bytes: &[u8]) -> Option<DiscretionOffsetType> {
        match bytes {
            b"0" => Some(DiscretionOffsetType::Price),
            b"1" => Some(DiscretionOffsetType::BasisPoints),
            b"2" => Some(DiscretionOffsetType::Ticks),
            b"3" => Some(DiscretionOffsetType::PriceTier),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscretionRoundDirection {
    MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick = b'1' as isize,
    MorePassiveOnABuyOrderRoundDownToNearestTickOnASellOrderRoundUpToNearestTick = b'2' as isize,
}

impl FIXValue for DiscretionRoundDirection {
    fn from_bytes(bytes: &[u8]) -> Option<DiscretionRoundDirection> {
        match bytes {
            b"1" => Some(DiscretionRoundDirection::MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick),
            b"2" => Some(DiscretionRoundDirection::MorePassiveOnABuyOrderRoundDownToNearestTickOnASellOrderRoundUpToNearestTick),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscretionScope {
    Local = b'1' as isize,
    National = b'2' as isize,
    Global = b'3' as isize,
    NationalExcludingLocal = b'4' as isize,
}

impl FIXValue for DiscretionScope {
    fn from_bytes(bytes: &[u8]) -> Option<DiscretionScope> {
        match bytes {
            b"1" => Some(DiscretionScope::Local),
            b"2" => Some(DiscretionScope::National),
            b"3" => Some(DiscretionScope::Global),
            b"4" => Some(DiscretionScope::NationalExcludingLocal),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistribPaymentMethod {
    Crest = b'1' as isize,
    Nscc = b'2' as isize,
    Euroclear = b'3' as isize,
    Clearstream = b'4' as isize,
    Cheque = b'5' as isize,
    TelegraphicTransfer = b'6' as isize,
    Fedwire = b'7' as isize,
    DirectCredit = b'8' as isize,
    AchCredit = b'9' as isize,
    Bpay,
    HighValueClearingSystem,
    ReinvestInFund,
}

impl FIXValue for DistribPaymentMethod {
    fn from_bytes(bytes: &[u8]) -> Option<DistribPaymentMethod> {
        match bytes {
            b"1" => Some(DistribPaymentMethod::Crest),
            b"2" => Some(DistribPaymentMethod::Nscc),
            b"3" => Some(DistribPaymentMethod::Euroclear),
            b"4" => Some(DistribPaymentMethod::Clearstream),
            b"5" => Some(DistribPaymentMethod::Cheque),
            b"6" => Some(DistribPaymentMethod::TelegraphicTransfer),
            b"7" => Some(DistribPaymentMethod::Fedwire),
            b"8" => Some(DistribPaymentMethod::DirectCredit),
            b"9" => Some(DistribPaymentMethod::AchCredit),
            b"10" => Some(DistribPaymentMethod::Bpay),
            b"11" => Some(DistribPaymentMethod::HighValueClearingSystem),
            b"12" => Some(DistribPaymentMethod::ReinvestInFund),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            DistribPaymentMethod::Bpay => v.extend(b"10"),
            DistribPaymentMethod::HighValueClearingSystem => v.extend(b"11"),
            DistribPaymentMethod::ReinvestInFund => v.extend(b"12"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DlvyInstType {
    Securities = b'S' as isize,
    Cash = b'C' as isize,
}

impl FIXValue for DlvyInstType {
    fn from_bytes(bytes: &[u8]) -> Option<DlvyInstType> {
        match bytes {
            b"S" => Some(DlvyInstType::Securities),
            b"C" => Some(DlvyInstType::Cash),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Put = b'1' as isize,
    Call = b'2' as isize,
    Tender = b'3' as isize,
    SinkingFundCall = b'4' as isize,
    Other,
}

impl FIXValue for EventType {
    fn from_bytes(bytes: &[u8]) -> Option<EventType> {
        match bytes {
            b"1" => Some(EventType::Put),
            b"2" => Some(EventType::Call),
            b"3" => Some(EventType::Tender),
            b"4" => Some(EventType::SinkingFundCall),
            b"99" => Some(EventType::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            EventType::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecInst {
    NotHeld = b'1' as isize,
    Work = b'2' as isize,
    GoAlong = b'3' as isize,
    OverTheDay = b'4' as isize,
    Held = b'5' as isize,
    ParticipateDontInitiate = b'6' as isize,
    StrictScale = b'7' as isize,
    TryToScale = b'8' as isize,
    StayOnBidside = b'9' as isize,
    StayOnOfferside = b'0' as isize,
    NoCross = b'A' as isize,
    OkToCross = b'B' as isize,
    CallFirst = b'C' as isize,
    PercentOfVolume = b'D' as isize,
    DoNotIncrease = b'E' as isize,
    DoNotReduce = b'F' as isize,
    AllOrNone = b'G' as isize,
    ReinstateOnSystemFailure = b'H' as isize,
    InstitutionsOnly = b'I' as isize,
    ReinstateOnTradingHalt = b'J' as isize,
    CancelOnTradingHalt = b'K' as isize,
    LastPeg = b'L' as isize,
    MidPricePeg = b'M' as isize,
    NonNegotiable = b'N' as isize,
    OpeningPeg = b'O' as isize,
    MarketPeg = b'P' as isize,
    CancelOnSystemFailure = b'Q' as isize,
    PrimaryPeg = b'R' as isize,
    Suspend = b'S' as isize,
    CustomerDisplayInstruction = b'U' as isize,
    Netting = b'V' as isize,
    PegToVwap = b'W' as isize,
    TradeAlong = b'X' as isize,
    TryToStop = b'Y' as isize,
    CancelIfNotBest = b'Z' as isize,
    TrailingStopPeg = b'a' as isize,
    StrictLimit = b'b' as isize,
    IgnorePriceValidityChecks = b'c' as isize,
    PegToLimitPrice = b'd' as isize,
    WorkToTargetStrategy = b'e' as isize,
}

impl FIXValue for ExecInst {
    fn from_bytes(bytes: &[u8]) -> Option<ExecInst> {
        match bytes {
            b"1" => Some(ExecInst::NotHeld),
            b"2" => Some(ExecInst::Work),
            b"3" => Some(ExecInst::GoAlong),
            b"4" => Some(ExecInst::OverTheDay),
            b"5" => Some(ExecInst::Held),
            b"6" => Some(ExecInst::ParticipateDontInitiate),
            b"7" => Some(ExecInst::StrictScale),
            b"8" => Some(ExecInst::TryToScale),
            b"9" => Some(ExecInst::StayOnBidside),
            b"0" => Some(ExecInst::StayOnOfferside),
            b"A" => Some(ExecInst::NoCross),
            b"B" => Some(ExecInst::OkToCross),
            b"C" => Some(ExecInst::CallFirst),
            b"D" => Some(ExecInst::PercentOfVolume),
            b"E" => Some(ExecInst::DoNotIncrease),
            b"F" => Some(ExecInst::DoNotReduce),
            b"G" => Some(ExecInst::AllOrNone),
            b"H" => Some(ExecInst::ReinstateOnSystemFailure),
            b"I" => Some(ExecInst::InstitutionsOnly),
            b"J" => Some(ExecInst::ReinstateOnTradingHalt),
            b"K" => Some(ExecInst::CancelOnTradingHalt),
            b"L" => Some(ExecInst::LastPeg),
            b"M" => Some(ExecInst::MidPricePeg),
            b"N" => Some(ExecInst::NonNegotiable),
            b"O" => Some(ExecInst::OpeningPeg),
            b"P" => Some(ExecInst::MarketPeg),
            b"Q" => Some(ExecInst::CancelOnSystemFailure),
            b"R" => Some(ExecInst::PrimaryPeg),
            b"S" => Some(ExecInst::Suspend),
            b"U" => Some(ExecInst::CustomerDisplayInstruction),
            b"V" => Some(ExecInst::Netting),
            b"W" => Some(ExecInst::PegToVwap),
            b"X" => Some(ExecInst::TradeAlong),
            b"Y" => Some(ExecInst::TryToStop),
            b"Z" => Some(ExecInst::CancelIfNotBest),
            b"a" => Some(ExecInst::TrailingStopPeg),
            b"b" => Some(ExecInst::StrictLimit),
            b"c" => Some(ExecInst::IgnorePriceValidityChecks),
            b"d" => Some(ExecInst::PegToLimitPrice),
            b"e" => Some(ExecInst::WorkToTargetStrategy),
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
    CancelOnTradingHalt = b'6' as isize,
    CancelOnSystemFailure = b'7' as isize,
    Market = b'8' as isize,
    CanceledNotBest = b'9' as isize,
    WarehouseRecap,
    Other,
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
            b"6" => Some(ExecRestatementReason::CancelOnTradingHalt),
            b"7" => Some(ExecRestatementReason::CancelOnSystemFailure),
            b"8" => Some(ExecRestatementReason::Market),
            b"9" => Some(ExecRestatementReason::CanceledNotBest),
            b"10" => Some(ExecRestatementReason::WarehouseRecap),
            b"99" => Some(ExecRestatementReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            ExecRestatementReason::WarehouseRecap => v.extend(b"10"),
            ExecRestatementReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecType {
    New = b'0' as isize,
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
    Trade = b'F' as isize,
    TradeCorrect = b'G' as isize,
    TradeCancel = b'H' as isize,
    OrderStatus = b'I' as isize,
}

impl FIXValue for ExecType {
    fn from_bytes(bytes: &[u8]) -> Option<ExecType> {
        match bytes {
            b"0" => Some(ExecType::New),
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
            b"F" => Some(ExecType::Trade),
            b"G" => Some(ExecType::TradeCorrect),
            b"H" => Some(ExecType::TradeCancel),
            b"I" => Some(ExecType::OrderStatus),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExerciseMethod {
    Automatic = b'A' as isize,
    Manual = b'M' as isize,
}

impl FIXValue for ExerciseMethod {
    fn from_bytes(bytes: &[u8]) -> Option<ExerciseMethod> {
        match bytes {
            b"A" => Some(ExerciseMethod::Automatic),
            b"M" => Some(ExerciseMethod::Manual),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpirationCycle {
    ExpireOnTradingSessionClose = b'0' as isize,
    ExpireOnTradingSessionOpen = b'1' as isize,
}

impl FIXValue for ExpirationCycle {
    fn from_bytes(bytes: &[u8]) -> Option<ExpirationCycle> {
        match bytes {
            b"0" => Some(ExpirationCycle::ExpireOnTradingSessionClose),
            b"1" => Some(ExpirationCycle::ExpireOnTradingSessionOpen),
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
    AllOrNone = b'A' as isize,
    MarketOnClose = b'B' as isize,
    AtTheClose = b'C' as isize,
    Vwap = b'D' as isize,
    InTouchWith = b'I' as isize,
    Limit = b'L' as isize,
    MoreBehind = b'M' as isize,
    AtTheOpen = b'O' as isize,
    TakingAPosition = b'P' as isize,
    AtTheMarket = b'Q' as isize,
    ReadyToTrade = b'R' as isize,
    PortfolioShown = b'S' as isize,
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
            b"B" => Some(IOIQualifier::MarketOnClose),
            b"C" => Some(IOIQualifier::AtTheClose),
            b"D" => Some(IOIQualifier::Vwap),
            b"I" => Some(IOIQualifier::InTouchWith),
            b"L" => Some(IOIQualifier::Limit),
            b"M" => Some(IOIQualifier::MoreBehind),
            b"O" => Some(IOIQualifier::AtTheOpen),
            b"P" => Some(IOIQualifier::TakingAPosition),
            b"Q" => Some(IOIQualifier::AtTheMarket),
            b"R" => Some(IOIQualifier::ReadyToTrade),
            b"S" => Some(IOIQualifier::PortfolioShown),
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
pub enum InstrAttribType {
    Flat = b'1' as isize,
    ZeroCoupon = b'2' as isize,
    InterestBearing = b'3' as isize,
    NoPeriodicPayments = b'4' as isize,
    VariableRate = b'5' as isize,
    LessFeeForPut = b'6' as isize,
    SteppedCoupon = b'7' as isize,
    CouponPeriod = b'8' as isize,
    WhenAndIfIssued = b'9' as isize,
    OriginalIssueDiscount,
    CallablePuttable,
    EscrowedToMaturity,
    EscrowedToRedemptionDateCallableSupplyRedemptionDateInTheInstrattribvalue,
    Prerefunded,
    InDefault,
    Unrated,
    Taxable,
    Indexed,
    SubjectToAlternativeMinimumTax,
    OriginalIssueDiscountPriceSupplyPriceInTheInstrattribvalue,
    CallableBelowMaturityValue,
    CallableWithoutNoticeByMailToHolderUnlessRegistered,
    TextSupplyTheTextOfTheAttributeOrDisclaimerInTheInstrattribvalue,
}

impl FIXValue for InstrAttribType {
    fn from_bytes(bytes: &[u8]) -> Option<InstrAttribType> {
        match bytes {
            b"1" => Some(InstrAttribType::Flat),
            b"2" => Some(InstrAttribType::ZeroCoupon),
            b"3" => Some(InstrAttribType::InterestBearing),
            b"4" => Some(InstrAttribType::NoPeriodicPayments),
            b"5" => Some(InstrAttribType::VariableRate),
            b"6" => Some(InstrAttribType::LessFeeForPut),
            b"7" => Some(InstrAttribType::SteppedCoupon),
            b"8" => Some(InstrAttribType::CouponPeriod),
            b"9" => Some(InstrAttribType::WhenAndIfIssued),
            b"10" => Some(InstrAttribType::OriginalIssueDiscount),
            b"11" => Some(InstrAttribType::CallablePuttable),
            b"12" => Some(InstrAttribType::EscrowedToMaturity),
            b"13" => Some(InstrAttribType::EscrowedToRedemptionDateCallableSupplyRedemptionDateInTheInstrattribvalue),
            b"14" => Some(InstrAttribType::Prerefunded),
            b"15" => Some(InstrAttribType::InDefault),
            b"16" => Some(InstrAttribType::Unrated),
            b"17" => Some(InstrAttribType::Taxable),
            b"18" => Some(InstrAttribType::Indexed),
            b"19" => Some(InstrAttribType::SubjectToAlternativeMinimumTax),
            b"20" => Some(InstrAttribType::OriginalIssueDiscountPriceSupplyPriceInTheInstrattribvalue),
            b"21" => Some(InstrAttribType::CallableBelowMaturityValue),
            b"22" => Some(InstrAttribType::CallableWithoutNoticeByMailToHolderUnlessRegistered),
            b"99" => Some(InstrAttribType::TextSupplyTheTextOfTheAttributeOrDisclaimerInTheInstrattribvalue),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            InstrAttribType::OriginalIssueDiscount => v.extend(b"10"),
            InstrAttribType::CallablePuttable => v.extend(b"11"),
            InstrAttribType::EscrowedToMaturity => v.extend(b"12"),
            InstrAttribType::EscrowedToRedemptionDateCallableSupplyRedemptionDateInTheInstrattribvalue => v.extend(b"13"),
            InstrAttribType::Prerefunded => v.extend(b"14"),
            InstrAttribType::InDefault => v.extend(b"15"),
            InstrAttribType::Unrated => v.extend(b"16"),
            InstrAttribType::Taxable => v.extend(b"17"),
            InstrAttribType::Indexed => v.extend(b"18"),
            InstrAttribType::SubjectToAlternativeMinimumTax => v.extend(b"19"),
            InstrAttribType::OriginalIssueDiscountPriceSupplyPriceInTheInstrattribvalue => v.extend(b"20"),
            InstrAttribType::CallableBelowMaturityValue => v.extend(b"21"),
            InstrAttribType::CallableWithoutNoticeByMailToHolderUnlessRegistered => v.extend(b"22"),
            InstrAttribType::TextSupplyTheTextOfTheAttributeOrDisclaimerInTheInstrattribvalue => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastFragment {
    Yes = b'Y' as isize,
    No = b'N' as isize,
}

impl FIXValue for LastFragment {
    fn from_bytes(bytes: &[u8]) -> Option<LastFragment> {
        match bytes {
            b"Y" => Some(LastFragment::Yes),
            b"N" => Some(LastFragment::No),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastLiquidityInd {
    AddedLiquidity = b'1' as isize,
    RemovedLiquidity = b'2' as isize,
    LiquidityRoutedOut = b'3' as isize,
}

impl FIXValue for LastLiquidityInd {
    fn from_bytes(bytes: &[u8]) -> Option<LastLiquidityInd> {
        match bytes {
            b"1" => Some(LastLiquidityInd::AddedLiquidity),
            b"2" => Some(LastLiquidityInd::RemovedLiquidity),
            b"3" => Some(LastLiquidityInd::LiquidityRoutedOut),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegSwapType {
    ParForPar = b'1' as isize,
    ModifiedDuration = b'2' as isize,
    Risk = b'4' as isize,
    Proceeds = b'5' as isize,
}

impl FIXValue for LegSwapType {
    fn from_bytes(bytes: &[u8]) -> Option<LegSwapType> {
        match bytes {
            b"1" => Some(LegSwapType::ParForPar),
            b"2" => Some(LegSwapType::ModifiedDuration),
            b"4" => Some(LegSwapType::Risk),
            b"5" => Some(LegSwapType::Proceeds),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidityIndType {
    FivedayMovingAverage = b'1' as isize,
    TwentyDayMovingAverage = b'2' as isize,
    NormalMarketSize = b'3' as isize,
    Other = b'4' as isize,
}

impl FIXValue for LiquidityIndType {
    fn from_bytes(bytes: &[u8]) -> Option<LiquidityIndType> {
        match bytes {
            b"1" => Some(LiquidityIndType::FivedayMovingAverage),
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
    ExchangeSwitchCivOrderSellDriven = b'3' as isize,
    ExchangeSwitchCivOrderBuyDrivenCashTopUp = b'4' as isize,
    ExchangeSwitchCivOrderBuyDrivenCashWithdraw = b'5' as isize,
}

impl FIXValue for ListExecInstType {
    fn from_bytes(bytes: &[u8]) -> Option<ListExecInstType> {
        match bytes {
            b"1" => Some(ListExecInstType::Immediate),
            b"2" => Some(ListExecInstType::WaitForExecuteInstruction),
            b"3" => Some(ListExecInstType::ExchangeSwitchCivOrderSellDriven),
            b"4" => Some(ListExecInstType::ExchangeSwitchCivOrderBuyDrivenCashTopUp),
            b"5" => Some(ListExecInstType::ExchangeSwitchCivOrderBuyDrivenCashWithdraw),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListOrderStatus {
    Inbiddingprocess = b'1' as isize,
    Receivedforexecution = b'2' as isize,
    Executing = b'3' as isize,
    Canceling = b'4' as isize,
    Alert = b'5' as isize,
    AllDone = b'6' as isize,
    Reject = b'7' as isize,
}

impl FIXValue for ListOrderStatus {
    fn from_bytes(bytes: &[u8]) -> Option<ListOrderStatus> {
        match bytes {
            b"1" => Some(ListOrderStatus::Inbiddingprocess),
            b"2" => Some(ListOrderStatus::Receivedforexecution),
            b"3" => Some(ListOrderStatus::Executing),
            b"4" => Some(ListOrderStatus::Canceling),
            b"5" => Some(ListOrderStatus::Alert),
            b"6" => Some(ListOrderStatus::AllDone),
            b"7" => Some(ListOrderStatus::Reject),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListStatusType {
    Ack = b'1' as isize,
    Response = b'2' as isize,
    Timed = b'3' as isize,
    Execstarted = b'4' as isize,
    Alldone = b'5' as isize,
    Alert = b'6' as isize,
}

impl FIXValue for ListStatusType {
    fn from_bytes(bytes: &[u8]) -> Option<ListStatusType> {
        match bytes {
            b"1" => Some(ListStatusType::Ack),
            b"2" => Some(ListStatusType::Response),
            b"3" => Some(ListStatusType::Timed),
            b"4" => Some(ListStatusType::Execstarted),
            b"5" => Some(ListStatusType::Alldone),
            b"6" => Some(ListStatusType::Alert),
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
    Imbalance = b'A' as isize,
    TradeVolume = b'B' as isize,
    OpenInterest = b'C' as isize,
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
            b"A" => Some(MDEntryType::Imbalance),
            b"B" => Some(MDEntryType::TradeVolume),
            b"C" => Some(MDEntryType::OpenInterest),
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
    UnsupportedTradingsessionid = b'9' as isize,
    UnsupportedScope = b'A' as isize,
    UnsupportedOpenclosesettleflag = b'B' as isize,
    UnsupportedMdimplicitdelete = b'C' as isize,
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
            b"9" => Some(MDReqRejReason::UnsupportedTradingsessionid),
            b"A" => Some(MDReqRejReason::UnsupportedScope),
            b"B" => Some(MDReqRejReason::UnsupportedOpenclosesettleflag),
            b"C" => Some(MDReqRejReason::UnsupportedMdimplicitdelete),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassCancelRejectReason {
    MassCancelNotSupported = b'0' as isize,
    InvalidOrUnknownSecurity = b'1' as isize,
    InvalidOrUnknownUnderlying = b'2' as isize,
    InvalidOrUnknownProduct = b'3' as isize,
    InvalidOrUnknownCficode = b'4' as isize,
    InvalidOrUnknownSecurityType = b'5' as isize,
    InvalidOrUnknownTradingSession = b'6' as isize,
    Other,
}

impl FIXValue for MassCancelRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<MassCancelRejectReason> {
        match bytes {
            b"0" => Some(MassCancelRejectReason::MassCancelNotSupported),
            b"1" => Some(MassCancelRejectReason::InvalidOrUnknownSecurity),
            b"2" => Some(MassCancelRejectReason::InvalidOrUnknownUnderlying),
            b"3" => Some(MassCancelRejectReason::InvalidOrUnknownProduct),
            b"4" => Some(MassCancelRejectReason::InvalidOrUnknownCficode),
            b"5" => Some(MassCancelRejectReason::InvalidOrUnknownSecurityType),
            b"6" => Some(MassCancelRejectReason::InvalidOrUnknownTradingSession),
            b"99" => Some(MassCancelRejectReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            MassCancelRejectReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime,
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges,
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime,
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadges,
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime,
    ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAcceptsPairOffs,
    ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorMinusBadgesAndTimesActM1Match,
    SummarizedMatchMinusBadgesAndTimesActM2Match,
    ActAcceptedTrade,
    ActDefaultTrade,
    ActDefaultAfterM2,
    ActM6Match,
    OcsLockedInNonAct,
    SummarizedMatchUsingA1ExactMatchCriteriaExceptQuantityIsSummarized,
    SummarizedMatchUsingA2ExactMatchCriteriaExceptQuantityIsSummarized,
    SummarizedMatchUsingA3ExactMatchCriteriaExceptQuantityIsSummarized,
    SummarizedMatchUsingA4ExactMatchCriteriaExceptQuantityIsSummarized,
    SummarizedMatchUsingA5ExactMatchCriteriaExceptQuantityIsSummarized,
}

impl FIXValue for MatchType {
    fn from_bytes(bytes: &[u8]) -> Option<MatchType> {
        match bytes {
            b"A1" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime),
            b"A2" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges),
            b"A3" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime),
            b"A4" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadges),
            b"A5" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime),
            b"AQ" => Some(MatchType::ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAcceptsPairOffs),
            b"M1" => Some(MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorMinusBadgesAndTimesActM1Match),
            b"M2" => Some(MatchType::SummarizedMatchMinusBadgesAndTimesActM2Match),
            b"M3" => Some(MatchType::ActAcceptedTrade),
            b"M4" => Some(MatchType::ActDefaultTrade),
            b"M5" => Some(MatchType::ActDefaultAfterM2),
            b"M6" => Some(MatchType::ActM6Match),
            b"MT" => Some(MatchType::OcsLockedInNonAct),
            b"S1" => Some(MatchType::SummarizedMatchUsingA1ExactMatchCriteriaExceptQuantityIsSummarized),
            b"S2" => Some(MatchType::SummarizedMatchUsingA2ExactMatchCriteriaExceptQuantityIsSummarized),
            b"S3" => Some(MatchType::SummarizedMatchUsingA3ExactMatchCriteriaExceptQuantityIsSummarized),
            b"S4" => Some(MatchType::SummarizedMatchUsingA4ExactMatchCriteriaExceptQuantityIsSummarized),
            b"S5" => Some(MatchType::SummarizedMatchUsingA5ExactMatchCriteriaExceptQuantityIsSummarized),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadgesAndExecutionTime => v.extend(b"A1"),
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusFourBadges => v.extend(b"A2"),
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadgesAndExecutionTime => v.extend(b"A3"),
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusTwoBadges => v.extend(b"A4"),
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorPlusExecutionTime => v.extend(b"A5"),
            MatchType::ComparedRecordsResultingFromStampedAdvisoriesOrSpecialistAcceptsPairOffs => v.extend(b"AQ"),
            MatchType::ExactMatchOnTradeDateStockSymbolQuantityPriceTradeTypeAndSpecialTradeIndicatorMinusBadgesAndTimesActM1Match => v.extend(b"M1"),
            MatchType::SummarizedMatchMinusBadgesAndTimesActM2Match => v.extend(b"M2"),
            MatchType::ActAcceptedTrade => v.extend(b"M3"),
            MatchType::ActDefaultTrade => v.extend(b"M4"),
            MatchType::ActDefaultAfterM2 => v.extend(b"M5"),
            MatchType::ActM6Match => v.extend(b"M6"),
            MatchType::OcsLockedInNonAct => v.extend(b"MT"),
            MatchType::SummarizedMatchUsingA1ExactMatchCriteriaExceptQuantityIsSummarized => v.extend(b"S1"),
            MatchType::SummarizedMatchUsingA2ExactMatchCriteriaExceptQuantityIsSummarized => v.extend(b"S2"),
            MatchType::SummarizedMatchUsingA3ExactMatchCriteriaExceptQuantityIsSummarized => v.extend(b"S3"),
            MatchType::SummarizedMatchUsingA4ExactMatchCriteriaExceptQuantityIsSummarized => v.extend(b"S4"),
            MatchType::SummarizedMatchUsingA5ExactMatchCriteriaExceptQuantityIsSummarized => v.extend(b"S5"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageEncoding {
    Euc,
    Jis,
    ForUsingSjis,
    Unicode,
}

impl FIXValue for MessageEncoding {
    fn from_bytes(bytes: &[u8]) -> Option<MessageEncoding> {
        match bytes {
            b"EUC-JP" => Some(MessageEncoding::Euc),
            b"ISO-2022-JP" => Some(MessageEncoding::Jis),
            b"Shift_JIS" => Some(MessageEncoding::ForUsingSjis),
            b"UTF-8" => Some(MessageEncoding::Unicode),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            MessageEncoding::Euc => v.extend(b"EUC-JP"),
            MessageEncoding::Jis => v.extend(b"ISO-2022-JP"),
            MessageEncoding::ForUsingSjis => v.extend(b"Shift_JIS"),
            MessageEncoding::Unicode => v.extend(b"UTF-8"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiscFeeBasis {
    Absolute = b'0' as isize,
    PerUnit = b'1' as isize,
    Percentage = b'2' as isize,
}

impl FIXValue for MiscFeeBasis {
    fn from_bytes(bytes: &[u8]) -> Option<MiscFeeBasis> {
        match bytes {
            b"0" => Some(MiscFeeBasis::Absolute),
            b"1" => Some(MiscFeeBasis::PerUnit),
            b"2" => Some(MiscFeeBasis::Percentage),
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
    ConsumptionTax = b'9' as isize,
    PerTransaction,
    Conversion,
    Agent,
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
            b"10" => Some(MiscFeeType::PerTransaction),
            b"11" => Some(MiscFeeType::Conversion),
            b"12" => Some(MiscFeeType::Agent),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            MiscFeeType::PerTransaction => v.extend(b"10"),
            MiscFeeType::Conversion => v.extend(b"11"),
            MiscFeeType::Agent => v.extend(b"12"),
            _ => v.push(*self as u8)
        }
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
    AllocationInstruction = b'J' as isize,
    ListCancelRequest = b'K' as isize,
    ListExecute = b'L' as isize,
    ListStatusRequest = b'M' as isize,
    ListStatus = b'N' as isize,
    AllocationInstructionAck = b'P' as isize,
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
    QuoteResponse,
    Confirmation,
    PositionMaintenanceRequest,
    PositionMaintenanceReport,
    RequestForPositions,
    RequestForPositionsAck,
    PositionReport,
    TradeCaptureReportRequestAck,
    TradeCaptureReportAck,
    AllocationReport,
    AllocationReportAck,
    ConfirmationAck,
    SettlementInstructionRequest,
    AssignmentReport,
    CollateralRequest,
    CollateralAssignment,
    CollateralResponse,
    CollateralReport,
    CollateralInquiry,
    NetworkBc,
    NetworkBd,
    UserRequest,
    UserResponse,
    CollateralInquiryAck,
    ConfirmationRequest,
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
            b"J" => Some(MsgType::AllocationInstruction),
            b"K" => Some(MsgType::ListCancelRequest),
            b"L" => Some(MsgType::ListExecute),
            b"M" => Some(MsgType::ListStatusRequest),
            b"N" => Some(MsgType::ListStatus),
            b"P" => Some(MsgType::AllocationInstructionAck),
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
            b"AJ" => Some(MsgType::QuoteResponse),
            b"AK" => Some(MsgType::Confirmation),
            b"AL" => Some(MsgType::PositionMaintenanceRequest),
            b"AM" => Some(MsgType::PositionMaintenanceReport),
            b"AN" => Some(MsgType::RequestForPositions),
            b"AO" => Some(MsgType::RequestForPositionsAck),
            b"AP" => Some(MsgType::PositionReport),
            b"AQ" => Some(MsgType::TradeCaptureReportRequestAck),
            b"AR" => Some(MsgType::TradeCaptureReportAck),
            b"AS" => Some(MsgType::AllocationReport),
            b"AT" => Some(MsgType::AllocationReportAck),
            b"AU" => Some(MsgType::ConfirmationAck),
            b"AV" => Some(MsgType::SettlementInstructionRequest),
            b"AW" => Some(MsgType::AssignmentReport),
            b"AX" => Some(MsgType::CollateralRequest),
            b"AY" => Some(MsgType::CollateralAssignment),
            b"AZ" => Some(MsgType::CollateralResponse),
            b"BA" => Some(MsgType::CollateralReport),
            b"BB" => Some(MsgType::CollateralInquiry),
            b"BC" => Some(MsgType::NetworkBc),
            b"BD" => Some(MsgType::NetworkBd),
            b"BE" => Some(MsgType::UserRequest),
            b"BF" => Some(MsgType::UserResponse),
            b"BG" => Some(MsgType::CollateralInquiryAck),
            b"BH" => Some(MsgType::ConfirmationRequest),
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
            MsgType::QuoteResponse => v.extend(b"AJ"),
            MsgType::Confirmation => v.extend(b"AK"),
            MsgType::PositionMaintenanceRequest => v.extend(b"AL"),
            MsgType::PositionMaintenanceReport => v.extend(b"AM"),
            MsgType::RequestForPositions => v.extend(b"AN"),
            MsgType::RequestForPositionsAck => v.extend(b"AO"),
            MsgType::PositionReport => v.extend(b"AP"),
            MsgType::TradeCaptureReportRequestAck => v.extend(b"AQ"),
            MsgType::TradeCaptureReportAck => v.extend(b"AR"),
            MsgType::AllocationReport => v.extend(b"AS"),
            MsgType::AllocationReportAck => v.extend(b"AT"),
            MsgType::ConfirmationAck => v.extend(b"AU"),
            MsgType::SettlementInstructionRequest => v.extend(b"AV"),
            MsgType::AssignmentReport => v.extend(b"AW"),
            MsgType::CollateralRequest => v.extend(b"AX"),
            MsgType::CollateralAssignment => v.extend(b"AY"),
            MsgType::CollateralResponse => v.extend(b"AZ"),
            MsgType::CollateralReport => v.extend(b"BA"),
            MsgType::CollateralInquiry => v.extend(b"BB"),
            MsgType::NetworkBc => v.extend(b"BC"),
            MsgType::NetworkBd => v.extend(b"BD"),
            MsgType::UserRequest => v.extend(b"BE"),
            MsgType::UserResponse => v.extend(b"BF"),
            MsgType::CollateralInquiryAck => v.extend(b"BG"),
            MsgType::ConfirmationRequest => v.extend(b"BH"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiLegRptTypeReq {
    ReportByMulitlegSecurityOnly = b'0' as isize,
    ReportByMultilegSecurityAndByInstrumentLegsBelongingToTheMultilegSecurity = b'1' as isize,
    ReportByInstrumentLegsBelongingToTheMultilegSecurityOnly = b'2' as isize,
}

impl FIXValue for MultiLegRptTypeReq {
    fn from_bytes(bytes: &[u8]) -> Option<MultiLegRptTypeReq> {
        match bytes {
            b"0" => Some(MultiLegRptTypeReq::ReportByMulitlegSecurityOnly),
            b"1" => Some(MultiLegRptTypeReq::ReportByMultilegSecurityAndByInstrumentLegsBelongingToTheMultilegSecurity),
            b"2" => Some(MultiLegRptTypeReq::ReportByInstrumentLegsBelongingToTheMultilegSecurityOnly),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkRequestType {
    Snapshot = b'1' as isize,
    Subscribe = b'2' as isize,
    StopSubscribing = b'4' as isize,
    LevelOfDetailThenNocompidsBecomesRequired = b'8' as isize,
}

impl FIXValue for NetworkRequestType {
    fn from_bytes(bytes: &[u8]) -> Option<NetworkRequestType> {
        match bytes {
            b"1" => Some(NetworkRequestType::Snapshot),
            b"2" => Some(NetworkRequestType::Subscribe),
            b"4" => Some(NetworkRequestType::StopSubscribing),
            b"8" => Some(NetworkRequestType::LevelOfDetailThenNocompidsBecomesRequired),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkStatusResponseType {
    Full = b'1' as isize,
    IncrementalUpdate = b'2' as isize,
}

impl FIXValue for NetworkStatusResponseType {
    fn from_bytes(bytes: &[u8]) -> Option<NetworkStatusResponseType> {
        match bytes {
            b"1" => Some(NetworkStatusResponseType::Full),
            b"2" => Some(NetworkStatusResponseType::IncrementalUpdate),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OddLot {
    Yes = b'Y' as isize,
    No = b'N' as isize,
}

impl FIXValue for OddLot {
    fn from_bytes(bytes: &[u8]) -> Option<OddLot> {
        match bytes {
            b"Y" => Some(OddLot::Yes),
            b"N" => Some(OddLot::No),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenCloseSettlFlag {
    DailyOpen = b'0' as isize,
    SessionOpen = b'1' as isize,
    DeliverySettlementEntry = b'2' as isize,
    ExpectedEntry = b'3' as isize,
    EntryFromPreviousBusinessDay = b'4' as isize,
    TheoreticalPriceValue = b'5' as isize,
}

impl FIXValue for OpenCloseSettlFlag {
    fn from_bytes(bytes: &[u8]) -> Option<OpenCloseSettlFlag> {
        match bytes {
            b"0" => Some(OpenCloseSettlFlag::DailyOpen),
            b"1" => Some(OpenCloseSettlFlag::SessionOpen),
            b"2" => Some(OpenCloseSettlFlag::DeliverySettlementEntry),
            b"3" => Some(OpenCloseSettlFlag::ExpectedEntry),
            b"4" => Some(OpenCloseSettlFlag::EntryFromPreviousBusinessDay),
            b"5" => Some(OpenCloseSettlFlag::TheoreticalPriceValue),
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
    UnsupportedOrderCharacteristic12SurveillenceOption,
    IncorrectQuantity,
    IncorrectAllocatedQuantity,
    UnknownAccount,
    Other,
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
            b"11" => Some(OrdRejReason::UnsupportedOrderCharacteristic12SurveillenceOption),
            b"13" => Some(OrdRejReason::IncorrectQuantity),
            b"14" => Some(OrdRejReason::IncorrectAllocatedQuantity),
            b"15" => Some(OrdRejReason::UnknownAccount),
            b"99" => Some(OrdRejReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            OrdRejReason::InvalidInvestorId => v.extend(b"10"),
            OrdRejReason::UnsupportedOrderCharacteristic12SurveillenceOption => v.extend(b"11"),
            OrdRejReason::IncorrectQuantity => v.extend(b"13"),
            OrdRejReason::IncorrectAllocatedQuantity => v.extend(b"14"),
            OrdRejReason::UnknownAccount => v.extend(b"15"),
            OrdRejReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdStatus {
    New = b'0' as isize,
    PartiallyFilled = b'1' as isize,
    Filled = b'2' as isize,
    DoneForDay = b'3' as isize,
    Canceled = b'4' as isize,
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
    WithOrWithout = b'6' as isize,
    LimitOrBetter = b'7' as isize,
    LimitWithOrWithout = b'8' as isize,
    OnBasis = b'9' as isize,
    PreviouslyQuoted = b'D' as isize,
    PreviouslyIndicated = b'E' as isize,
    Forex = b'G' as isize,
    Funari = b'I' as isize,
    MarketIfTouched = b'J' as isize,
    MarketWithLeftoverAsLimit = b'K' as isize,
    PreviousFundValuationPoint = b'L' as isize,
    NextFundValuationPoint = b'M' as isize,
    Pegged = b'P' as isize,
}

impl FIXValue for OrdType {
    fn from_bytes(bytes: &[u8]) -> Option<OrdType> {
        match bytes {
            b"1" => Some(OrdType::Market),
            b"2" => Some(OrdType::Limit),
            b"3" => Some(OrdType::Stop),
            b"4" => Some(OrdType::StopLimit),
            b"6" => Some(OrdType::WithOrWithout),
            b"7" => Some(OrdType::LimitOrBetter),
            b"8" => Some(OrdType::LimitWithOrWithout),
            b"9" => Some(OrdType::OnBasis),
            b"D" => Some(OrdType::PreviouslyQuoted),
            b"E" => Some(OrdType::PreviouslyIndicated),
            b"G" => Some(OrdType::Forex),
            b"I" => Some(OrdType::Funari),
            b"J" => Some(OrdType::MarketIfTouched),
            b"K" => Some(OrdType::MarketWithLeftoverAsLimit),
            b"L" => Some(OrdType::PreviousFundValuationPoint),
            b"M" => Some(OrdType::NextFundValuationPoint),
            b"P" => Some(OrdType::Pegged),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipType {
    JointInvestors = b'J' as isize,
    TenantsInCommon = b'T' as isize,
    JointTrustees = b'2' as isize,
}

impl FIXValue for OwnershipType {
    fn from_bytes(bytes: &[u8]) -> Option<OwnershipType> {
        match bytes {
            b"J" => Some(OwnershipType::JointInvestors),
            b"T" => Some(OwnershipType::TenantsInCommon),
            b"2" => Some(OwnershipType::JointTrustees),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyIDSource {
    Bic = b'B' as isize,
    GenerallyAcceptedMarketParticipantIdentifier = b'C' as isize,
    ProprietaryCustomCode = b'D' as isize,
    IsoCountryCode = b'E' as isize,
    SettlementEntityLocation = b'F' as isize,
    Mic = b'G' as isize,
    CsdParticipantMemberCode = b'H' as isize,
    KoreanInvestorId = b'1' as isize,
    TaiwaneseQualifiedForeignInvestorIdQfii = b'2' as isize,
    TaiwaneseTradingAccount = b'3' as isize,
    MalaysianCentralDepository = b'4' as isize,
    ChineseBShare = b'5' as isize,
    UkNationalInsuranceOrPensionNumber = b'6' as isize,
    UsSocialSecurityNumber = b'7' as isize,
    UsEmployerIdentificationNumber = b'8' as isize,
    AustralianBusinessNumber = b'9' as isize,
    AustralianTaxFileNumber = b'A' as isize,
    DirectedBrokerThreeCharacterAcronymAsDefinedInIsitcEtcBestPracticeGuidelinesDocument = b'I' as isize,
}

impl FIXValue for PartyIDSource {
    fn from_bytes(bytes: &[u8]) -> Option<PartyIDSource> {
        match bytes {
            b"B" => Some(PartyIDSource::Bic),
            b"C" => Some(PartyIDSource::GenerallyAcceptedMarketParticipantIdentifier),
            b"D" => Some(PartyIDSource::ProprietaryCustomCode),
            b"E" => Some(PartyIDSource::IsoCountryCode),
            b"F" => Some(PartyIDSource::SettlementEntityLocation),
            b"G" => Some(PartyIDSource::Mic),
            b"H" => Some(PartyIDSource::CsdParticipantMemberCode),
            b"1" => Some(PartyIDSource::KoreanInvestorId),
            b"2" => Some(PartyIDSource::TaiwaneseQualifiedForeignInvestorIdQfii),
            b"3" => Some(PartyIDSource::TaiwaneseTradingAccount),
            b"4" => Some(PartyIDSource::MalaysianCentralDepository),
            b"5" => Some(PartyIDSource::ChineseBShare),
            b"6" => Some(PartyIDSource::UkNationalInsuranceOrPensionNumber),
            b"7" => Some(PartyIDSource::UsSocialSecurityNumber),
            b"8" => Some(PartyIDSource::UsEmployerIdentificationNumber),
            b"9" => Some(PartyIDSource::AustralianBusinessNumber),
            b"A" => Some(PartyIDSource::AustralianTaxFileNumber),
            b"I" => Some(PartyIDSource::DirectedBrokerThreeCharacterAcronymAsDefinedInIsitcEtcBestPracticeGuidelinesDocument),
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
    ClearingOrganization,
    Exchange,
    CustomerAccount,
    CorrespondentClearingOrganization,
    CorrespondentBroker,
    BuyerSeller,
    Custodian,
    Intermediary,
    Agent,
    SubCustodian,
    Beneficiary,
    InterestedParty,
    RegulatoryBody,
    LiquidityProvider,
    EnteringTrader,
    ContraTrader,
    PositionAccount,
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
            b"21" => Some(PartyRole::ClearingOrganization),
            b"22" => Some(PartyRole::Exchange),
            b"24" => Some(PartyRole::CustomerAccount),
            b"25" => Some(PartyRole::CorrespondentClearingOrganization),
            b"26" => Some(PartyRole::CorrespondentBroker),
            b"27" => Some(PartyRole::BuyerSeller),
            b"28" => Some(PartyRole::Custodian),
            b"29" => Some(PartyRole::Intermediary),
            b"30" => Some(PartyRole::Agent),
            b"31" => Some(PartyRole::SubCustodian),
            b"32" => Some(PartyRole::Beneficiary),
            b"33" => Some(PartyRole::InterestedParty),
            b"34" => Some(PartyRole::RegulatoryBody),
            b"35" => Some(PartyRole::LiquidityProvider),
            b"36" => Some(PartyRole::EnteringTrader),
            b"37" => Some(PartyRole::ContraTrader),
            b"38" => Some(PartyRole::PositionAccount),
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
            PartyRole::ClearingOrganization => v.extend(b"21"),
            PartyRole::Exchange => v.extend(b"22"),
            PartyRole::CustomerAccount => v.extend(b"24"),
            PartyRole::CorrespondentClearingOrganization => v.extend(b"25"),
            PartyRole::CorrespondentBroker => v.extend(b"26"),
            PartyRole::BuyerSeller => v.extend(b"27"),
            PartyRole::Custodian => v.extend(b"28"),
            PartyRole::Intermediary => v.extend(b"29"),
            PartyRole::Agent => v.extend(b"30"),
            PartyRole::SubCustodian => v.extend(b"31"),
            PartyRole::Beneficiary => v.extend(b"32"),
            PartyRole::InterestedParty => v.extend(b"33"),
            PartyRole::RegulatoryBody => v.extend(b"34"),
            PartyRole::LiquidityProvider => v.extend(b"35"),
            PartyRole::EnteringTrader => v.extend(b"36"),
            PartyRole::ContraTrader => v.extend(b"37"),
            PartyRole::PositionAccount => v.extend(b"38"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartySubIDType {
    Firm = b'1' as isize,
    Person = b'2' as isize,
    System = b'3' as isize,
    Application = b'4' as isize,
    FullLegalNameOfFirm = b'5' as isize,
    PostalAddress = b'6' as isize,
    PhoneNumber = b'7' as isize,
    EmailAddress = b'8' as isize,
    ContactName = b'9' as isize,
    SecuritiesAccountNumber,
    RegistrationNumber,
    RegisteredAddress12,
    RegulatoryStatus,
    RegistrationName,
    CashAccountNumber,
    Bic,
    CsdParticipantMemberCode,
    RegisteredAddress18,
    FundAccountName,
    TelexNumber,
    FaxNumber,
    SecuritiesAccountName,
    CashAccountName,
    Department,
    Location,
    PositionAccountType,
}

impl FIXValue for PartySubIDType {
    fn from_bytes(bytes: &[u8]) -> Option<PartySubIDType> {
        match bytes {
            b"1" => Some(PartySubIDType::Firm),
            b"2" => Some(PartySubIDType::Person),
            b"3" => Some(PartySubIDType::System),
            b"4" => Some(PartySubIDType::Application),
            b"5" => Some(PartySubIDType::FullLegalNameOfFirm),
            b"6" => Some(PartySubIDType::PostalAddress),
            b"7" => Some(PartySubIDType::PhoneNumber),
            b"8" => Some(PartySubIDType::EmailAddress),
            b"9" => Some(PartySubIDType::ContactName),
            b"10" => Some(PartySubIDType::SecuritiesAccountNumber),
            b"11" => Some(PartySubIDType::RegistrationNumber),
            b"12" => Some(PartySubIDType::RegisteredAddress12),
            b"13" => Some(PartySubIDType::RegulatoryStatus),
            b"14" => Some(PartySubIDType::RegistrationName),
            b"15" => Some(PartySubIDType::CashAccountNumber),
            b"16" => Some(PartySubIDType::Bic),
            b"17" => Some(PartySubIDType::CsdParticipantMemberCode),
            b"18" => Some(PartySubIDType::RegisteredAddress18),
            b"19" => Some(PartySubIDType::FundAccountName),
            b"20" => Some(PartySubIDType::TelexNumber),
            b"21" => Some(PartySubIDType::FaxNumber),
            b"22" => Some(PartySubIDType::SecuritiesAccountName),
            b"23" => Some(PartySubIDType::CashAccountName),
            b"24" => Some(PartySubIDType::Department),
            b"25" => Some(PartySubIDType::Location),
            b"26" => Some(PartySubIDType::PositionAccountType),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            PartySubIDType::SecuritiesAccountNumber => v.extend(b"10"),
            PartySubIDType::RegistrationNumber => v.extend(b"11"),
            PartySubIDType::RegisteredAddress12 => v.extend(b"12"),
            PartySubIDType::RegulatoryStatus => v.extend(b"13"),
            PartySubIDType::RegistrationName => v.extend(b"14"),
            PartySubIDType::CashAccountNumber => v.extend(b"15"),
            PartySubIDType::Bic => v.extend(b"16"),
            PartySubIDType::CsdParticipantMemberCode => v.extend(b"17"),
            PartySubIDType::RegisteredAddress18 => v.extend(b"18"),
            PartySubIDType::FundAccountName => v.extend(b"19"),
            PartySubIDType::TelexNumber => v.extend(b"20"),
            PartySubIDType::FaxNumber => v.extend(b"21"),
            PartySubIDType::SecuritiesAccountName => v.extend(b"22"),
            PartySubIDType::CashAccountName => v.extend(b"23"),
            PartySubIDType::Department => v.extend(b"24"),
            PartySubIDType::Location => v.extend(b"25"),
            PartySubIDType::PositionAccountType => v.extend(b"26"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PegLimitType {
    OrBetter = b'0' as isize,
    StrictLimitIsAStrictLimit = b'1' as isize,
    OrWorseForABuyThePegLimitIsAMinimumAndForASellThePegLimitIsAMaximum = b'2' as isize,
}

impl FIXValue for PegLimitType {
    fn from_bytes(bytes: &[u8]) -> Option<PegLimitType> {
        match bytes {
            b"0" => Some(PegLimitType::OrBetter),
            b"1" => Some(PegLimitType::StrictLimitIsAStrictLimit),
            b"2" => Some(PegLimitType::OrWorseForABuyThePegLimitIsAMinimumAndForASellThePegLimitIsAMaximum),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PegMoveType {
    Floating = b'0' as isize,
    Fixed = b'1' as isize,
}

impl FIXValue for PegMoveType {
    fn from_bytes(bytes: &[u8]) -> Option<PegMoveType> {
        match bytes {
            b"0" => Some(PegMoveType::Floating),
            b"1" => Some(PegMoveType::Fixed),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PegOffsetType {
    Price = b'0' as isize,
    BasisPoints = b'1' as isize,
    Ticks = b'2' as isize,
    PriceTier = b'3' as isize,
}

impl FIXValue for PegOffsetType {
    fn from_bytes(bytes: &[u8]) -> Option<PegOffsetType> {
        match bytes {
            b"0" => Some(PegOffsetType::Price),
            b"1" => Some(PegOffsetType::BasisPoints),
            b"2" => Some(PegOffsetType::Ticks),
            b"3" => Some(PegOffsetType::PriceTier),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PegRoundDirection {
    MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick = b'1' as isize,
    MorePassiveOnABuyOrderRoundDownToNearestTickOnASellOrderRoundUpToNearestTick = b'2' as isize,
}

impl FIXValue for PegRoundDirection {
    fn from_bytes(bytes: &[u8]) -> Option<PegRoundDirection> {
        match bytes {
            b"1" => Some(PegRoundDirection::MoreAggressiveOnABuyOrderRoundThePriceUpRoundUpToTheNearestTickOnASellRoundDownToTheNearestTick),
            b"2" => Some(PegRoundDirection::MorePassiveOnABuyOrderRoundDownToNearestTickOnASellOrderRoundUpToNearestTick),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PegScope {
    Local = b'1' as isize,
    National = b'2' as isize,
    Global = b'3' as isize,
    NationalExcludingLocal = b'4' as isize,
}

impl FIXValue for PegScope {
    fn from_bytes(bytes: &[u8]) -> Option<PegScope> {
        match bytes {
            b"1" => Some(PegScope::Local),
            b"2" => Some(PegScope::National),
            b"3" => Some(PegScope::Global),
            b"4" => Some(PegScope::NationalExcludingLocal),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosAmtType {
    CashAmount,
    CashResidualAmount,
    FinalMarkToMarketAmount,
    IncrementalMarkToMarketAmount,
    PremiumAmount,
    StartOfDayMarkToMarketAmount,
    TradeVariationAmount,
    ValueAdjustedAmount,
}

impl FIXValue for PosAmtType {
    fn from_bytes(bytes: &[u8]) -> Option<PosAmtType> {
        match bytes {
            b"CASH" => Some(PosAmtType::CashAmount),
            b"CRES" => Some(PosAmtType::CashResidualAmount),
            b"FMTM" => Some(PosAmtType::FinalMarkToMarketAmount),
            b"IMTM" => Some(PosAmtType::IncrementalMarkToMarketAmount),
            b"PREM" => Some(PosAmtType::PremiumAmount),
            b"SMTM" => Some(PosAmtType::StartOfDayMarkToMarketAmount),
            b"TVAR" => Some(PosAmtType::TradeVariationAmount),
            b"VADJ" => Some(PosAmtType::ValueAdjustedAmount),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            PosAmtType::CashAmount => v.extend(b"CASH"),
            PosAmtType::CashResidualAmount => v.extend(b"CRES"),
            PosAmtType::FinalMarkToMarketAmount => v.extend(b"FMTM"),
            PosAmtType::IncrementalMarkToMarketAmount => v.extend(b"IMTM"),
            PosAmtType::PremiumAmount => v.extend(b"PREM"),
            PosAmtType::StartOfDayMarkToMarketAmount => v.extend(b"SMTM"),
            PosAmtType::TradeVariationAmount => v.extend(b"TVAR"),
            PosAmtType::ValueAdjustedAmount => v.extend(b"VADJ"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosMaintAction {
    NewUsedToIncrementTheOverallTransactionQuantity = b'1' as isize,
    ReplaceUsedToOverrideTheOverallTransactionQuantityOrSpecificAddMessagesBasedOnTheReferenceId = b'2' as isize,
    CancelUsedToRemoveTheOverallTransactionOrSpecificAddMessagesBasedOnReferenceId = b'3' as isize,
}

impl FIXValue for PosMaintAction {
    fn from_bytes(bytes: &[u8]) -> Option<PosMaintAction> {
        match bytes {
            b"1" => Some(PosMaintAction::NewUsedToIncrementTheOverallTransactionQuantity),
            b"2" => Some(PosMaintAction::ReplaceUsedToOverrideTheOverallTransactionQuantityOrSpecificAddMessagesBasedOnTheReferenceId),
            b"3" => Some(PosMaintAction::CancelUsedToRemoveTheOverallTransactionOrSpecificAddMessagesBasedOnReferenceId),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosMaintResult {
    SuccessfulCompletion = b'0' as isize,
    Rejected = b'1' as isize,
    Other,
}

impl FIXValue for PosMaintResult {
    fn from_bytes(bytes: &[u8]) -> Option<PosMaintResult> {
        match bytes {
            b"0" => Some(PosMaintResult::SuccessfulCompletion),
            b"1" => Some(PosMaintResult::Rejected),
            b"99" => Some(PosMaintResult::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            PosMaintResult::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosMaintStatus {
    Accepted = b'0' as isize,
    AcceptedWithWarnings = b'1' as isize,
    Rejected = b'2' as isize,
    Completed = b'3' as isize,
    CompletedWithWarnings = b'4' as isize,
}

impl FIXValue for PosMaintStatus {
    fn from_bytes(bytes: &[u8]) -> Option<PosMaintStatus> {
        match bytes {
            b"0" => Some(PosMaintStatus::Accepted),
            b"1" => Some(PosMaintStatus::AcceptedWithWarnings),
            b"2" => Some(PosMaintStatus::Rejected),
            b"3" => Some(PosMaintStatus::Completed),
            b"4" => Some(PosMaintStatus::CompletedWithWarnings),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosQtyStatus {
    Submitted = b'0' as isize,
    Accepted = b'1' as isize,
    Rejected = b'2' as isize,
}

impl FIXValue for PosQtyStatus {
    fn from_bytes(bytes: &[u8]) -> Option<PosQtyStatus> {
        match bytes {
            b"0" => Some(PosQtyStatus::Submitted),
            b"1" => Some(PosQtyStatus::Accepted),
            b"2" => Some(PosQtyStatus::Rejected),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosReqResult {
    ValidRequest = b'0' as isize,
    InvalidOrUnsupportedRequest = b'1' as isize,
    NoPositionsFoundThatMatchCriteria = b'2' as isize,
    NotAuthorizedToRequestPositions = b'3' as isize,
    RequestForPositionNotSupported = b'4' as isize,
    Other,
}

impl FIXValue for PosReqResult {
    fn from_bytes(bytes: &[u8]) -> Option<PosReqResult> {
        match bytes {
            b"0" => Some(PosReqResult::ValidRequest),
            b"1" => Some(PosReqResult::InvalidOrUnsupportedRequest),
            b"2" => Some(PosReqResult::NoPositionsFoundThatMatchCriteria),
            b"3" => Some(PosReqResult::NotAuthorizedToRequestPositions),
            b"4" => Some(PosReqResult::RequestForPositionNotSupported),
            b"99" => Some(PosReqResult::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            PosReqResult::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosReqStatus {
    Completed = b'0' as isize,
    CompletedWithWarnings = b'1' as isize,
    Rejected = b'2' as isize,
}

impl FIXValue for PosReqStatus {
    fn from_bytes(bytes: &[u8]) -> Option<PosReqStatus> {
        match bytes {
            b"0" => Some(PosReqStatus::Completed),
            b"1" => Some(PosReqStatus::CompletedWithWarnings),
            b"2" => Some(PosReqStatus::Rejected),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosReqType {
    Positions = b'0' as isize,
    Trades = b'1' as isize,
    Exercises = b'2' as isize,
    Assignments = b'3' as isize,
}

impl FIXValue for PosReqType {
    fn from_bytes(bytes: &[u8]) -> Option<PosReqType> {
        match bytes {
            b"0" => Some(PosReqType::Positions),
            b"1" => Some(PosReqType::Trades),
            b"2" => Some(PosReqType::Exercises),
            b"3" => Some(PosReqType::Assignments),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosTransType {
    Exercise = b'1' as isize,
    DoNotExercise = b'2' as isize,
    PositionAdjustment = b'3' as isize,
    PositionChangeSubmissionMarginDisposition = b'4' as isize,
    Pledge = b'5' as isize,
}

impl FIXValue for PosTransType {
    fn from_bytes(bytes: &[u8]) -> Option<PosTransType> {
        match bytes {
            b"1" => Some(PosTransType::Exercise),
            b"2" => Some(PosTransType::DoNotExercise),
            b"3" => Some(PosTransType::PositionAdjustment),
            b"4" => Some(PosTransType::PositionChangeSubmissionMarginDisposition),
            b"5" => Some(PosTransType::Pledge),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosType {
    AllocationTradeQty,
    OptionAssignment,
    AsOfTradeQty,
    DeliveryQty,
    ElectronicTradeQty,
    OptionExerciseQty,
    EndOfDayQty,
    IntraSpreadQty,
    InterSpreadQty,
    AdjustmentQty,
    PitTradeQty,
    StartOfDayQty,
    IntegralSplit,
    TransactionFromAssignment,
    TotalTransactionQty,
    TransactionQuantity,
    TransferTradeQty,
    TransactionFromExercise,
    CrossMarginQty,
}

impl FIXValue for PosType {
    fn from_bytes(bytes: &[u8]) -> Option<PosType> {
        match bytes {
            b"ALC" => Some(PosType::AllocationTradeQty),
            b"AS" => Some(PosType::OptionAssignment),
            b"ASF" => Some(PosType::AsOfTradeQty),
            b"DLV" => Some(PosType::DeliveryQty),
            b"ETR" => Some(PosType::ElectronicTradeQty),
            b"EX" => Some(PosType::OptionExerciseQty),
            b"FIN" => Some(PosType::EndOfDayQty),
            b"IAS" => Some(PosType::IntraSpreadQty),
            b"IES" => Some(PosType::InterSpreadQty),
            b"PA" => Some(PosType::AdjustmentQty),
            b"PIT" => Some(PosType::PitTradeQty),
            b"SOD" => Some(PosType::StartOfDayQty),
            b"SPL" => Some(PosType::IntegralSplit),
            b"TA" => Some(PosType::TransactionFromAssignment),
            b"TOT" => Some(PosType::TotalTransactionQty),
            b"TQ" => Some(PosType::TransactionQuantity),
            b"TRF" => Some(PosType::TransferTradeQty),
            b"TX" => Some(PosType::TransactionFromExercise),
            b"XM" => Some(PosType::CrossMarginQty),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            PosType::AllocationTradeQty => v.extend(b"ALC"),
            PosType::OptionAssignment => v.extend(b"AS"),
            PosType::AsOfTradeQty => v.extend(b"ASF"),
            PosType::DeliveryQty => v.extend(b"DLV"),
            PosType::ElectronicTradeQty => v.extend(b"ETR"),
            PosType::OptionExerciseQty => v.extend(b"EX"),
            PosType::EndOfDayQty => v.extend(b"FIN"),
            PosType::IntraSpreadQty => v.extend(b"IAS"),
            PosType::InterSpreadQty => v.extend(b"IES"),
            PosType::AdjustmentQty => v.extend(b"PA"),
            PosType::PitTradeQty => v.extend(b"PIT"),
            PosType::StartOfDayQty => v.extend(b"SOD"),
            PosType::IntegralSplit => v.extend(b"SPL"),
            PosType::TransactionFromAssignment => v.extend(b"TA"),
            PosType::TotalTransactionQty => v.extend(b"TOT"),
            PosType::TransactionQuantity => v.extend(b"TQ"),
            PosType::TransferTradeQty => v.extend(b"TRF"),
            PosType::TransactionFromExercise => v.extend(b"TX"),
            PosType::CrossMarginQty => v.extend(b"XM"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PossResend {
    Yes = b'Y' as isize,
    No = b'N' as isize,
}

impl FIXValue for PossResend {
    fn from_bytes(bytes: &[u8]) -> Option<PossResend> {
        match bytes {
            b"Y" => Some(PossResend::Yes),
            b"N" => Some(PossResend::No),
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
    PerUnit = b'2' as isize,
    FixedAmount = b'3' as isize,
    DiscountPercentagePointsBelowPar = b'4' as isize,
    PremiumPercentagePointsOverPar = b'5' as isize,
    Spread = b'6' as isize,
    TedPrice = b'7' as isize,
    TedYield = b'8' as isize,
    Yield = b'9' as isize,
    FixedCabinetTradePrice,
    VariableCabinetTradePrice,
}

impl FIXValue for PriceType {
    fn from_bytes(bytes: &[u8]) -> Option<PriceType> {
        match bytes {
            b"1" => Some(PriceType::Percentage),
            b"2" => Some(PriceType::PerUnit),
            b"3" => Some(PriceType::FixedAmount),
            b"4" => Some(PriceType::DiscountPercentagePointsBelowPar),
            b"5" => Some(PriceType::PremiumPercentagePointsOverPar),
            b"6" => Some(PriceType::Spread),
            b"7" => Some(PriceType::TedPrice),
            b"8" => Some(PriceType::TedYield),
            b"9" => Some(PriceType::Yield),
            b"10" => Some(PriceType::FixedCabinetTradePrice),
            b"11" => Some(PriceType::VariableCabinetTradePrice),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            PriceType::FixedCabinetTradePrice => v.extend(b"10"),
            PriceType::VariableCabinetTradePrice => v.extend(b"11"),
            _ => v.push(*self as u8)
        }
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
    Financing,
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
            b"13" => Some(Product::Financing),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            Product::Mortgage => v.extend(b"10"),
            Product::Municipal => v.extend(b"11"),
            Product::Other => v.extend(b"12"),
            Product::Financing => v.extend(b"13"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishTrdIndicator {
    Yes = b'Y' as isize,
    No = b'N' as isize,
}

impl FIXValue for PublishTrdIndicator {
    fn from_bytes(bytes: &[u8]) -> Option<PublishTrdIndicator> {
        match bytes {
            b"Y" => Some(PublishTrdIndicator::Yes),
            b"N" => Some(PublishTrdIndicator::No),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QtyType {
    Units = b'0' as isize,
    Contracts = b'1' as isize,
}

impl FIXValue for QtyType {
    fn from_bytes(bytes: &[u8]) -> Option<QtyType> {
        match bytes {
            b"0" => Some(QtyType::Units),
            b"1" => Some(QtyType::Contracts),
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
    CancelAllQuotes = b'4' as isize,
}

impl FIXValue for QuoteCancelType {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteCancelType> {
        match bytes {
            b"1" => Some(QuoteCancelType::CancelForSymbol),
            b"2" => Some(QuoteCancelType::CancelForSecurityType),
            b"3" => Some(QuoteCancelType::CancelForUnderlyingSymbol),
            b"4" => Some(QuoteCancelType::CancelAllQuotes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuotePriceType {
    Percent = b'1' as isize,
    PerShare = b'2' as isize,
    FixedAmount = b'3' as isize,
    DiscountPercentagePointsBelowPar = b'4' as isize,
    PremiumPercentagePointsOverPar = b'5' as isize,
    BasisPointsRelativeToBenchmark = b'6' as isize,
    TedPrice = b'7' as isize,
    TedYield = b'8' as isize,
    YieldSpread = b'9' as isize,
    Yield,
}

impl FIXValue for QuotePriceType {
    fn from_bytes(bytes: &[u8]) -> Option<QuotePriceType> {
        match bytes {
            b"1" => Some(QuotePriceType::Percent),
            b"2" => Some(QuotePriceType::PerShare),
            b"3" => Some(QuotePriceType::FixedAmount),
            b"4" => Some(QuotePriceType::DiscountPercentagePointsBelowPar),
            b"5" => Some(QuotePriceType::PremiumPercentagePointsOverPar),
            b"6" => Some(QuotePriceType::BasisPointsRelativeToBenchmark),
            b"7" => Some(QuotePriceType::TedPrice),
            b"8" => Some(QuotePriceType::TedYield),
            b"9" => Some(QuotePriceType::YieldSpread),
            b"10" => Some(QuotePriceType::Yield),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            QuotePriceType::Yield => v.extend(b"10"),
            _ => v.push(*self as u8)
        }
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
    Other,
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
            b"99" => Some(QuoteRejectReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            QuoteRejectReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
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
    NoMatchForInquiry = b'7' as isize,
    NoMarketForInstrument = b'8' as isize,
    NoInventory = b'9' as isize,
    Pass,
    Other,
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
            b"7" => Some(QuoteRequestRejectReason::NoMatchForInquiry),
            b"8" => Some(QuoteRequestRejectReason::NoMarketForInstrument),
            b"9" => Some(QuoteRequestRejectReason::NoInventory),
            b"10" => Some(QuoteRequestRejectReason::Pass),
            b"99" => Some(QuoteRequestRejectReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            QuoteRequestRejectReason::Pass => v.extend(b"10"),
            QuoteRequestRejectReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteRespType {
    HitLift = b'1' as isize,
    Counter = b'2' as isize,
    Expired = b'3' as isize,
    Cover = b'4' as isize,
    DoneAway = b'5' as isize,
    Pass = b'6' as isize,
}

impl FIXValue for QuoteRespType {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteRespType> {
        match bytes {
            b"1" => Some(QuoteRespType::HitLift),
            b"2" => Some(QuoteRespType::Counter),
            b"3" => Some(QuoteRespType::Expired),
            b"4" => Some(QuoteRespType::Cover),
            b"5" => Some(QuoteRespType::DoneAway),
            b"6" => Some(QuoteRespType::Pass),
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
    Pass,
    LockedMarketWarning,
    CrossMarketWarning,
    CanceledDueToLockMarket,
    CanceledDueToCrossMarket,
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
            b"11" => Some(QuoteStatus::Pass),
            b"12" => Some(QuoteStatus::LockedMarketWarning),
            b"13" => Some(QuoteStatus::CrossMarketWarning),
            b"14" => Some(QuoteStatus::CanceledDueToLockMarket),
            b"15" => Some(QuoteStatus::CanceledDueToCrossMarket),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            QuoteStatus::Pending => v.extend(b"10"),
            QuoteStatus::Pass => v.extend(b"11"),
            QuoteStatus::LockedMarketWarning => v.extend(b"12"),
            QuoteStatus::CrossMarketWarning => v.extend(b"13"),
            QuoteStatus::CanceledDueToLockMarket => v.extend(b"14"),
            QuoteStatus::CanceledDueToCrossMarket => v.extend(b"15"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteType {
    Indicative = b'0' as isize,
    Tradeable = b'1' as isize,
    RestrictedTradeable = b'2' as isize,
    Counter = b'3' as isize,
}

impl FIXValue for QuoteType {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteType> {
        match bytes {
            b"0" => Some(QuoteType::Indicative),
            b"1" => Some(QuoteType::Tradeable),
            b"2" => Some(QuoteType::RestrictedTradeable),
            b"3" => Some(QuoteType::Counter),
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
    Other,
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
            b"99" => Some(RegistRejReasonCode::Other),
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
            RegistRejReasonCode::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistStatus {
    Accepted = b'A' as isize,
    Rejected = b'R' as isize,
    Held = b'H' as isize,
    ReminderIeRegistrationInstructionsAreStillOutstanding = b'N' as isize,
}

impl FIXValue for RegistStatus {
    fn from_bytes(bytes: &[u8]) -> Option<RegistStatus> {
        match bytes {
            b"A" => Some(RegistStatus::Accepted),
            b"R" => Some(RegistStatus::Rejected),
            b"H" => Some(RegistStatus::Held),
            b"N" => Some(RegistStatus::ReminderIeRegistrationInstructionsAreStillOutstanding),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseTransportType {
    InbandTransportTheRequestWasSentOver = b'0' as isize,
    OutOfBandPreArrangedOutOfBandDeliveryMechanism = b'1' as isize,
}

impl FIXValue for ResponseTransportType {
    fn from_bytes(bytes: &[u8]) -> Option<ResponseTransportType> {
        match bytes {
            b"0" => Some(ResponseTransportType::InbandTransportTheRequestWasSentOver),
            b"1" => Some(ResponseTransportType::OutOfBandPreArrangedOutOfBandDeliveryMechanism),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityIDSource {
    Cusip = b'1' as isize,
    Sedol = b'2' as isize,
    Quik = b'3' as isize,
    IsinNumber = b'4' as isize,
    RicCode = b'5' as isize,
    IsoCurrencyCode = b'6' as isize,
    IsoCountryCode = b'7' as isize,
    ExchangeSymbol = b'8' as isize,
    ConsolidatedTapeAssociation = b'9' as isize,
    BloombergSymbol = b'A' as isize,
    Wertpapier = b'B' as isize,
    Dutch = b'C' as isize,
    Valoren = b'D' as isize,
    Sicovam = b'E' as isize,
    Belgian = b'F' as isize,
    Common = b'G' as isize,
    ClearingHouse = b'H' as isize,
    IsdaFpmlProductSpecification = b'I' as isize,
    OptionsPriceReportingAuthority = b'J' as isize,
}

impl FIXValue for SecurityIDSource {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityIDSource> {
        match bytes {
            b"1" => Some(SecurityIDSource::Cusip),
            b"2" => Some(SecurityIDSource::Sedol),
            b"3" => Some(SecurityIDSource::Quik),
            b"4" => Some(SecurityIDSource::IsinNumber),
            b"5" => Some(SecurityIDSource::RicCode),
            b"6" => Some(SecurityIDSource::IsoCurrencyCode),
            b"7" => Some(SecurityIDSource::IsoCountryCode),
            b"8" => Some(SecurityIDSource::ExchangeSymbol),
            b"9" => Some(SecurityIDSource::ConsolidatedTapeAssociation),
            b"A" => Some(SecurityIDSource::BloombergSymbol),
            b"B" => Some(SecurityIDSource::Wertpapier),
            b"C" => Some(SecurityIDSource::Dutch),
            b"D" => Some(SecurityIDSource::Valoren),
            b"E" => Some(SecurityIDSource::Sicovam),
            b"F" => Some(SecurityIDSource::Belgian),
            b"G" => Some(SecurityIDSource::Common),
            b"H" => Some(SecurityIDSource::ClearingHouse),
            b"I" => Some(SecurityIDSource::IsdaFpmlProductSpecification),
            b"J" => Some(SecurityIDSource::OptionsPriceReportingAuthority),
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
    RejectSecurityProposal = b'5' as isize,
    CanNotMatchSelectionCriteria = b'6' as isize,
}

impl FIXValue for SecurityResponseType {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityResponseType> {
        match bytes {
            b"1" => Some(SecurityResponseType::AcceptSecurityProposalAsIs),
            b"2" => Some(SecurityResponseType::AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage),
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
    AssetBackedSecurities,
    AmendedRestated,
    OtherAnticipationNotesBanGanEtc,
    BankersAcceptance,
    BankNotes,
    BillOfExchanges,
    BradyBond,
    BridgeLoan,
    BuySellback,
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
    EuroCertificateOfDeposit,
    EuroCorporateBond,
    EuroCommercialPaper,
    EuroSovereigns,
    EuroSupranationalCoupons,
    FederalAgencyCoupon,
    FederalAgencyDiscountNote,
    ForeignExchangeContract,
    Forward,
    Future,
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
    Option,
    PrivateExportFunding,
    Pfandbriefe,
    PromissoryNote,
    PreferredStock,
    PlazosFijos,
    RevenueAnticipationNote,
    Replaced,
    Repurchase,
    Retired,
    RevenueBonds,
    RevolverLoan,
    RevolverTermLoan,
    SecuritiesLoan,
    SecuritiesPledge,
    SpecialAssessment,
    SpecialObligation,
    SpecialTax,
    ShortTermLoanNote,
    StructuredNotes,
    UsdSupranationalCoupons,
    SwingLineFacility,
    TaxAnticipationNote,
    TaxAllocation,
    ToBeAnnounced,
    UsTreasuryBillTbill,
    UsTreasuryBond,
    PrincipalStripOfACallableBondOrNote,
    TimeDeposit,
    TaxExemptCommercialPaper,
    TermLoan,
    InterestStripFromAnyBondOrNote,
    TreasuryInflationProtectedSecurities,
    UsTreasuryNoteTnote,
    PrincipalStripFromANonCallableBondOrNote,
    TaxRevenueAnticipationNote,
    UsTreasuryNoteUst,
    UsTreasuryBillUstb,
    VariableRateDemandNote,
    Warrant,
    Withdrawn,
    ExtendedCommNote,
    IndexedLinked,
    YankeeCorporateBond,
    YankeeCertificateOfDeposit,
}

impl FIXValue for SecurityType {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityType> {
        match bytes {
            b"ABS" => Some(SecurityType::AssetBackedSecurities),
            b"AMENDED" => Some(SecurityType::AmendedRestated),
            b"AN" => Some(SecurityType::OtherAnticipationNotesBanGanEtc),
            b"BA" => Some(SecurityType::BankersAcceptance),
            b"BN" => Some(SecurityType::BankNotes),
            b"BOX" => Some(SecurityType::BillOfExchanges),
            b"BRADY" => Some(SecurityType::BradyBond),
            b"BRIDGE" => Some(SecurityType::BridgeLoan),
            b"BUYSELL" => Some(SecurityType::BuySellback),
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
            b"EUCD" => Some(SecurityType::EuroCertificateOfDeposit),
            b"EUCORP" => Some(SecurityType::EuroCorporateBond),
            b"EUCP" => Some(SecurityType::EuroCommercialPaper),
            b"EUSOV" => Some(SecurityType::EuroSovereigns),
            b"EUSUPRA" => Some(SecurityType::EuroSupranationalCoupons),
            b"FAC" => Some(SecurityType::FederalAgencyCoupon),
            b"FADN" => Some(SecurityType::FederalAgencyDiscountNote),
            b"FOR" => Some(SecurityType::ForeignExchangeContract),
            b"FORWARD" => Some(SecurityType::Forward),
            b"FUT" => Some(SecurityType::Future),
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
            b"OPT" => Some(SecurityType::Option),
            b"PEF" => Some(SecurityType::PrivateExportFunding),
            b"PFAND" => Some(SecurityType::Pfandbriefe),
            b"PN" => Some(SecurityType::PromissoryNote),
            b"PS" => Some(SecurityType::PreferredStock),
            b"PZFJ" => Some(SecurityType::PlazosFijos),
            b"RAN" => Some(SecurityType::RevenueAnticipationNote),
            b"REPLACD" => Some(SecurityType::Replaced),
            b"REPO" => Some(SecurityType::Repurchase),
            b"RETIRED" => Some(SecurityType::Retired),
            b"REV" => Some(SecurityType::RevenueBonds),
            b"RVLV" => Some(SecurityType::RevolverLoan),
            b"RVLVTRM" => Some(SecurityType::RevolverTermLoan),
            b"SECLOAN" => Some(SecurityType::SecuritiesLoan),
            b"SECPLEDGE" => Some(SecurityType::SecuritiesPledge),
            b"SPCLA" => Some(SecurityType::SpecialAssessment),
            b"SPCLO" => Some(SecurityType::SpecialObligation),
            b"SPCLT" => Some(SecurityType::SpecialTax),
            b"STN" => Some(SecurityType::ShortTermLoanNote),
            b"STRUCT" => Some(SecurityType::StructuredNotes),
            b"SUPRA" => Some(SecurityType::UsdSupranationalCoupons),
            b"SWING" => Some(SecurityType::SwingLineFacility),
            b"TAN" => Some(SecurityType::TaxAnticipationNote),
            b"TAXA" => Some(SecurityType::TaxAllocation),
            b"TBA" => Some(SecurityType::ToBeAnnounced),
            b"TBILL" => Some(SecurityType::UsTreasuryBillTbill),
            b"TBOND" => Some(SecurityType::UsTreasuryBond),
            b"TCAL" => Some(SecurityType::PrincipalStripOfACallableBondOrNote),
            b"TD" => Some(SecurityType::TimeDeposit),
            b"TECP" => Some(SecurityType::TaxExemptCommercialPaper),
            b"TERM" => Some(SecurityType::TermLoan),
            b"TINT" => Some(SecurityType::InterestStripFromAnyBondOrNote),
            b"TIPS" => Some(SecurityType::TreasuryInflationProtectedSecurities),
            b"TNOTE" => Some(SecurityType::UsTreasuryNoteTnote),
            b"TPRN" => Some(SecurityType::PrincipalStripFromANonCallableBondOrNote),
            b"TRAN" => Some(SecurityType::TaxRevenueAnticipationNote),
            b"UST" => Some(SecurityType::UsTreasuryNoteUst),
            b"USTB" => Some(SecurityType::UsTreasuryBillUstb),
            b"VRDN" => Some(SecurityType::VariableRateDemandNote),
            b"WAR" => Some(SecurityType::Warrant),
            b"WITHDRN" => Some(SecurityType::Withdrawn),
            b"XCN" => Some(SecurityType::ExtendedCommNote),
            b"XLINKD" => Some(SecurityType::IndexedLinked),
            b"YANK" => Some(SecurityType::YankeeCorporateBond),
            b"YCD" => Some(SecurityType::YankeeCertificateOfDeposit),
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
            SecurityType::BuySellback => v.extend(b"BUYSELL"),
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
            SecurityType::EuroCertificateOfDeposit => v.extend(b"EUCD"),
            SecurityType::EuroCorporateBond => v.extend(b"EUCORP"),
            SecurityType::EuroCommercialPaper => v.extend(b"EUCP"),
            SecurityType::EuroSovereigns => v.extend(b"EUSOV"),
            SecurityType::EuroSupranationalCoupons => v.extend(b"EUSUPRA"),
            SecurityType::FederalAgencyCoupon => v.extend(b"FAC"),
            SecurityType::FederalAgencyDiscountNote => v.extend(b"FADN"),
            SecurityType::ForeignExchangeContract => v.extend(b"FOR"),
            SecurityType::Forward => v.extend(b"FORWARD"),
            SecurityType::Future => v.extend(b"FUT"),
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
            SecurityType::Option => v.extend(b"OPT"),
            SecurityType::PrivateExportFunding => v.extend(b"PEF"),
            SecurityType::Pfandbriefe => v.extend(b"PFAND"),
            SecurityType::PromissoryNote => v.extend(b"PN"),
            SecurityType::PreferredStock => v.extend(b"PS"),
            SecurityType::PlazosFijos => v.extend(b"PZFJ"),
            SecurityType::RevenueAnticipationNote => v.extend(b"RAN"),
            SecurityType::Replaced => v.extend(b"REPLACD"),
            SecurityType::Repurchase => v.extend(b"REPO"),
            SecurityType::Retired => v.extend(b"RETIRED"),
            SecurityType::RevenueBonds => v.extend(b"REV"),
            SecurityType::RevolverLoan => v.extend(b"RVLV"),
            SecurityType::RevolverTermLoan => v.extend(b"RVLVTRM"),
            SecurityType::SecuritiesLoan => v.extend(b"SECLOAN"),
            SecurityType::SecuritiesPledge => v.extend(b"SECPLEDGE"),
            SecurityType::SpecialAssessment => v.extend(b"SPCLA"),
            SecurityType::SpecialObligation => v.extend(b"SPCLO"),
            SecurityType::SpecialTax => v.extend(b"SPCLT"),
            SecurityType::ShortTermLoanNote => v.extend(b"STN"),
            SecurityType::StructuredNotes => v.extend(b"STRUCT"),
            SecurityType::UsdSupranationalCoupons => v.extend(b"SUPRA"),
            SecurityType::SwingLineFacility => v.extend(b"SWING"),
            SecurityType::TaxAnticipationNote => v.extend(b"TAN"),
            SecurityType::TaxAllocation => v.extend(b"TAXA"),
            SecurityType::ToBeAnnounced => v.extend(b"TBA"),
            SecurityType::UsTreasuryBillTbill => v.extend(b"TBILL"),
            SecurityType::UsTreasuryBond => v.extend(b"TBOND"),
            SecurityType::PrincipalStripOfACallableBondOrNote => v.extend(b"TCAL"),
            SecurityType::TimeDeposit => v.extend(b"TD"),
            SecurityType::TaxExemptCommercialPaper => v.extend(b"TECP"),
            SecurityType::TermLoan => v.extend(b"TERM"),
            SecurityType::InterestStripFromAnyBondOrNote => v.extend(b"TINT"),
            SecurityType::TreasuryInflationProtectedSecurities => v.extend(b"TIPS"),
            SecurityType::UsTreasuryNoteTnote => v.extend(b"TNOTE"),
            SecurityType::PrincipalStripFromANonCallableBondOrNote => v.extend(b"TPRN"),
            SecurityType::TaxRevenueAnticipationNote => v.extend(b"TRAN"),
            SecurityType::UsTreasuryNoteUst => v.extend(b"UST"),
            SecurityType::UsTreasuryBillUstb => v.extend(b"USTB"),
            SecurityType::VariableRateDemandNote => v.extend(b"VRDN"),
            SecurityType::Warrant => v.extend(b"WAR"),
            SecurityType::Withdrawn => v.extend(b"WITHDRN"),
            SecurityType::ExtendedCommNote => v.extend(b"XCN"),
            SecurityType::IndexedLinked => v.extend(b"XLINKD"),
            SecurityType::YankeeCorporateBond => v.extend(b"YANK"),
            SecurityType::YankeeCertificateOfDeposit => v.extend(b"YCD"),
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
    Other,
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
            b"99" => Some(SessionRejectReason::Other),
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
            SessionRejectReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlDeliveryType {
    VersusPaymentDeliver = b'0' as isize,
    FreeDeliver = b'1' as isize,
    TriParty = b'2' as isize,
    HoldInCustody = b'3' as isize,
}

impl FIXValue for SettlDeliveryType {
    fn from_bytes(bytes: &[u8]) -> Option<SettlDeliveryType> {
        match bytes {
            b"0" => Some(SettlDeliveryType::VersusPaymentDeliver),
            b"1" => Some(SettlDeliveryType::FreeDeliver),
            b"2" => Some(SettlDeliveryType::TriParty),
            b"3" => Some(SettlDeliveryType::HoldInCustody),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstMode {
    StandingInstructionsProvided = b'1' as isize,
    SpecificOrderForASingleAccount = b'4' as isize,
    RequestReject = b'5' as isize,
}

impl FIXValue for SettlInstMode {
    fn from_bytes(bytes: &[u8]) -> Option<SettlInstMode> {
        match bytes {
            b"1" => Some(SettlInstMode::StandingInstructionsProvided),
            b"4" => Some(SettlInstMode::SpecificOrderForASingleAccount),
            b"5" => Some(SettlInstMode::RequestReject),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstReqRejCode {
    UnableToProcessRequest = b'0' as isize,
    UnknownAccount = b'1' as isize,
    NoMatchingSettlementInstructionsFound = b'2' as isize,
    Other,
}

impl FIXValue for SettlInstReqRejCode {
    fn from_bytes(bytes: &[u8]) -> Option<SettlInstReqRejCode> {
        match bytes {
            b"0" => Some(SettlInstReqRejCode::UnableToProcessRequest),
            b"1" => Some(SettlInstReqRejCode::UnknownAccount),
            b"2" => Some(SettlInstReqRejCode::NoMatchingSettlementInstructionsFound),
            b"99" => Some(SettlInstReqRejCode::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            SettlInstReqRejCode::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlInstSource {
    BrokersInstructions = b'1' as isize,
    InstitutionsInstructions = b'2' as isize,
    Investor = b'3' as isize,
}

impl FIXValue for SettlInstSource {
    fn from_bytes(bytes: &[u8]) -> Option<SettlInstSource> {
        match bytes {
            b"1" => Some(SettlInstSource::BrokersInstructions),
            b"2" => Some(SettlInstSource::InstitutionsInstructions),
            b"3" => Some(SettlInstSource::Investor),
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
    Cancel = b'C' as isize,
    Replace = b'R' as isize,
    Restate = b'T' as isize,
}

impl FIXValue for SettlInstTransType {
    fn from_bytes(bytes: &[u8]) -> Option<SettlInstTransType> {
        match bytes {
            b"N" => Some(SettlInstTransType::New),
            b"C" => Some(SettlInstTransType::Cancel),
            b"R" => Some(SettlInstTransType::Replace),
            b"T" => Some(SettlInstTransType::Restate),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlPriceType {
    Final = b'1' as isize,
    Theoretical = b'2' as isize,
}

impl FIXValue for SettlPriceType {
    fn from_bytes(bytes: &[u8]) -> Option<SettlPriceType> {
        match bytes {
            b"1" => Some(SettlPriceType::Final),
            b"2" => Some(SettlPriceType::Theoretical),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlSessID {
    ElectronicTradingHours,
    Intraday,
    RegularTradingHours,
}

impl FIXValue for SettlSessID {
    fn from_bytes(bytes: &[u8]) -> Option<SettlSessID> {
        match bytes {
            b"ETH" => Some(SettlSessID::ElectronicTradingHours),
            b"ITD" => Some(SettlSessID::Intraday),
            b"RTH" => Some(SettlSessID::RegularTradingHours),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            SettlSessID::ElectronicTradingHours => v.extend(b"ETH"),
            SettlSessID::Intraday => v.extend(b"ITD"),
            SettlSessID::RegularTradingHours => v.extend(b"RTH"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlType {
    Regular = b'0' as isize,
    Cash = b'1' as isize,
    NextDay = b'2' as isize,
    TPlus2 = b'3' as isize,
    TPlus3 = b'4' as isize,
    TPlus4 = b'5' as isize,
    Future = b'6' as isize,
    WhenAndIfIssued = b'7' as isize,
    SellersOption = b'8' as isize,
    TPlus5 = b'9' as isize,
}

impl FIXValue for SettlType {
    fn from_bytes(bytes: &[u8]) -> Option<SettlType> {
        match bytes {
            b"0" => Some(SettlType::Regular),
            b"1" => Some(SettlType::Cash),
            b"2" => Some(SettlType::NextDay),
            b"3" => Some(SettlType::TPlus2),
            b"4" => Some(SettlType::TPlus3),
            b"5" => Some(SettlType::TPlus4),
            b"6" => Some(SettlType::Future),
            b"7" => Some(SettlType::WhenAndIfIssued),
            b"8" => Some(SettlType::SellersOption),
            b"9" => Some(SettlType::TPlus5),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortSaleReason {
    DealerSoldShort = b'0' as isize,
    DealerSoldShortExempt = b'1' as isize,
    SellingCustomerSoldShort = b'2' as isize,
    SellingCustomerSoldShortExempt = b'3' as isize,
    QualifedServiceRepresentative = b'4' as isize,
    QsrOrAguContraSideSoldShortExempt = b'5' as isize,
}

impl FIXValue for ShortSaleReason {
    fn from_bytes(bytes: &[u8]) -> Option<ShortSaleReason> {
        match bytes {
            b"0" => Some(ShortSaleReason::DealerSoldShort),
            b"1" => Some(ShortSaleReason::DealerSoldShortExempt),
            b"2" => Some(ShortSaleReason::SellingCustomerSoldShort),
            b"3" => Some(ShortSaleReason::SellingCustomerSoldShortExempt),
            b"4" => Some(ShortSaleReason::QualifedServiceRepresentative),
            b"5" => Some(ShortSaleReason::QsrOrAguContraSideSoldShortExempt),
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
    CrossShortExempt = b'A' as isize,
    AsDefined = b'B' as isize,
    Opposite = b'C' as isize,
    Subscribe = b'D' as isize,
    Redeem = b'E' as isize,
    Lend = b'F' as isize,
    Borrow = b'G' as isize,
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
            b"A" => Some(Side::CrossShortExempt),
            b"B" => Some(Side::AsDefined),
            b"C" => Some(Side::Opposite),
            b"D" => Some(Side::Subscribe),
            b"E" => Some(Side::Redeem),
            b"F" => Some(Side::Lend),
            b"G" => Some(Side::Borrow),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideMultiLegReportingType {
    SingleSecurity = b'1' as isize,
    IndividualLegOfAMultiLegSecurity = b'2' as isize,
    MultiLegSecurity = b'3' as isize,
}

impl FIXValue for SideMultiLegReportingType {
    fn from_bytes(bytes: &[u8]) -> Option<SideMultiLegReportingType> {
        match bytes {
            b"1" => Some(SideMultiLegReportingType::SingleSecurity),
            b"2" => Some(SideMultiLegReportingType::IndividualLegOfAMultiLegSecurity),
            b"3" => Some(SideMultiLegReportingType::MultiLegSecurity),
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
    ThomsonAlert = b'2' as isize,
    AGlobalCustodian = b'3' as isize,
    Accountnet = b'4' as isize,
}

impl FIXValue for StandInstDbType {
    fn from_bytes(bytes: &[u8]) -> Option<StandInstDbType> {
        match bytes {
            b"0" => Some(StandInstDbType::Other),
            b"1" => Some(StandInstDbType::DtcSid),
            b"2" => Some(StandInstDbType::ThomsonAlert),
            b"3" => Some(StandInstDbType::AGlobalCustodian),
            b"4" => Some(StandInstDbType::Accountnet),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusValue {
    Connected = b'1' as isize,
    NotConnectedDownExpectedUp = b'2' as isize,
    NotConnectedDownExpectedDown = b'3' as isize,
    InProcess = b'4' as isize,
}

impl FIXValue for StatusValue {
    fn from_bytes(bytes: &[u8]) -> Option<StatusValue> {
        match bytes {
            b"1" => Some(StatusValue::Connected),
            b"2" => Some(StatusValue::NotConnectedDownExpectedUp),
            b"3" => Some(StatusValue::NotConnectedDownExpectedDown),
            b"4" => Some(StatusValue::InProcess),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StipulationType {
    Amt,
    AutoReinvestmentAtRateOrBetter,
    BankQualified,
    BargainConditionsSee,
    CouponRange,
    IsoCurrencyCode,
    CustomStartEndDate,
    GeographicsAndRange,
    ValuationDiscount,
    Insured,
    YearOrYearMonthOfIssue,
    IssuersTicker,
    IssueSizeRange,
    LookbackDays,
    ExplicitLotIdentifier,
    LotVariance,
    MaturityYearAndMonth,
    MaturityRange,
    MaximumSubstitutions,
    MinimumDenomination,
    MinimumIncrement,
    MinimumQuantity,
    PaymentFrequencyCalendar,
    NumberOfPieces,
    PoolsMaximum,
    PoolsPerLot,
    PoolsPerMillion,
    PoolsPerTrade,
    PriceRange,
    PricingFrequency,
    ProductionYear,
    CallProtection,
    Purpose,
    BenchmarkPriceSource,
    RatingSourceAndRange,
    TypeOfRedemptionValuesAreNoncallableCallablePrefundedEscrowedtomaturityPutableConvertible,
    Restricted,
    MarketSector,
    SecuritytypeIncludedOrExcluded,
    Structure,
    SubstitutionsFrequency,
    SubstitutionsLeft,
    FreeformText,
    TradeVariance,
    WeightedAverageCouponvalueInPercent,
    WeightedAverageLifeCouponValueInPercent,
    WeightedAverageLoanAgeValueInMonths,
    WeightedAverageMaturityValueInMonths,
    WholePool,
    YieldRange,
}

impl FIXValue for StipulationType {
    fn from_bytes(bytes: &[u8]) -> Option<StipulationType> {
        match bytes {
            b"AMT" => Some(StipulationType::Amt),
            b"AUTOREINV" => Some(StipulationType::AutoReinvestmentAtRateOrBetter),
            b"BANKQUAL" => Some(StipulationType::BankQualified),
            b"BGNCON" => Some(StipulationType::BargainConditionsSee),
            b"COUPON" => Some(StipulationType::CouponRange),
            b"CURRENCY" => Some(StipulationType::IsoCurrencyCode),
            b"CUSTOMDATE" => Some(StipulationType::CustomStartEndDate),
            b"GEOG" => Some(StipulationType::GeographicsAndRange),
            b"HAIRCUT" => Some(StipulationType::ValuationDiscount),
            b"INSURED" => Some(StipulationType::Insured),
            b"ISSUE" => Some(StipulationType::YearOrYearMonthOfIssue),
            b"ISSUER" => Some(StipulationType::IssuersTicker),
            b"ISSUESIZE" => Some(StipulationType::IssueSizeRange),
            b"LOOKBACK" => Some(StipulationType::LookbackDays),
            b"LOT" => Some(StipulationType::ExplicitLotIdentifier),
            b"LOTVAR" => Some(StipulationType::LotVariance),
            b"MAT" => Some(StipulationType::MaturityYearAndMonth),
            b"MATURITY" => Some(StipulationType::MaturityRange),
            b"MAXSUBS" => Some(StipulationType::MaximumSubstitutions),
            b"MINDNOM" => Some(StipulationType::MinimumDenomination),
            b"MININCR" => Some(StipulationType::MinimumIncrement),
            b"MINQTY" => Some(StipulationType::MinimumQuantity),
            b"PAYFREQ" => Some(StipulationType::PaymentFrequencyCalendar),
            b"PIECES" => Some(StipulationType::NumberOfPieces),
            b"PMAX" => Some(StipulationType::PoolsMaximum),
            b"PPL" => Some(StipulationType::PoolsPerLot),
            b"PPM" => Some(StipulationType::PoolsPerMillion),
            b"PPT" => Some(StipulationType::PoolsPerTrade),
            b"PRICE" => Some(StipulationType::PriceRange),
            b"PRICEFREQ" => Some(StipulationType::PricingFrequency),
            b"PROD" => Some(StipulationType::ProductionYear),
            b"PROTECT" => Some(StipulationType::CallProtection),
            b"PURPOSE" => Some(StipulationType::Purpose),
            b"PXSOURCE" => Some(StipulationType::BenchmarkPriceSource),
            b"RATING" => Some(StipulationType::RatingSourceAndRange),
            b"REDEMPTION" => Some(StipulationType::TypeOfRedemptionValuesAreNoncallableCallablePrefundedEscrowedtomaturityPutableConvertible),
            b"RESTRICTED" => Some(StipulationType::Restricted),
            b"SECTOR" => Some(StipulationType::MarketSector),
            b"SECTYPE" => Some(StipulationType::SecuritytypeIncludedOrExcluded),
            b"STRUCT" => Some(StipulationType::Structure),
            b"SUBSFREQ" => Some(StipulationType::SubstitutionsFrequency),
            b"SUBSLEFT" => Some(StipulationType::SubstitutionsLeft),
            b"TEXT" => Some(StipulationType::FreeformText),
            b"TRDVAR" => Some(StipulationType::TradeVariance),
            b"WAC" => Some(StipulationType::WeightedAverageCouponvalueInPercent),
            b"WAL" => Some(StipulationType::WeightedAverageLifeCouponValueInPercent),
            b"WALA" => Some(StipulationType::WeightedAverageLoanAgeValueInMonths),
            b"WAM" => Some(StipulationType::WeightedAverageMaturityValueInMonths),
            b"WHOLE" => Some(StipulationType::WholePool),
            b"YIELD" => Some(StipulationType::YieldRange),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            StipulationType::Amt => v.extend(b"AMT"),
            StipulationType::AutoReinvestmentAtRateOrBetter => v.extend(b"AUTOREINV"),
            StipulationType::BankQualified => v.extend(b"BANKQUAL"),
            StipulationType::BargainConditionsSee => v.extend(b"BGNCON"),
            StipulationType::CouponRange => v.extend(b"COUPON"),
            StipulationType::IsoCurrencyCode => v.extend(b"CURRENCY"),
            StipulationType::CustomStartEndDate => v.extend(b"CUSTOMDATE"),
            StipulationType::GeographicsAndRange => v.extend(b"GEOG"),
            StipulationType::ValuationDiscount => v.extend(b"HAIRCUT"),
            StipulationType::Insured => v.extend(b"INSURED"),
            StipulationType::YearOrYearMonthOfIssue => v.extend(b"ISSUE"),
            StipulationType::IssuersTicker => v.extend(b"ISSUER"),
            StipulationType::IssueSizeRange => v.extend(b"ISSUESIZE"),
            StipulationType::LookbackDays => v.extend(b"LOOKBACK"),
            StipulationType::ExplicitLotIdentifier => v.extend(b"LOT"),
            StipulationType::LotVariance => v.extend(b"LOTVAR"),
            StipulationType::MaturityYearAndMonth => v.extend(b"MAT"),
            StipulationType::MaturityRange => v.extend(b"MATURITY"),
            StipulationType::MaximumSubstitutions => v.extend(b"MAXSUBS"),
            StipulationType::MinimumDenomination => v.extend(b"MINDNOM"),
            StipulationType::MinimumIncrement => v.extend(b"MININCR"),
            StipulationType::MinimumQuantity => v.extend(b"MINQTY"),
            StipulationType::PaymentFrequencyCalendar => v.extend(b"PAYFREQ"),
            StipulationType::NumberOfPieces => v.extend(b"PIECES"),
            StipulationType::PoolsMaximum => v.extend(b"PMAX"),
            StipulationType::PoolsPerLot => v.extend(b"PPL"),
            StipulationType::PoolsPerMillion => v.extend(b"PPM"),
            StipulationType::PoolsPerTrade => v.extend(b"PPT"),
            StipulationType::PriceRange => v.extend(b"PRICE"),
            StipulationType::PricingFrequency => v.extend(b"PRICEFREQ"),
            StipulationType::ProductionYear => v.extend(b"PROD"),
            StipulationType::CallProtection => v.extend(b"PROTECT"),
            StipulationType::Purpose => v.extend(b"PURPOSE"),
            StipulationType::BenchmarkPriceSource => v.extend(b"PXSOURCE"),
            StipulationType::RatingSourceAndRange => v.extend(b"RATING"),
            StipulationType::TypeOfRedemptionValuesAreNoncallableCallablePrefundedEscrowedtomaturityPutableConvertible => v.extend(b"REDEMPTION"),
            StipulationType::Restricted => v.extend(b"RESTRICTED"),
            StipulationType::MarketSector => v.extend(b"SECTOR"),
            StipulationType::SecuritytypeIncludedOrExcluded => v.extend(b"SECTYPE"),
            StipulationType::Structure => v.extend(b"STRUCT"),
            StipulationType::SubstitutionsFrequency => v.extend(b"SUBSFREQ"),
            StipulationType::SubstitutionsLeft => v.extend(b"SUBSLEFT"),
            StipulationType::FreeformText => v.extend(b"TEXT"),
            StipulationType::TradeVariance => v.extend(b"TRDVAR"),
            StipulationType::WeightedAverageCouponvalueInPercent => v.extend(b"WAC"),
            StipulationType::WeightedAverageLifeCouponValueInPercent => v.extend(b"WAL"),
            StipulationType::WeightedAverageLoanAgeValueInMonths => v.extend(b"WALA"),
            StipulationType::WeightedAverageMaturityValueInMonths => v.extend(b"WAM"),
            StipulationType::WholePool => v.extend(b"WHOLE"),
            StipulationType::YieldRange => v.extend(b"YIELD"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetStrategy {
    Vwap = b'1' as isize,
    Participate = b'2' as isize,
    MininizeMarketImpact = b'3' as isize,
}

impl FIXValue for TargetStrategy {
    fn from_bytes(bytes: &[u8]) -> Option<TargetStrategy> {
        match bytes {
            b"1" => Some(TargetStrategy::Vwap),
            b"2" => Some(TargetStrategy::Participate),
            b"3" => Some(TargetStrategy::MininizeMarketImpact),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminationType {
    Overnight = b'1' as isize,
    Term = b'2' as isize,
    Flexible = b'3' as isize,
    Open = b'4' as isize,
}

impl FIXValue for TerminationType {
    fn from_bytes(bytes: &[u8]) -> Option<TerminationType> {
        match bytes {
            b"1" => Some(TerminationType::Overnight),
            b"2" => Some(TerminationType::Term),
            b"3" => Some(TerminationType::Flexible),
            b"4" => Some(TerminationType::Open),
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
    AtTheClose = b'7' as isize,
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
            b"7" => Some(TimeInForce::AtTheClose),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradSesStatus {
    Unknown = b'0' as isize,
    Halted = b'1' as isize,
    Open = b'2' as isize,
    Closed = b'3' as isize,
    PreOpen = b'4' as isize,
    PreClose = b'5' as isize,
    RequestRejected = b'6' as isize,
}

impl FIXValue for TradSesStatus {
    fn from_bytes(bytes: &[u8]) -> Option<TradSesStatus> {
        match bytes {
            b"0" => Some(TradSesStatus::Unknown),
            b"1" => Some(TradSesStatus::Halted),
            b"2" => Some(TradSesStatus::Open),
            b"3" => Some(TradSesStatus::Closed),
            b"4" => Some(TradSesStatus::PreOpen),
            b"5" => Some(TradSesStatus::PreClose),
            b"6" => Some(TradSesStatus::RequestRejected),
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
    Other,
}

impl FIXValue for TradSesStatusRejReason {
    fn from_bytes(bytes: &[u8]) -> Option<TradSesStatusRejReason> {
        match bytes {
            b"1" => Some(TradSesStatusRejReason::UnknownOrInvalidTradingsessionid),
            b"99" => Some(TradSesStatusRejReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            TradSesStatusRejReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeAllocIndicator {
    AllocationNotRequired = b'0' as isize,
    AllocationRequired = b'1' as isize,
    UseAllocationProvidedWithTheTrade = b'2' as isize,
}

impl FIXValue for TradeAllocIndicator {
    fn from_bytes(bytes: &[u8]) -> Option<TradeAllocIndicator> {
        match bytes {
            b"0" => Some(TradeAllocIndicator::AllocationNotRequired),
            b"1" => Some(TradeAllocIndicator::AllocationRequired),
            b"2" => Some(TradeAllocIndicator::UseAllocationProvidedWithTheTrade),
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
    ImbalanceMoreBuyers = b'P' as isize,
    ImbalanceMoreSellers = b'Q' as isize,
    OpeningPrice = b'R' as isize,
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
            b"P" => Some(TradeCondition::ImbalanceMoreBuyers),
            b"Q" => Some(TradeCondition::ImbalanceMoreSellers),
            b"R" => Some(TradeCondition::OpeningPrice),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeReportRejectReason {
    Successful = b'0' as isize,
    InvalidPartyInformation = b'1' as isize,
    UnknownInstrument = b'2' as isize,
    UnauthorizedToReportTrades = b'3' as isize,
    InvalidTradeType = b'4' as isize,
    Other,
}

impl FIXValue for TradeReportRejectReason {
    fn from_bytes(bytes: &[u8]) -> Option<TradeReportRejectReason> {
        match bytes {
            b"0" => Some(TradeReportRejectReason::Successful),
            b"1" => Some(TradeReportRejectReason::InvalidPartyInformation),
            b"2" => Some(TradeReportRejectReason::UnknownInstrument),
            b"3" => Some(TradeReportRejectReason::UnauthorizedToReportTrades),
            b"4" => Some(TradeReportRejectReason::InvalidTradeType),
            b"99" => Some(TradeReportRejectReason::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            TradeReportRejectReason::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeReportType {
    Submit = b'0' as isize,
    Alleged = b'1' as isize,
    Accept = b'2' as isize,
    Decline = b'3' as isize,
    Addendum = b'4' as isize,
    NoWas = b'5' as isize,
    TradeReportCancel = b'6' as isize,
    LockedInTradeBreak = b'7' as isize,
}

impl FIXValue for TradeReportType {
    fn from_bytes(bytes: &[u8]) -> Option<TradeReportType> {
        match bytes {
            b"0" => Some(TradeReportType::Submit),
            b"1" => Some(TradeReportType::Alleged),
            b"2" => Some(TradeReportType::Accept),
            b"3" => Some(TradeReportType::Decline),
            b"4" => Some(TradeReportType::Addendum),
            b"5" => Some(TradeReportType::NoWas),
            b"6" => Some(TradeReportType::TradeReportCancel),
            b"7" => Some(TradeReportType::LockedInTradeBreak),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeRequestResult {
    Successful = b'0' as isize,
    InvalidOrUnknownInstrument = b'1' as isize,
    InvalidTypeOfTradeRequested = b'2' as isize,
    InvalidParties = b'3' as isize,
    InvalidTransportTypeRequested = b'4' as isize,
    InvalidDestinationRequested = b'5' as isize,
    TraderequesttypeNotSupported = b'8' as isize,
    UnauthorizedForTradeCaptureReportRequest = b'9' as isize,
    Other,
}

impl FIXValue for TradeRequestResult {
    fn from_bytes(bytes: &[u8]) -> Option<TradeRequestResult> {
        match bytes {
            b"0" => Some(TradeRequestResult::Successful),
            b"1" => Some(TradeRequestResult::InvalidOrUnknownInstrument),
            b"2" => Some(TradeRequestResult::InvalidTypeOfTradeRequested),
            b"3" => Some(TradeRequestResult::InvalidParties),
            b"4" => Some(TradeRequestResult::InvalidTransportTypeRequested),
            b"5" => Some(TradeRequestResult::InvalidDestinationRequested),
            b"8" => Some(TradeRequestResult::TraderequesttypeNotSupported),
            b"9" => Some(TradeRequestResult::UnauthorizedForTradeCaptureReportRequest),
            b"99" => Some(TradeRequestResult::Other),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            TradeRequestResult::Other => v.extend(b"99"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeRequestStatus {
    Accepted = b'0' as isize,
    Completed = b'1' as isize,
    Rejected = b'2' as isize,
}

impl FIXValue for TradeRequestStatus {
    fn from_bytes(bytes: &[u8]) -> Option<TradeRequestStatus> {
        match bytes {
            b"0" => Some(TradeRequestStatus::Accepted),
            b"1" => Some(TradeRequestStatus::Completed),
            b"2" => Some(TradeRequestStatus::Rejected),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrdRegTimestampType {
    ExecutionTime = b'1' as isize,
    TimeIn = b'2' as isize,
    TimeOut = b'3' as isize,
    BrokerReceipt = b'4' as isize,
    BrokerExecution = b'5' as isize,
}

impl FIXValue for TrdRegTimestampType {
    fn from_bytes(bytes: &[u8]) -> Option<TrdRegTimestampType> {
        match bytes {
            b"1" => Some(TrdRegTimestampType::ExecutionTime),
            b"2" => Some(TrdRegTimestampType::TimeIn),
            b"3" => Some(TrdRegTimestampType::TimeOut),
            b"4" => Some(TrdRegTimestampType::BrokerReceipt),
            b"5" => Some(TrdRegTimestampType::BrokerExecution),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrdRptStatus {
    Accepted = b'0' as isize,
    Rejected = b'1' as isize,
}

impl FIXValue for TrdRptStatus {
    fn from_bytes(bytes: &[u8]) -> Option<TrdRptStatus> {
        match bytes {
            b"0" => Some(TrdRptStatus::Accepted),
            b"1" => Some(TrdRptStatus::Rejected),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrdType {
    RegularTrade = b'0' as isize,
    BlockTrade = b'1' as isize,
    Efp = b'2' as isize,
    Transfer = b'3' as isize,
    LateTrade = b'4' as isize,
    TTrade = b'5' as isize,
    WeightedAveragePriceTrade = b'6' as isize,
    BunchedTrade = b'7' as isize,
    LateBunchedTrade = b'8' as isize,
    PriorReferencePriceTrade = b'9' as isize,
    AfterHoursTrade,
}

impl FIXValue for TrdType {
    fn from_bytes(bytes: &[u8]) -> Option<TrdType> {
        match bytes {
            b"0" => Some(TrdType::RegularTrade),
            b"1" => Some(TrdType::BlockTrade),
            b"2" => Some(TrdType::Efp),
            b"3" => Some(TrdType::Transfer),
            b"4" => Some(TrdType::LateTrade),
            b"5" => Some(TrdType::TTrade),
            b"6" => Some(TrdType::WeightedAveragePriceTrade),
            b"7" => Some(TrdType::BunchedTrade),
            b"8" => Some(TrdType::LateBunchedTrade),
            b"9" => Some(TrdType::PriorReferencePriceTrade),
            b"10" => Some(TrdType::AfterHoursTrade),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            TrdType::AfterHoursTrade => v.extend(b"10"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRequestType {
    Logonuser = b'1' as isize,
    Logoffuser = b'2' as isize,
    Changepasswordforuser = b'3' as isize,
    RequestIndividualUserStatus = b'4' as isize,
}

impl FIXValue for UserRequestType {
    fn from_bytes(bytes: &[u8]) -> Option<UserRequestType> {
        match bytes {
            b"1" => Some(UserRequestType::Logonuser),
            b"2" => Some(UserRequestType::Logoffuser),
            b"3" => Some(UserRequestType::Changepasswordforuser),
            b"4" => Some(UserRequestType::RequestIndividualUserStatus),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    LoggedIn = b'1' as isize,
    NotLoggedIn = b'2' as isize,
    UserNotRecognised = b'3' as isize,
    PasswordIncorrect = b'4' as isize,
    PasswordChanged = b'5' as isize,
    Other = b'6' as isize,
}

impl FIXValue for UserStatus {
    fn from_bytes(bytes: &[u8]) -> Option<UserStatus> {
        match bytes {
            b"1" => Some(UserStatus::LoggedIn),
            b"2" => Some(UserStatus::NotLoggedIn),
            b"3" => Some(UserStatus::UserNotRecognised),
            b"4" => Some(UserStatus::PasswordIncorrect),
            b"5" => Some(UserStatus::PasswordChanged),
            b"6" => Some(UserStatus::Other),
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
    AnnualYield,
    YieldAtIssue,
    YieldToAverageMaturity,
    BookYield,
    YieldToNextCall,
    YieldChangeSinceClose,
    ClosingYield,
    CompoundYield,
    CurrentYield,
    GovernmentEquivalentYield,
    TrueGrossYield,
    YieldWithInflationAssumption,
    InverseFloaterBondYield,
    MostRecentClosingYield,
    ClosingYieldMostRecentMonth,
    ClosingYieldMostRecentQuarter,
    ClosingYieldMostRecentYear,
    YieldToLongestAverageLife,
    MarkToMarketYield,
    YieldToMaturity,
    YieldToNextRefund,
    OpenAverageYield,
    PreviousCloseYield,
    ProceedsYield,
    YieldToNextPut,
    SemiAnnualYield,
    YieldToShortestAverageLife,
    SimpleYield,
    TaxEquivalentYield,
    YieldToTenderDate,
    TrueYield,
    YieldValueOf132,
    YieldToWorst,
}

impl FIXValue for YieldType {
    fn from_bytes(bytes: &[u8]) -> Option<YieldType> {
        match bytes {
            b"AFTERTAX" => Some(YieldType::AfterTaxYield),
            b"ANNUAL" => Some(YieldType::AnnualYield),
            b"ATISSUE" => Some(YieldType::YieldAtIssue),
            b"AVGMATURITY" => Some(YieldType::YieldToAverageMaturity),
            b"BOOK" => Some(YieldType::BookYield),
            b"CALL" => Some(YieldType::YieldToNextCall),
            b"CHANGE" => Some(YieldType::YieldChangeSinceClose),
            b"CLOSE" => Some(YieldType::ClosingYield),
            b"COMPOUND" => Some(YieldType::CompoundYield),
            b"CURRENT" => Some(YieldType::CurrentYield),
            b"GOVTEQUIV" => Some(YieldType::GovernmentEquivalentYield),
            b"GROSS" => Some(YieldType::TrueGrossYield),
            b"INFLATION" => Some(YieldType::YieldWithInflationAssumption),
            b"INVERSEFLOATER" => Some(YieldType::InverseFloaterBondYield),
            b"LASTCLOSE" => Some(YieldType::MostRecentClosingYield),
            b"LASTMONTH" => Some(YieldType::ClosingYieldMostRecentMonth),
            b"LASTQUARTER" => Some(YieldType::ClosingYieldMostRecentQuarter),
            b"LASTYEAR" => Some(YieldType::ClosingYieldMostRecentYear),
            b"LONGAVGLIFE" => Some(YieldType::YieldToLongestAverageLife),
            b"MARK" => Some(YieldType::MarkToMarketYield),
            b"MATURITY" => Some(YieldType::YieldToMaturity),
            b"NEXTREFUND" => Some(YieldType::YieldToNextRefund),
            b"OPENAVG" => Some(YieldType::OpenAverageYield),
            b"PREVCLOSE" => Some(YieldType::PreviousCloseYield),
            b"PROCEEDS" => Some(YieldType::ProceedsYield),
            b"PUT" => Some(YieldType::YieldToNextPut),
            b"SEMIANNUAL" => Some(YieldType::SemiAnnualYield),
            b"SHORTAVGLIFE" => Some(YieldType::YieldToShortestAverageLife),
            b"SIMPLE" => Some(YieldType::SimpleYield),
            b"TAXEQUIV" => Some(YieldType::TaxEquivalentYield),
            b"TENDER" => Some(YieldType::YieldToTenderDate),
            b"TRUE" => Some(YieldType::TrueYield),
            b"VALUE1/32" => Some(YieldType::YieldValueOf132),
            b"WORST" => Some(YieldType::YieldToWorst),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            YieldType::AfterTaxYield => v.extend(b"AFTERTAX"),
            YieldType::AnnualYield => v.extend(b"ANNUAL"),
            YieldType::YieldAtIssue => v.extend(b"ATISSUE"),
            YieldType::YieldToAverageMaturity => v.extend(b"AVGMATURITY"),
            YieldType::BookYield => v.extend(b"BOOK"),
            YieldType::YieldToNextCall => v.extend(b"CALL"),
            YieldType::YieldChangeSinceClose => v.extend(b"CHANGE"),
            YieldType::ClosingYield => v.extend(b"CLOSE"),
            YieldType::CompoundYield => v.extend(b"COMPOUND"),
            YieldType::CurrentYield => v.extend(b"CURRENT"),
            YieldType::GovernmentEquivalentYield => v.extend(b"GOVTEQUIV"),
            YieldType::TrueGrossYield => v.extend(b"GROSS"),
            YieldType::YieldWithInflationAssumption => v.extend(b"INFLATION"),
            YieldType::InverseFloaterBondYield => v.extend(b"INVERSEFLOATER"),
            YieldType::MostRecentClosingYield => v.extend(b"LASTCLOSE"),
            YieldType::ClosingYieldMostRecentMonth => v.extend(b"LASTMONTH"),
            YieldType::ClosingYieldMostRecentQuarter => v.extend(b"LASTQUARTER"),
            YieldType::ClosingYieldMostRecentYear => v.extend(b"LASTYEAR"),
            YieldType::YieldToLongestAverageLife => v.extend(b"LONGAVGLIFE"),
            YieldType::MarkToMarketYield => v.extend(b"MARK"),
            YieldType::YieldToMaturity => v.extend(b"MATURITY"),
            YieldType::YieldToNextRefund => v.extend(b"NEXTREFUND"),
            YieldType::OpenAverageYield => v.extend(b"OPENAVG"),
            YieldType::PreviousCloseYield => v.extend(b"PREVCLOSE"),
            YieldType::ProceedsYield => v.extend(b"PROCEEDS"),
            YieldType::YieldToNextPut => v.extend(b"PUT"),
            YieldType::SemiAnnualYield => v.extend(b"SEMIANNUAL"),
            YieldType::YieldToShortestAverageLife => v.extend(b"SHORTAVGLIFE"),
            YieldType::SimpleYield => v.extend(b"SIMPLE"),
            YieldType::TaxEquivalentYield => v.extend(b"TAXEQUIV"),
            YieldType::YieldToTenderDate => v.extend(b"TENDER"),
            YieldType::TrueYield => v.extend(b"TRUE"),
            YieldType::YieldValueOf132 => v.extend(b"VALUE1/32"),
            YieldType::YieldToWorst => v.extend(b"WORST"),
        }
    }
}

pub struct AffectedOrdGrp {
    pub orig_cl_ord_id: Option<FIXString>,
    pub affected_order_id: Option<FIXString>,
    pub affected_secondary_order_id: Option<FIXString>
}

pub struct AllocAckGrp {
    pub alloc_account: Option<FIXString>,
    pub alloc_acct_id_source: Option<FIXInt>,
    pub alloc_price: Option<Price>,
    pub individual_alloc_id: Option<FIXString>,
    pub individual_alloc_rej_code: Option<FIXInt>,
    pub alloc_text: Option<FIXString>,
    pub encoded_alloc_text: Option<Data>
}

pub struct AllocGrp {
    pub alloc_account: Option<FIXString>,
    pub alloc_acct_id_source: Option<FIXInt>,
    pub match_status: Option<MatchStatus>,
    pub alloc_price: Option<Price>,
    pub alloc_qty: Option<Qty>,
    pub individual_alloc_id: Option<FIXString>,
    pub process_code: Option<ProcessCode>,
    pub nested_parties: Option<NestedParties>,
    pub notify_broker_of_credit: Option<NotifyBrokerOfCredit>,
    pub alloc_handl_inst: Option<AllocHandlInst>,
    pub alloc_text: Option<FIXString>,
    pub encoded_alloc_text: Option<Data>,
    pub commission_data: Option<CommissionData>,
    pub alloc_avg_px: Option<Price>,
    pub alloc_net_money: Option<Amt>,
    pub settl_curr_amt: Option<Amt>,
    pub alloc_settl_curr_amt: Option<Amt>,
    pub settl_currency: Option<Currency>,
    pub alloc_settl_currency: Option<Currency>,
    pub settl_curr_fx_rate: Option<FIXFloat>,
    pub settl_curr_fx_rate_calc: Option<SettlCurrFxRateCalc>,
    pub alloc_accrued_interest_amt: Option<Amt>,
    pub alloc_interest_at_maturity: Option<Amt>,
    pub misc_fees_grp: Option<MiscFeesGrp>,
    pub clr_inst_grp: Option<ClrInstGrp>,
    pub alloc_settl_inst_type: Option<AllocSettlInstType>,
    pub settl_instructions_data: Option<SettlInstructionsData>
}

pub struct AttrbGrp {
    pub instr_attrib_type: Option<InstrAttribType>,
    pub instr_attrib_value: Option<FIXString>
}

pub struct BidCompReqGrp {
    pub list_id: Option<FIXString>,
    pub side: Option<Side>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub net_gross_ind: Option<NetGrossInd>,
    pub settl_type: Option<SettlType>,
    pub settl_date: Option<LocalMktDate>,
    pub account: Option<FIXString>,
    pub acct_id_source: Option<AcctIDSource>
}

pub struct BidCompRspGrp {
    pub commission_data: CommissionData,
    pub list_id: Option<FIXString>,
    pub country: Option<Country>,
    pub side: Option<Side>,
    pub price: Option<Price>,
    pub price_type: Option<PriceType>,
    pub fair_value: Option<Amt>,
    pub net_gross_ind: Option<NetGrossInd>,
    pub settl_type: Option<SettlType>,
    pub settl_date: Option<LocalMktDate>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>
}

pub struct BidDescReqGrp {
    pub bid_descriptor_type: Option<BidDescriptorType>,
    pub bid_descriptor: Option<FIXString>,
    pub side_value_ind: Option<SideValueInd>,
    pub liquidity_value: Option<Amt>,
    pub liquidity_num_securities: Option<FIXInt>,
    pub liquidity_pct_low: Option<Percentage>,
    pub liquidity_pct_high: Option<Percentage>,
    pub efp_tracking_error: Option<Percentage>,
    pub fair_value: Option<Amt>,
    pub outside_index_pct: Option<Percentage>,
    pub value_of_futures: Option<Amt>
}

pub struct ClrInstGrp {
    pub clearing_instruction: Option<ClearingInstruction>
}

pub struct CollInqQualGrp {
    pub coll_inquiry_qualifier: Option<CollInquiryQualifier>
}

pub struct CommissionData {
    pub commission: Option<Amt>,
    pub comm_type: Option<CommType>,
    pub comm_currency: Option<Currency>,
    pub fund_renew_waiv: Option<FundRenewWaiv>
}

pub struct CompIDReqGrp {
    pub ref_comp_id: Option<FIXString>,
    pub ref_sub_id: Option<FIXString>,
    pub location_id: Option<FIXString>,
    pub desk_id: Option<FIXString>
}

pub struct CompIDStatGrp {
    pub ref_comp_id: Option<FIXString>,
    pub ref_sub_id: Option<FIXString>,
    pub location_id: Option<FIXString>,
    pub desk_id: Option<FIXString>,
    pub status_value: Option<StatusValue>,
    pub status_text: Option<FIXString>
}

pub struct ContAmtGrp {
    pub cont_amt_type: Option<ContAmtType>,
    pub cont_amt_value: Option<FIXFloat>,
    pub cont_amt_curr: Option<Currency>
}

pub struct ContraGrp {
    pub contra_broker: Option<FIXString>,
    pub contra_trader: Option<FIXString>,
    pub contra_trade_qty: Option<Qty>,
    pub contra_trade_time: Option<UTCTimestamp>,
    pub contra_leg_ref_id: Option<FIXString>
}

pub struct CpctyConfGrp {
    pub order_capacity: OrderCapacity,
    pub order_restrictions: Option<OrderRestrictions>,
    pub order_capacity_qty: Qty
}

pub struct DiscretionInstructions {
    pub discretion_inst: Option<DiscretionInst>,
    pub discretion_offset_value: Option<FIXFloat>,
    pub discretion_move_type: Option<DiscretionMoveType>,
    pub discretion_offset_type: Option<DiscretionOffsetType>,
    pub discretion_limit_type: Option<DiscretionLimitType>,
    pub discretion_round_direction: Option<DiscretionRoundDirection>,
    pub discretion_scope: Option<DiscretionScope>
}

pub struct DlvyInstGrp {
    pub settl_inst_source: Option<SettlInstSource>,
    pub dlvy_inst_type: Option<DlvyInstType>,
    pub settl_parties: Option<SettlParties>
}

pub struct EvntGrp {
    pub event_type: Option<EventType>,
    pub event_date: Option<LocalMktDate>,
    pub event_px: Option<Price>,
    pub event_text: Option<FIXString>
}

pub struct ExecAllocGrp {
    pub last_qty: Option<Qty>,
    pub exec_id: Option<FIXString>,
    pub secondary_exec_id: Option<FIXString>,
    pub last_px: Option<Price>,
    pub last_par_px: Option<Price>,
    pub last_capacity: Option<LastCapacity>
}

pub struct ExecCollGrp {
    pub exec_id: Option<FIXString>
}

pub struct ExecsGrp {
    pub exec_id: Option<FIXString>
}

pub struct FinancingDetails {
    pub agreement_desc: Option<FIXString>,
    pub agreement_id: Option<FIXString>,
    pub agreement_date: Option<LocalMktDate>,
    pub agreement_currency: Option<Currency>,
    pub termination_type: Option<TerminationType>,
    pub start_date: Option<LocalMktDate>,
    pub end_date: Option<LocalMktDate>,
    pub delivery_type: Option<DeliveryType>,
    pub margin_ratio: Option<Percentage>
}

pub struct Hop {
    pub hop_comp_id: Option<FIXString>,
    pub hop_sending_time: Option<UTCTimestamp>,
    pub hop_ref_id: Option<SeqNum>
}

pub struct IOIQualGrp {
    pub ioi_qualifier: Option<IOIQualifier>
}

pub struct InstrmtGrp {
    pub instrument: Option<Instrument>
}

pub struct InstrmtLegExecGrp {
    pub instrument_leg: Option<InstrumentLeg>,
    pub leg_qty: Option<Qty>,
    pub leg_swap_type: Option<LegSwapType>,
    pub leg_stipulations: Option<LegStipulations>,
    pub leg_position_effect: Option<FIXChar>,
    pub leg_covered_or_uncovered: Option<FIXInt>,
    pub nested_parties: Option<NestedParties>,
    pub leg_ref_id: Option<FIXString>,
    pub leg_price: Option<Price>,
    pub leg_settl_type: Option<FIXChar>,
    pub leg_settl_date: Option<LocalMktDate>,
    pub leg_last_px: Option<Price>
}

pub struct InstrmtLegGrp {
    pub instrument_leg: Option<InstrumentLeg>
}

pub struct InstrmtLegIOIGrp {
    pub instrument_leg: Option<InstrumentLeg>,
    pub leg_ioi_qty: Option<FIXString>,
    pub leg_stipulations: Option<LegStipulations>
}

pub struct InstrmtLegSecListGrp {
    pub instrument_leg: Option<InstrumentLeg>,
    pub leg_swap_type: Option<LegSwapType>,
    pub leg_settl_type: Option<FIXChar>,
    pub leg_stipulations: Option<LegStipulations>,
    pub leg_benchmark_curve_data: Option<LegBenchmarkCurveData>
}

pub struct InstrmtMDReqGrp {
    pub instrument: Instrument,
    pub und_instrmt_grp: Option<UndInstrmtGrp>,
    pub instrmt_leg_grp: Option<InstrmtLegGrp>
}

pub struct InstrmtStrkPxGrp {
    pub instrument: Instrument
}

pub struct Instrument {
    pub symbol: Option<FIXString>,
    pub symbol_sfx: Option<FIXString>,
    pub security_id: Option<FIXString>,
    pub security_id_source: Option<SecurityIDSource>,
    pub sec_alt_id_grp: Option<SecAltIDGrp>,
    pub product: Option<Product>,
    pub cfi_code: Option<FIXString>,
    pub security_type: Option<SecurityType>,
    pub security_sub_type: Option<FIXString>,
    pub maturity_month_year: Option<MonthYear>,
    pub maturity_date: Option<LocalMktDate>,
    pub put_or_call: Option<PutOrCall>,
    pub coupon_payment_date: Option<LocalMktDate>,
    pub issue_date: Option<LocalMktDate>,
    pub repo_collateral_security_type: Option<FIXString>,
    pub repurchase_term: Option<FIXInt>,
    pub repurchase_rate: Option<Percentage>,
    pub factor: Option<FIXFloat>,
    pub credit_rating: Option<FIXString>,
    pub instr_registry: Option<FIXString>,
    pub country_of_issue: Option<Country>,
    pub state_or_province_of_issue: Option<FIXString>,
    pub locale_of_issue: Option<FIXString>,
    pub redemption_date: Option<LocalMktDate>,
    pub strike_price: Option<Price>,
    pub strike_currency: Option<Currency>,
    pub opt_attribute: Option<FIXChar>,
    pub contract_multiplier: Option<FIXFloat>,
    pub coupon_rate: Option<Percentage>,
    pub security_exchange: Option<MICExchange>,
    pub issuer: Option<FIXString>,
    pub encoded_issuer: Option<Data>,
    pub security_desc: Option<FIXString>,
    pub encoded_security_desc: Option<Data>,
    pub pool: Option<FIXString>,
    pub contract_settl_month: Option<MonthYear>,
    pub cp_program: Option<CPProgram>,
    pub cp_reg_type: Option<FIXString>,
    pub evnt_grp: Option<EvntGrp>,
    pub dated_date: Option<LocalMktDate>,
    pub interest_accrual_date: Option<LocalMktDate>
}

pub struct InstrumentExtension {
    pub delivery_form: Option<DeliveryForm>,
    pub pct_at_risk: Option<Percentage>,
    pub attrb_grp: Option<AttrbGrp>
}

pub struct InstrumentLeg {
    pub leg_symbol: Option<FIXString>,
    pub leg_symbol_sfx: Option<FIXString>,
    pub leg_security_id: Option<FIXString>,
    pub leg_security_id_source: Option<FIXString>,
    pub leg_sec_alt_id_grp: Option<LegSecAltIDGrp>,
    pub leg_product: Option<FIXInt>,
    pub leg_cfi_code: Option<FIXString>,
    pub leg_security_type: Option<FIXString>,
    pub leg_security_sub_type: Option<FIXString>,
    pub leg_maturity_month_year: Option<MonthYear>,
    pub leg_maturity_date: Option<LocalMktDate>,
    pub leg_coupon_payment_date: Option<LocalMktDate>,
    pub leg_issue_date: Option<LocalMktDate>,
    pub leg_repo_collateral_security_type: Option<FIXString>,
    pub leg_repurchase_term: Option<FIXInt>,
    pub leg_repurchase_rate: Option<Percentage>,
    pub leg_factor: Option<FIXFloat>,
    pub leg_credit_rating: Option<FIXString>,
    pub leg_instr_registry: Option<FIXString>,
    pub leg_country_of_issue: Option<Country>,
    pub leg_state_or_province_of_issue: Option<FIXString>,
    pub leg_locale_of_issue: Option<FIXString>,
    pub leg_redemption_date: Option<LocalMktDate>,
    pub leg_strike_price: Option<Price>,
    pub leg_strike_currency: Option<Currency>,
    pub leg_opt_attribute: Option<FIXChar>,
    pub leg_contract_multiplier: Option<FIXFloat>,
    pub leg_coupon_rate: Option<Percentage>,
    pub leg_security_exchange: Option<MICExchange>,
    pub leg_issuer: Option<FIXString>,
    pub encoded_leg_issuer: Option<Data>,
    pub leg_security_desc: Option<FIXString>,
    pub encoded_leg_security_desc: Option<Data>,
    pub leg_ratio_qty: Option<FIXFloat>,
    pub leg_side: Option<FIXChar>,
    pub leg_currency: Option<Currency>,
    pub leg_pool: Option<FIXString>,
    pub leg_dated_date: Option<LocalMktDate>,
    pub leg_contract_settl_month: Option<MonthYear>,
    pub leg_interest_accrual_date: Option<LocalMktDate>
}

pub struct LegBenchmarkCurveData {
    pub leg_benchmark_curve_currency: Option<Currency>,
    pub leg_benchmark_curve_name: Option<FIXString>,
    pub leg_benchmark_curve_point: Option<FIXString>,
    pub leg_benchmark_price: Option<Price>,
    pub leg_benchmark_price_type: Option<FIXInt>
}

pub struct LegOrdGrp {
    pub instrument_leg: Option<InstrumentLeg>,
    pub leg_qty: Option<Qty>,
    pub leg_swap_type: Option<LegSwapType>,
    pub leg_stipulations: Option<LegStipulations>,
    pub leg_pre_alloc_grp: Option<LegPreAllocGrp>,
    pub leg_position_effect: Option<FIXChar>,
    pub leg_covered_or_uncovered: Option<FIXInt>,
    pub nested_parties: Option<NestedParties>,
    pub leg_ref_id: Option<FIXString>,
    pub leg_price: Option<Price>,
    pub leg_settl_type: Option<FIXChar>,
    pub leg_settl_date: Option<LocalMktDate>
}

pub struct LegPreAllocGrp {
    pub leg_alloc_account: Option<FIXString>,
    pub leg_individual_alloc_id: Option<FIXString>,
    pub nested_parties2: Option<NestedParties2>,
    pub leg_alloc_qty: Option<Qty>,
    pub leg_alloc_acct_id_source: Option<FIXString>,
    pub leg_settl_currency: Option<Currency>
}

pub struct LegQuotGrp {
    pub instrument_leg: Option<InstrumentLeg>,
    pub leg_qty: Option<Qty>,
    pub leg_swap_type: Option<LegSwapType>,
    pub leg_settl_type: Option<FIXChar>,
    pub leg_settl_date: Option<LocalMktDate>,
    pub leg_stipulations: Option<LegStipulations>,
    pub nested_parties: Option<NestedParties>,
    pub leg_price_type: Option<FIXInt>,
    pub leg_bid_px: Option<Price>,
    pub leg_offer_px: Option<Price>,
    pub leg_benchmark_curve_data: Option<LegBenchmarkCurveData>
}

pub struct LegQuotStatGrp {
    pub instrument_leg: Option<InstrumentLeg>,
    pub leg_qty: Option<Qty>,
    pub leg_swap_type: Option<LegSwapType>,
    pub leg_settl_type: Option<FIXChar>,
    pub leg_settl_date: Option<LocalMktDate>,
    pub leg_stipulations: Option<LegStipulations>,
    pub nested_parties: Option<NestedParties>
}

pub struct LegSecAltIDGrp {
    pub leg_security_alt_id: Option<FIXString>,
    pub leg_security_alt_id_source: Option<FIXString>
}

pub struct LegStipulations {
    pub leg_stipulation_type: Option<FIXString>,
    pub leg_stipulation_value: Option<FIXString>
}

pub struct LinesOfTextGrp {
    pub text: FIXString,
    pub encoded_text: Option<Data>
}

pub struct ListOrdGrp {
    pub cl_ord_id: FIXString,
    pub secondary_cl_ord_id: Option<FIXString>,
    pub list_seq_no: FIXInt,
    pub cl_ord_link_id: Option<FIXString>,
    pub settl_inst_mode: Option<SettlInstMode>,
    pub parties: Option<Parties>,
    pub trade_origination_date: Option<LocalMktDate>,
    pub trade_date: Option<LocalMktDate>,
    pub account: Option<FIXString>,
    pub acct_id_source: Option<AcctIDSource>,
    pub account_type: Option<AccountType>,
    pub day_booking_inst: Option<DayBookingInst>,
    pub booking_unit: Option<BookingUnit>,
    pub alloc_id: Option<FIXString>,
    pub prealloc_method: Option<PreallocMethod>,
    pub pre_alloc_grp: Option<PreAllocGrp>,
    pub settl_type: Option<SettlType>,
    pub settl_date: Option<LocalMktDate>,
    pub cash_margin: Option<CashMargin>,
    pub clearing_fee_indicator: Option<ClearingFeeIndicator>,
    pub handl_inst: Option<HandlInst>,
    pub exec_inst: Option<ExecInst>,
    pub min_qty: Option<Qty>,
    pub max_floor: Option<Qty>,
    pub ex_destination: Option<MICExchange>,
    pub trdg_ses_grp: Option<TrdgSesGrp>,
    pub process_code: Option<ProcessCode>,
    pub instrument: Instrument,
    pub und_instrmt_grp: Option<UndInstrmtGrp>,
    pub prev_close_px: Option<Price>,
    pub side: Side,
    pub side_value_ind: Option<SideValueInd>,
    pub locate_reqd: Option<LocateReqd>,
    pub transact_time: Option<UTCTimestamp>,
    pub stipulations: Option<Stipulations>,
    pub qty_type: Option<QtyType>,
    pub order_qty_data: OrderQtyData,
    pub ord_type: Option<OrdType>,
    pub price_type: Option<PriceType>,
    pub price: Option<Price>,
    pub stop_px: Option<Price>,
    pub spread_or_benchmark_curve_data: Option<SpreadOrBenchmarkCurveData>,
    pub yield_data: Option<YieldData>,
    pub currency: Option<Currency>,
    pub compliance_id: Option<FIXString>,
    pub solicited_flag: Option<SolicitedFlag>,
    pub ioiid: Option<FIXString>,
    pub quote_id: Option<FIXString>,
    pub time_in_force: Option<TimeInForce>,
    pub effective_time: Option<UTCTimestamp>,
    pub expire_date: Option<LocalMktDate>,
    pub expire_time: Option<UTCTimestamp>,
    pub gt_booking_inst: Option<GTBookingInst>,
    pub commission_data: Option<CommissionData>,
    pub order_capacity: Option<OrderCapacity>,
    pub order_restrictions: Option<OrderRestrictions>,
    pub cust_order_capacity: Option<CustOrderCapacity>,
    pub forex_req: Option<ForexReq>,
    pub settl_currency: Option<Currency>,
    pub booking_type: Option<BookingType>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>,
    pub settl_date2: Option<LocalMktDate>,
    pub order_qty2: Option<Qty>,
    pub price2: Option<Price>,
    pub position_effect: Option<PositionEffect>,
    pub covered_or_uncovered: Option<CoveredOrUncovered>,
    pub max_show: Option<Qty>,
    pub peg_instructions: Option<PegInstructions>,
    pub discretion_instructions: Option<DiscretionInstructions>,
    pub target_strategy: Option<TargetStrategy>,
    pub target_strategy_parameters: Option<FIXString>,
    pub participation_rate: Option<Percentage>,
    pub designation: Option<FIXString>
}

pub struct MDFullGrp {
    pub md_entry_type: MDEntryType,
    pub md_entry_px: Option<Price>,
    pub currency: Option<Currency>,
    pub md_entry_size: Option<Qty>,
    pub md_entry_date: Option<UTCDateOnly>,
    pub md_entry_time: Option<UTCTimeOnly>,
    pub tick_direction: Option<TickDirection>,
    pub md_mkt: Option<MICExchange>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub quote_condition: Option<QuoteCondition>,
    pub trade_condition: Option<TradeCondition>,
    pub md_entry_originator: Option<FIXString>,
    pub location_id: Option<FIXString>,
    pub desk_id: Option<FIXString>,
    pub open_close_settl_flag: Option<OpenCloseSettlFlag>,
    pub time_in_force: Option<TimeInForce>,
    pub expire_date: Option<LocalMktDate>,
    pub expire_time: Option<UTCTimestamp>,
    pub min_qty: Option<Qty>,
    pub exec_inst: Option<ExecInst>,
    pub seller_days: Option<FIXInt>,
    pub order_id: Option<FIXString>,
    pub quote_entry_id: Option<FIXString>,
    pub md_entry_buyer: Option<FIXString>,
    pub md_entry_seller: Option<FIXString>,
    pub number_of_orders: Option<FIXInt>,
    pub md_entry_position_no: Option<FIXInt>,
    pub scope: Option<Scope>,
    pub price_delta: Option<FIXFloat>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>
}

pub struct MDIncGrp {
    pub md_update_action: MDUpdateAction,
    pub delete_reason: Option<DeleteReason>,
    pub md_entry_type: Option<MDEntryType>,
    pub md_entry_id: Option<FIXString>,
    pub md_entry_ref_id: Option<FIXString>,
    pub instrument: Option<Instrument>,
    pub und_instrmt_grp: Option<UndInstrmtGrp>,
    pub instrmt_leg_grp: Option<InstrmtLegGrp>,
    pub financial_status: Option<FinancialStatus>,
    pub corporate_action: Option<CorporateAction>,
    pub md_entry_px: Option<Price>,
    pub currency: Option<Currency>,
    pub md_entry_size: Option<Qty>,
    pub md_entry_date: Option<UTCDateOnly>,
    pub md_entry_time: Option<UTCTimeOnly>,
    pub tick_direction: Option<TickDirection>,
    pub md_mkt: Option<MICExchange>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub quote_condition: Option<QuoteCondition>,
    pub trade_condition: Option<TradeCondition>,
    pub md_entry_originator: Option<FIXString>,
    pub location_id: Option<FIXString>,
    pub desk_id: Option<FIXString>,
    pub open_close_settl_flag: Option<OpenCloseSettlFlag>,
    pub time_in_force: Option<TimeInForce>,
    pub expire_date: Option<LocalMktDate>,
    pub expire_time: Option<UTCTimestamp>,
    pub min_qty: Option<Qty>,
    pub exec_inst: Option<ExecInst>,
    pub seller_days: Option<FIXInt>,
    pub order_id: Option<FIXString>,
    pub quote_entry_id: Option<FIXString>,
    pub md_entry_buyer: Option<FIXString>,
    pub md_entry_seller: Option<FIXString>,
    pub number_of_orders: Option<FIXInt>,
    pub md_entry_position_no: Option<FIXInt>,
    pub scope: Option<Scope>,
    pub price_delta: Option<FIXFloat>,
    pub net_chg_prev_day: Option<PriceOffset>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>
}

pub struct MDReqGrp {
    pub md_entry_type: MDEntryType
}

pub struct MDRjctGrp {
    pub alt_md_source_id: Option<FIXString>
}

pub struct MiscFeesGrp {
    pub misc_fee_amt: Option<Amt>,
    pub misc_fee_curr: Option<Currency>,
    pub misc_fee_type: Option<MiscFeeType>,
    pub misc_fee_basis: Option<MiscFeeBasis>
}

pub struct NestedParties {
    pub nested_party_id: Option<FIXString>,
    pub nested_party_id_source: Option<FIXChar>,
    pub nested_party_role: Option<FIXInt>,
    pub nstd_ptys_sub_grp: Option<NstdPtysSubGrp>
}

pub struct NestedParties2 {
    pub nested2_party_id: Option<FIXString>,
    pub nested2_party_id_source: Option<FIXChar>,
    pub nested2_party_role: Option<FIXInt>,
    pub nstd_ptys2_sub_grp: Option<NstdPtys2SubGrp>
}

pub struct NestedParties3 {
    pub nested3_party_id: Option<FIXString>,
    pub nested3_party_id_source: Option<FIXChar>,
    pub nested3_party_role: Option<FIXInt>,
    pub nstd_ptys3_sub_grp: Option<NstdPtys3SubGrp>
}

pub struct NstdPtys2SubGrp {
    pub nested2_party_sub_id: Option<FIXString>,
    pub nested2_party_sub_id_type: Option<FIXInt>
}

pub struct NstdPtys3SubGrp {
    pub nested3_party_sub_id: Option<FIXString>,
    pub nested3_party_sub_id_type: Option<FIXInt>
}

pub struct NstdPtysSubGrp {
    pub nested_party_sub_id: Option<FIXString>,
    pub nested_party_sub_id_type: Option<FIXInt>
}

pub struct OrdAllocGrp {
    pub cl_ord_id: Option<FIXString>,
    pub order_id: Option<FIXString>,
    pub secondary_order_id: Option<FIXString>,
    pub secondary_cl_ord_id: Option<FIXString>,
    pub list_id: Option<FIXString>,
    pub nested_parties2: Option<NestedParties2>,
    pub order_qty: Option<Qty>,
    pub order_avg_px: Option<Price>,
    pub order_booking_qty: Option<Qty>
}

pub struct OrdListStatGrp {
    pub cl_ord_id: FIXString,
    pub secondary_cl_ord_id: Option<FIXString>,
    pub cum_qty: Qty,
    pub ord_status: OrdStatus,
    pub working_indicator: Option<WorkingIndicator>,
    pub leaves_qty: Qty,
    pub cxl_qty: Qty,
    pub avg_px: Price,
    pub ord_rej_reason: Option<OrdRejReason>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>
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
    pub ptys_sub_grp: Option<PtysSubGrp>
}

pub struct PegInstructions {
    pub peg_offset_value: Option<FIXFloat>,
    pub peg_move_type: Option<PegMoveType>,
    pub peg_offset_type: Option<PegOffsetType>,
    pub peg_limit_type: Option<PegLimitType>,
    pub peg_round_direction: Option<PegRoundDirection>,
    pub peg_scope: Option<PegScope>
}

pub struct PosUndInstrmtGrp {
    pub underlying_instrument: Option<UnderlyingInstrument>,
    pub underlying_settl_price: Price,
    pub underlying_settl_price_type: FIXInt
}

pub struct PositionAmountData {
    pub pos_amt_type: Option<PosAmtType>,
    pub pos_amt: Option<Amt>
}

pub struct PositionQty {
    pub pos_type: Option<PosType>,
    pub long_qty: Option<Qty>,
    pub short_qty: Option<Qty>,
    pub pos_qty_status: Option<PosQtyStatus>,
    pub nested_parties: Option<NestedParties>
}

pub struct PreAllocGrp {
    pub alloc_account: Option<FIXString>,
    pub alloc_acct_id_source: Option<FIXInt>,
    pub alloc_settl_currency: Option<Currency>,
    pub individual_alloc_id: Option<FIXString>,
    pub nested_parties: Option<NestedParties>,
    pub alloc_qty: Option<Qty>
}

pub struct PreAllocMlegGrp {
    pub alloc_account: Option<FIXString>,
    pub alloc_acct_id_source: Option<FIXInt>,
    pub alloc_settl_currency: Option<Currency>,
    pub individual_alloc_id: Option<FIXString>,
    pub nested_parties3: Option<NestedParties3>,
    pub alloc_qty: Option<Qty>
}

pub struct PtysSubGrp {
    pub party_sub_id: Option<FIXString>,
    pub party_sub_id_type: Option<PartySubIDType>
}

pub struct QuotCxlEntriesGrp {
    pub instrument: Option<Instrument>,
    pub financing_details: Option<FinancingDetails>,
    pub und_instrmt_grp: Option<UndInstrmtGrp>,
    pub instrmt_leg_grp: Option<InstrmtLegGrp>
}

pub struct QuotEntryAckGrp {
    pub quote_entry_id: Option<FIXString>,
    pub instrument: Option<Instrument>,
    pub instrmt_leg_grp: Option<InstrmtLegGrp>,
    pub bid_px: Option<Price>,
    pub offer_px: Option<Price>,
    pub bid_size: Option<Qty>,
    pub offer_size: Option<Qty>,
    pub valid_until_time: Option<UTCTimestamp>,
    pub bid_spot_rate: Option<Price>,
    pub offer_spot_rate: Option<Price>,
    pub bid_forward_points: Option<PriceOffset>,
    pub offer_forward_points: Option<PriceOffset>,
    pub mid_px: Option<Price>,
    pub bid_yield: Option<Percentage>,
    pub mid_yield: Option<Percentage>,
    pub offer_yield: Option<Percentage>,
    pub transact_time: Option<UTCTimestamp>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub settl_date: Option<LocalMktDate>,
    pub ord_type: Option<OrdType>,
    pub settl_date2: Option<LocalMktDate>,
    pub order_qty2: Option<Qty>,
    pub bid_forward_points2: Option<PriceOffset>,
    pub offer_forward_points2: Option<PriceOffset>,
    pub currency: Option<Currency>,
    pub quote_entry_reject_reason: Option<FIXInt>
}

pub struct QuotEntryGrp {
    pub quote_entry_id: FIXString,
    pub instrument: Option<Instrument>,
    pub instrmt_leg_grp: Option<InstrmtLegGrp>,
    pub bid_px: Option<Price>,
    pub offer_px: Option<Price>,
    pub bid_size: Option<Qty>,
    pub offer_size: Option<Qty>,
    pub valid_until_time: Option<UTCTimestamp>,
    pub bid_spot_rate: Option<Price>,
    pub offer_spot_rate: Option<Price>,
    pub bid_forward_points: Option<PriceOffset>,
    pub offer_forward_points: Option<PriceOffset>,
    pub mid_px: Option<Price>,
    pub bid_yield: Option<Percentage>,
    pub mid_yield: Option<Percentage>,
    pub offer_yield: Option<Percentage>,
    pub transact_time: Option<UTCTimestamp>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub settl_date: Option<LocalMktDate>,
    pub ord_type: Option<OrdType>,
    pub settl_date2: Option<LocalMktDate>,
    pub order_qty2: Option<Qty>,
    pub bid_forward_points2: Option<PriceOffset>,
    pub offer_forward_points2: Option<PriceOffset>,
    pub currency: Option<Currency>
}

pub struct QuotQualGrp {
    pub quote_qualifier: Option<FIXChar>
}

pub struct QuotReqGrp {
    pub instrument: Instrument,
    pub financing_details: Option<FinancingDetails>,
    pub und_instrmt_grp: Option<UndInstrmtGrp>,
    pub prev_close_px: Option<Price>,
    pub quote_request_type: Option<QuoteRequestType>,
    pub quote_type: Option<QuoteType>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub trade_origination_date: Option<LocalMktDate>,
    pub side: Option<Side>,
    pub qty_type: Option<QtyType>,
    pub order_qty_data: Option<OrderQtyData>,
    pub settl_type: Option<SettlType>,
    pub settl_date: Option<LocalMktDate>,
    pub settl_date2: Option<LocalMktDate>,
    pub order_qty2: Option<Qty>,
    pub currency: Option<Currency>,
    pub stipulations: Option<Stipulations>,
    pub account: Option<FIXString>,
    pub acct_id_source: Option<AcctIDSource>,
    pub account_type: Option<AccountType>,
    pub quot_req_legs_grp: Option<QuotReqLegsGrp>,
    pub quot_qual_grp: Option<QuotQualGrp>,
    pub quote_price_type: Option<QuotePriceType>,
    pub ord_type: Option<OrdType>,
    pub valid_until_time: Option<UTCTimestamp>,
    pub expire_time: Option<UTCTimestamp>,
    pub transact_time: Option<UTCTimestamp>,
    pub spread_or_benchmark_curve_data: Option<SpreadOrBenchmarkCurveData>,
    pub price_type: Option<PriceType>,
    pub price: Option<Price>,
    pub price2: Option<Price>,
    pub yield_data: Option<YieldData>,
    pub parties: Option<Parties>
}

pub struct QuotReqLegsGrp {
    pub instrument_leg: Option<InstrumentLeg>,
    pub leg_qty: Option<Qty>,
    pub leg_swap_type: Option<LegSwapType>,
    pub leg_settl_type: Option<FIXChar>,
    pub leg_settl_date: Option<LocalMktDate>,
    pub leg_stipulations: Option<LegStipulations>,
    pub nested_parties: Option<NestedParties>,
    pub leg_benchmark_curve_data: Option<LegBenchmarkCurveData>
}

pub struct QuotReqRjctGrp {
    pub instrument: Instrument,
    pub financing_details: Option<FinancingDetails>,
    pub und_instrmt_grp: Option<UndInstrmtGrp>,
    pub prev_close_px: Option<Price>,
    pub quote_request_type: Option<QuoteRequestType>,
    pub quote_type: Option<QuoteType>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub trade_origination_date: Option<LocalMktDate>,
    pub side: Option<Side>,
    pub qty_type: Option<QtyType>,
    pub order_qty_data: Option<OrderQtyData>,
    pub settl_type: Option<SettlType>,
    pub settl_date: Option<LocalMktDate>,
    pub settl_date2: Option<LocalMktDate>,
    pub order_qty2: Option<Qty>,
    pub currency: Option<Currency>,
    pub stipulations: Option<Stipulations>,
    pub account: Option<FIXString>,
    pub acct_id_source: Option<AcctIDSource>,
    pub account_type: Option<AccountType>,
    pub quot_req_legs_grp: Option<QuotReqLegsGrp>,
    pub quot_qual_grp: Option<QuotQualGrp>,
    pub quote_price_type: Option<QuotePriceType>,
    pub ord_type: Option<OrdType>,
    pub expire_time: Option<UTCTimestamp>,
    pub transact_time: Option<UTCTimestamp>,
    pub spread_or_benchmark_curve_data: Option<SpreadOrBenchmarkCurveData>,
    pub price_type: Option<PriceType>,
    pub price: Option<Price>,
    pub price2: Option<Price>,
    pub yield_data: Option<YieldData>,
    pub parties: Option<Parties>
}

pub struct QuotSetAckGrp {
    pub quote_set_id: Option<FIXString>,
    pub underlying_instrument: Option<UnderlyingInstrument>,
    pub tot_no_quote_entries: Option<FIXInt>,
    pub last_fragment: Option<LastFragment>,
    pub quot_entry_ack_grp: Option<QuotEntryAckGrp>
}

pub struct QuotSetGrp {
    pub quote_set_id: FIXString,
    pub underlying_instrument: Option<UnderlyingInstrument>,
    pub quote_set_valid_until_time: Option<UTCTimestamp>,
    pub tot_no_quote_entries: FIXInt,
    pub last_fragment: Option<LastFragment>,
    pub quot_entry_grp: QuotEntryGrp
}

pub struct RFQReqGrp {
    pub instrument: Instrument,
    pub und_instrmt_grp: Option<UndInstrmtGrp>,
    pub instrmt_leg_grp: Option<InstrmtLegGrp>,
    pub prev_close_px: Option<Price>,
    pub quote_request_type: Option<QuoteRequestType>,
    pub quote_type: Option<QuoteType>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>
}

pub struct RelSymDerivSecGrp {
    pub instrument: Option<Instrument>,
    pub currency: Option<Currency>,
    pub expiration_cycle: Option<ExpirationCycle>,
    pub instrument_extension: Option<InstrumentExtension>,
    pub instrmt_leg_grp: Option<InstrmtLegGrp>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>
}

pub struct RgstDistInstGrp {
    pub distrib_payment_method: Option<DistribPaymentMethod>,
    pub distrib_percentage: Option<Percentage>,
    pub cash_distrib_curr: Option<Currency>,
    pub cash_distrib_agent_name: Option<FIXString>,
    pub cash_distrib_agent_code: Option<FIXString>,
    pub cash_distrib_agent_acct_number: Option<FIXString>,
    pub cash_distrib_pay_ref: Option<FIXString>,
    pub cash_distrib_agent_acct_name: Option<FIXString>
}

pub struct RgstDtlsGrp {
    pub regist_dtls: Option<FIXString>,
    pub regist_email: Option<FIXString>,
    pub mailing_dtls: Option<FIXString>,
    pub mailing_inst: Option<FIXString>,
    pub nested_parties: Option<NestedParties>,
    pub owner_type: Option<OwnerType>,
    pub date_of_birth: Option<LocalMktDate>,
    pub investor_country_of_residence: Option<Country>
}

pub struct RoutingGrp {
    pub routing_type: Option<RoutingType>,
    pub routing_id: Option<FIXString>
}

pub struct SecAltIDGrp {
    pub security_alt_id: Option<FIXString>,
    pub security_alt_id_source: Option<FIXString>
}

pub struct SecListGrp {
    pub instrument: Option<Instrument>,
    pub instrument_extension: Option<InstrumentExtension>,
    pub financing_details: Option<FinancingDetails>,
    pub und_instrmt_grp: Option<UndInstrmtGrp>,
    pub currency: Option<Currency>,
    pub stipulations: Option<Stipulations>,
    pub instrmt_leg_sec_list_grp: Option<InstrmtLegSecListGrp>,
    pub spread_or_benchmark_curve_data: Option<SpreadOrBenchmarkCurveData>,
    pub yield_data: Option<YieldData>,
    pub round_lot: Option<Qty>,
    pub min_trade_vol: Option<Qty>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub expiration_cycle: Option<ExpirationCycle>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>
}

pub struct SecTypesGrp {
    pub security_type: Option<SecurityType>,
    pub security_sub_type: Option<FIXString>,
    pub product: Option<Product>,
    pub cfi_code: Option<FIXString>
}

pub struct SettlInstGrp {
    pub settl_inst_id: Option<FIXString>,
    pub settl_inst_trans_type: Option<SettlInstTransType>,
    pub settl_inst_ref_id: Option<FIXString>,
    pub parties: Option<Parties>,
    pub side: Option<Side>,
    pub product: Option<Product>,
    pub security_type: Option<SecurityType>,
    pub cfi_code: Option<FIXString>,
    pub effective_time: Option<UTCTimestamp>,
    pub expire_time: Option<UTCTimestamp>,
    pub last_update_time: Option<UTCTimestamp>,
    pub settl_instructions_data: Option<SettlInstructionsData>,
    pub payment_method: Option<PaymentMethod>,
    pub payment_ref: Option<FIXString>,
    pub card_holder_name: Option<FIXString>,
    pub card_number: Option<FIXString>,
    pub card_start_date: Option<LocalMktDate>,
    pub card_exp_date: Option<LocalMktDate>,
    pub card_iss_num: Option<FIXString>,
    pub payment_date: Option<LocalMktDate>,
    pub payment_remitter_id: Option<FIXString>
}

pub struct SettlInstructionsData {
    pub settl_delivery_type: Option<SettlDeliveryType>,
    pub stand_inst_db_type: Option<StandInstDbType>,
    pub stand_inst_db_name: Option<FIXString>,
    pub stand_inst_db_id: Option<FIXString>,
    pub dlvy_inst_grp: Option<DlvyInstGrp>
}

pub struct SettlParties {
    pub settl_party_id: Option<FIXString>,
    pub settl_party_id_source: Option<FIXChar>,
    pub settl_party_role: Option<FIXInt>,
    pub settl_ptys_sub_grp: Option<SettlPtysSubGrp>
}

pub struct SettlPtysSubGrp {
    pub settl_party_sub_id: Option<FIXString>,
    pub settl_party_sub_id_type: Option<FIXInt>
}

pub struct SideCrossOrdCxlGrp {
    pub side: Side,
    pub orig_cl_ord_id: FIXString,
    pub cl_ord_id: FIXString,
    pub secondary_cl_ord_id: Option<FIXString>,
    pub cl_ord_link_id: Option<FIXString>,
    pub orig_ord_mod_time: Option<UTCTimestamp>,
    pub parties: Option<Parties>,
    pub trade_origination_date: Option<LocalMktDate>,
    pub trade_date: Option<LocalMktDate>,
    pub order_qty_data: OrderQtyData,
    pub compliance_id: Option<FIXString>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>
}

pub struct SideCrossOrdModGrp {
    pub side: Side,
    pub cl_ord_id: FIXString,
    pub secondary_cl_ord_id: Option<FIXString>,
    pub cl_ord_link_id: Option<FIXString>,
    pub parties: Option<Parties>,
    pub trade_origination_date: Option<LocalMktDate>,
    pub trade_date: Option<LocalMktDate>,
    pub account: Option<FIXString>,
    pub acct_id_source: Option<AcctIDSource>,
    pub account_type: Option<AccountType>,
    pub day_booking_inst: Option<DayBookingInst>,
    pub booking_unit: Option<BookingUnit>,
    pub prealloc_method: Option<PreallocMethod>,
    pub alloc_id: Option<FIXString>,
    pub pre_alloc_grp: Option<PreAllocGrp>,
    pub qty_type: Option<QtyType>,
    pub order_qty_data: OrderQtyData,
    pub commission_data: Option<CommissionData>,
    pub order_capacity: Option<OrderCapacity>,
    pub order_restrictions: Option<OrderRestrictions>,
    pub cust_order_capacity: Option<CustOrderCapacity>,
    pub forex_req: Option<ForexReq>,
    pub settl_currency: Option<Currency>,
    pub booking_type: Option<BookingType>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>,
    pub position_effect: Option<PositionEffect>,
    pub covered_or_uncovered: Option<CoveredOrUncovered>,
    pub cash_margin: Option<CashMargin>,
    pub clearing_fee_indicator: Option<ClearingFeeIndicator>,
    pub solicited_flag: Option<SolicitedFlag>,
    pub side_compliance_id: Option<FIXString>
}

pub struct SpreadOrBenchmarkCurveData {
    pub spread: Option<PriceOffset>,
    pub benchmark_curve_currency: Option<Currency>,
    pub benchmark_curve_name: Option<FIXString>,
    pub benchmark_curve_point: Option<FIXString>,
    pub benchmark_price: Option<Price>,
    pub benchmark_price_type: Option<FIXInt>,
    pub benchmark_security_id: Option<FIXString>,
    pub benchmark_security_id_source: Option<FIXString>
}

pub struct Stipulations {
    pub stipulation_type: Option<StipulationType>,
    pub stipulation_value: Option<FIXString>
}

pub struct TrdAllocGrp {
    pub alloc_account: Option<FIXString>,
    pub alloc_acct_id_source: Option<FIXInt>,
    pub alloc_settl_currency: Option<Currency>,
    pub individual_alloc_id: Option<FIXString>,
    pub nested_parties2: Option<NestedParties2>,
    pub alloc_qty: Option<Qty>
}

pub struct TrdCapDtGrp {
    pub trade_date: Option<LocalMktDate>,
    pub transact_time: Option<UTCTimestamp>
}

pub struct TrdCapRptSideGrp {
    pub side: Side,
    pub order_id: FIXString,
    pub secondary_order_id: Option<FIXString>,
    pub cl_ord_id: Option<FIXString>,
    pub secondary_cl_ord_id: Option<FIXString>,
    pub list_id: Option<FIXString>,
    pub parties: Option<Parties>,
    pub account: Option<FIXString>,
    pub acct_id_source: Option<AcctIDSource>,
    pub account_type: Option<AccountType>,
    pub process_code: Option<ProcessCode>,
    pub odd_lot: Option<OddLot>,
    pub clr_inst_grp: Option<ClrInstGrp>,
    pub trade_input_source: Option<FIXString>,
    pub trade_input_device: Option<FIXString>,
    pub order_input_device: Option<FIXString>,
    pub currency: Option<Currency>,
    pub compliance_id: Option<FIXString>,
    pub solicited_flag: Option<SolicitedFlag>,
    pub order_capacity: Option<OrderCapacity>,
    pub order_restrictions: Option<OrderRestrictions>,
    pub cust_order_capacity: Option<CustOrderCapacity>,
    pub ord_type: Option<OrdType>,
    pub exec_inst: Option<ExecInst>,
    pub trans_bkd_time: Option<UTCTimestamp>,
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>,
    pub time_bracket: Option<FIXString>,
    pub commission_data: Option<CommissionData>,
    pub gross_trade_amt: Option<Amt>,
    pub num_days_interest: Option<FIXInt>,
    pub ex_date: Option<LocalMktDate>,
    pub accrued_interest_rate: Option<Percentage>,
    pub accrued_interest_amt: Option<Amt>,
    pub interest_at_maturity: Option<Amt>,
    pub end_accrued_interest_amt: Option<Amt>,
    pub start_cash: Option<Amt>,
    pub end_cash: Option<Amt>,
    pub concession: Option<Amt>,
    pub total_takedown: Option<Amt>,
    pub net_money: Option<Amt>,
    pub settl_curr_amt: Option<Amt>,
    pub settl_currency: Option<Currency>,
    pub settl_curr_fx_rate: Option<FIXFloat>,
    pub settl_curr_fx_rate_calc: Option<SettlCurrFxRateCalc>,
    pub position_effect: Option<PositionEffect>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>,
    pub side_multi_leg_reporting_type: Option<SideMultiLegReportingType>,
    pub cont_amt_grp: Option<ContAmtGrp>,
    pub stipulations: Option<Stipulations>,
    pub misc_fees_grp: Option<MiscFeesGrp>,
    pub exchange_rule: Option<FIXString>,
    pub trade_alloc_indicator: Option<TradeAllocIndicator>,
    pub prealloc_method: Option<PreallocMethod>,
    pub alloc_id: Option<FIXString>,
    pub trd_alloc_grp: Option<TrdAllocGrp>
}

pub struct TrdCollGrp {
    pub trade_report_id: Option<FIXString>,
    pub secondary_trade_report_id: Option<FIXString>
}

pub struct TrdInstrmtLegGrp {
    pub instrument_leg: Option<InstrumentLeg>,
    pub leg_qty: Option<Qty>,
    pub leg_swap_type: Option<LegSwapType>,
    pub leg_stipulations: Option<LegStipulations>,
    pub leg_position_effect: Option<FIXChar>,
    pub leg_covered_or_uncovered: Option<FIXInt>,
    pub nested_parties: Option<NestedParties>,
    pub leg_ref_id: Option<FIXString>,
    pub leg_price: Option<Price>,
    pub leg_settl_type: Option<FIXChar>,
    pub leg_settl_date: Option<LocalMktDate>,
    pub leg_last_px: Option<Price>
}

pub struct TrdRegTimestamps {
    pub trd_reg_timestamp: Option<UTCTimestamp>,
    pub trd_reg_timestamp_type: Option<TrdRegTimestampType>,
    pub trd_reg_timestamp_origin: Option<FIXString>
}

pub struct TrdgSesGrp {
    pub trading_session_id: Option<FIXString>,
    pub trading_session_sub_id: Option<FIXString>
}

pub struct UndInstrmtCollGrp {
    pub underlying_instrument: Option<UnderlyingInstrument>,
    pub coll_action: Option<CollAction>
}

pub struct UndInstrmtGrp {
    pub underlying_instrument: Option<UnderlyingInstrument>
}

pub struct UndInstrmtStrkPxGrp {
    pub underlying_instrument: Option<UnderlyingInstrument>,
    pub prev_close_px: Option<Price>,
    pub cl_ord_id: Option<FIXString>,
    pub secondary_cl_ord_id: Option<FIXString>,
    pub side: Option<Side>,
    pub price: Price,
    pub currency: Option<Currency>,
    pub text: Option<FIXString>,
    pub encoded_text: Option<Data>
}

pub struct UndSecAltIDGrp {
    pub underlying_security_alt_id: Option<FIXString>,
    pub underlying_security_alt_id_source: Option<FIXString>
}

pub struct UnderlyingInstrument {
    pub underlying_symbol: Option<FIXString>,
    pub underlying_symbol_sfx: Option<FIXString>,
    pub underlying_security_id: Option<FIXString>,
    pub underlying_security_id_source: Option<FIXString>,
    pub und_sec_alt_id_grp: Option<UndSecAltIDGrp>,
    pub underlying_product: Option<FIXInt>,
    pub underlying_cfi_code: Option<FIXString>,
    pub underlying_security_type: Option<FIXString>,
    pub underlying_security_sub_type: Option<FIXString>,
    pub underlying_maturity_month_year: Option<MonthYear>,
    pub underlying_maturity_date: Option<LocalMktDate>,
    pub underlying_put_or_call: Option<FIXInt>,
    pub underlying_coupon_payment_date: Option<LocalMktDate>,
    pub underlying_issue_date: Option<LocalMktDate>,
    pub underlying_repo_collateral_security_type: Option<FIXString>,
    pub underlying_repurchase_term: Option<FIXInt>,
    pub underlying_repurchase_rate: Option<Percentage>,
    pub underlying_factor: Option<FIXFloat>,
    pub underlying_credit_rating: Option<FIXString>,
    pub underlying_instr_registry: Option<FIXString>,
    pub underlying_country_of_issue: Option<Country>,
    pub underlying_state_or_province_of_issue: Option<FIXString>,
    pub underlying_locale_of_issue: Option<FIXString>,
    pub underlying_redemption_date: Option<LocalMktDate>,
    pub underlying_strike_price: Option<Price>,
    pub underlying_strike_currency: Option<Currency>,
    pub underlying_opt_attribute: Option<FIXChar>,
    pub underlying_contract_multiplier: Option<FIXFloat>,
    pub underlying_coupon_rate: Option<Percentage>,
    pub underlying_security_exchange: Option<MICExchange>,
    pub underlying_issuer: Option<FIXString>,
    pub encoded_underlying_issuer: Option<Data>,
    pub underlying_security_desc: Option<FIXString>,
    pub encoded_underlying_security_desc: Option<Data>,
    pub underlying_cp_program: Option<FIXString>,
    pub underlying_cp_reg_type: Option<FIXString>,
    pub underlying_currency: Option<Currency>,
    pub underlying_qty: Option<Qty>,
    pub underlying_px: Option<Price>,
    pub underlying_dirty_price: Option<Price>,
    pub underlying_end_price: Option<Price>,
    pub underlying_start_value: Option<Amt>,
    pub underlying_current_value: Option<Amt>,
    pub underlying_end_value: Option<Amt>,
    pub underlying_stipulations: Option<UnderlyingStipulations>
}

pub struct UnderlyingStipulations {
    pub underlying_stip_type: Option<FIXString>,
    pub underlying_stip_value: Option<FIXString>
}

pub struct YieldData {
    pub yield_type: Option<YieldType>,
    pub yield_: Option<Percentage>,
    pub yield_calc_date: Option<LocalMktDate>,
    pub yield_redemption_date: Option<LocalMktDate>,
    pub yield_redemption_price: Option<Price>,
    pub yield_redemption_price_type: Option<FIXInt>
}
