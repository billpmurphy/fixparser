use types::*;
use protocol::FIXValue;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
    HouseTrader = b'3' as isize,
    AccountIsHouseTraderAndIsCrossMargined = b'7' as isize,
    AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined = b'6' as isize,
    FloorTrader = b'4' as isize,
    AccountIsCarriedOnNonCustomerSideOfBooks = b'2' as isize,
    AccountIsCarriedOnCustomerSideOfBooks = b'1' as isize,
    JointBackofficeAccount = b'8' as isize,
}

impl FIXValue for AccountType {
    fn from_bytes(bytes: &[u8]) -> Option<AccountType> {
        match bytes {
            b"3" => Some(AccountType::HouseTrader),
            b"7" => Some(AccountType::AccountIsHouseTraderAndIsCrossMargined),
            b"6" => Some(AccountType::AccountIsCarriedOnNonCustomerSideOfBooksAndIsCrossMargined),
            b"4" => Some(AccountType::FloorTrader),
            b"2" => Some(AccountType::AccountIsCarriedOnNonCustomerSideOfBooks),
            b"1" => Some(AccountType::AccountIsCarriedOnCustomerSideOfBooks),
            b"8" => Some(AccountType::JointBackofficeAccount),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adjustment {
    Cancel = b'1' as isize,
    Error = b'2' as isize,
    Correction = b'3' as isize,
}

impl FIXValue for Adjustment {
    fn from_bytes(bytes: &[u8]) -> Option<Adjustment> {
        match bytes {
            b"1" => Some(Adjustment::Cancel),
            b"2" => Some(Adjustment::Error),
            b"3" => Some(Adjustment::Correction),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvSide {
    Buy = b'B' as isize,
    Sell = b'S' as isize,
    Trade = b'T' as isize,
    Cross = b'X' as isize,
}

impl FIXValue for AdvSide {
    fn from_bytes(bytes: &[u8]) -> Option<AdvSide> {
        match bytes {
            b"B" => Some(AdvSide::Buy),
            b"S" => Some(AdvSide::Sell),
            b"T" => Some(AdvSide::Trade),
            b"X" => Some(AdvSide::Cross),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvTransType {
    Cancel = b'C' as isize,
    New = b'N' as isize,
    Replace = b'R' as isize,
}

impl FIXValue for AdvTransType {
    fn from_bytes(bytes: &[u8]) -> Option<AdvTransType> {
        match bytes {
            b"C" => Some(AdvTransType::Cancel),
            b"N" => Some(AdvTransType::New),
            b"R" => Some(AdvTransType::Replace),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregatedBook {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for AggregatedBook {
    fn from_bytes(bytes: &[u8]) -> Option<AggregatedBook> {
        match bytes {
            b"N" => Some(AggregatedBook::No),
            b"Y" => Some(AggregatedBook::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocHandlInst {
    Match = b'1' as isize,
    Forward = b'2' as isize,
    ForwardAndMatch = b'3' as isize,
}

impl FIXValue for AllocHandlInst {
    fn from_bytes(bytes: &[u8]) -> Option<AllocHandlInst> {
        match bytes {
            b"1" => Some(AllocHandlInst::Match),
            b"2" => Some(AllocHandlInst::Forward),
            b"3" => Some(AllocHandlInst::ForwardAndMatch),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocLinkType {
    FXNetting = b'0' as isize,
    FXSwap = b'1' as isize,
}

impl FIXValue for AllocLinkType {
    fn from_bytes(bytes: &[u8]) -> Option<AllocLinkType> {
        match bytes {
            b"0" => Some(AllocLinkType::FXNetting),
            b"1" => Some(AllocLinkType::FXSwap),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasisPxType {
    ClosingPriceAtMorningSession = b'2' as isize,
    ClosingPrice = b'3' as isize,
    CurrentPrice = b'4' as isize,
    Sq = b'5' as isize,
    VwapThroughADay = b'6' as isize,
    VwapThroughAMorningSession = b'7' as isize,
    VwapThroughAnAfternoonSession = b'8' as isize,
    VwapThroughADayExceptYori = b'9' as isize,
    VwapThroughAMorningSessionExceptYori = b'A' as isize,
    VwapThroughAnAfternoonSessionExceptYori = b'B' as isize,
    Strike = b'C' as isize,
    Open = b'D' as isize,
    Others = b'Z' as isize,
}

impl FIXValue for BasisPxType {
    fn from_bytes(bytes: &[u8]) -> Option<BasisPxType> {
        match bytes {
            b"2" => Some(BasisPxType::ClosingPriceAtMorningSession),
            b"3" => Some(BasisPxType::ClosingPrice),
            b"4" => Some(BasisPxType::CurrentPrice),
            b"5" => Some(BasisPxType::Sq),
            b"6" => Some(BasisPxType::VwapThroughADay),
            b"7" => Some(BasisPxType::VwapThroughAMorningSession),
            b"8" => Some(BasisPxType::VwapThroughAnAfternoonSession),
            b"9" => Some(BasisPxType::VwapThroughADayExceptYori),
            b"A" => Some(BasisPxType::VwapThroughAMorningSessionExceptYori),
            b"B" => Some(BasisPxType::VwapThroughAnAfternoonSessionExceptYori),
            b"C" => Some(BasisPxType::Strike),
            b"D" => Some(BasisPxType::Open),
            b"Z" => Some(BasisPxType::Others),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Benchmark {
    Curve = b'1' as isize,
    FiveYr = b'2' as isize,
    Old5 = b'3' as isize,
    TenYr = b'4' as isize,
    Old10 = b'5' as isize,
    ThirtyYr = b'6' as isize,
    Old30 = b'7' as isize,
    ThreeMoLibor = b'8' as isize,
    SixMoLibor = b'9' as isize,
}

impl FIXValue for Benchmark {
    fn from_bytes(bytes: &[u8]) -> Option<Benchmark> {
        match bytes {
            b"1" => Some(Benchmark::Curve),
            b"2" => Some(Benchmark::FiveYr),
            b"3" => Some(Benchmark::Old5),
            b"4" => Some(Benchmark::TenYr),
            b"5" => Some(Benchmark::Old10),
            b"6" => Some(Benchmark::ThirtyYr),
            b"7" => Some(Benchmark::Old30),
            b"8" => Some(Benchmark::ThreeMoLibor),
            b"9" => Some(Benchmark::SixMoLibor),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookingUnit {
    AggregatePartialExecutionsOnThisOrderAndBookOneTradePerOrder = b'1' as isize,
    AggregateExecutionsForThisSymbolSideAndSettlementDate = b'2' as isize,
    EachPartialExecutionIsABookableUnit = b'0' as isize,
}

impl FIXValue for BookingUnit {
    fn from_bytes(bytes: &[u8]) -> Option<BookingUnit> {
        match bytes {
            b"1" => Some(BookingUnit::AggregatePartialExecutionsOnThisOrderAndBookOneTradePerOrder),
            b"2" => Some(BookingUnit::AggregateExecutionsForThisSymbolSideAndSettlementDate),
            b"0" => Some(BookingUnit::EachPartialExecutionIsABookableUnit),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancellationRights {
    NoWaiverAgreement = b'M' as isize,
    NoExecutionOnly = b'N' as isize,
    Yes = b'Y' as isize,
    NoInstitutional = b'O' as isize,
}

impl FIXValue for CancellationRights {
    fn from_bytes(bytes: &[u8]) -> Option<CancellationRights> {
        match bytes {
            b"M" => Some(CancellationRights::NoWaiverAgreement),
            b"N" => Some(CancellationRights::NoExecutionOnly),
            b"Y" => Some(CancellationRights::Yes),
            b"O" => Some(CancellationRights::NoInstitutional),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CashMargin {
    MarginOpen = b'2' as isize,
    MarginClose = b'3' as isize,
    Cash = b'1' as isize,
}

impl FIXValue for CashMargin {
    fn from_bytes(bytes: &[u8]) -> Option<CashMargin> {
        match bytes {
            b"2" => Some(CashMargin::MarginOpen),
            b"3" => Some(CashMargin::MarginClose),
            b"1" => Some(CashMargin::Cash),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContAmtType {
    CommissionAmount = b'1' as isize,
    Commission = b'2' as isize,
    InitialChargeAmount = b'3' as isize,
    InitialCharge = b'4' as isize,
    DiscountAmount = b'5' as isize,
    Discount = b'6' as isize,
    DilutionLevyAmount = b'7' as isize,
    DilutionLevy = b'8' as isize,
    ExitChargeAmount = b'9' as isize,
    ExitCharge,
    FundBasedRenewalCommission,
    ProjectedFundValue,
    FundBasedRenewalCommissionAmount13,
    FundBasedRenewalCommissionAmount14,
    NetSettlementAmount,
}

impl FIXValue for ContAmtType {
    fn from_bytes(bytes: &[u8]) -> Option<ContAmtType> {
        match bytes {
            b"1" => Some(ContAmtType::CommissionAmount),
            b"2" => Some(ContAmtType::Commission),
            b"3" => Some(ContAmtType::InitialChargeAmount),
            b"4" => Some(ContAmtType::InitialCharge),
            b"5" => Some(ContAmtType::DiscountAmount),
            b"6" => Some(ContAmtType::Discount),
            b"7" => Some(ContAmtType::DilutionLevyAmount),
            b"8" => Some(ContAmtType::DilutionLevy),
            b"9" => Some(ContAmtType::ExitChargeAmount),
            b"10" => Some(ContAmtType::ExitCharge),
            b"11" => Some(ContAmtType::FundBasedRenewalCommission),
            b"12" => Some(ContAmtType::ProjectedFundValue),
            b"13" => Some(ContAmtType::FundBasedRenewalCommissionAmount13),
            b"14" => Some(ContAmtType::FundBasedRenewalCommissionAmount14),
            b"15" => Some(ContAmtType::NetSettlementAmount),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            ContAmtType::ExitCharge => v.extend(b"10"),
            ContAmtType::FundBasedRenewalCommission => v.extend(b"11"),
            ContAmtType::ProjectedFundValue => v.extend(b"12"),
            ContAmtType::FundBasedRenewalCommissionAmount13 => v.extend(b"13"),
            ContAmtType::FundBasedRenewalCommissionAmount14 => v.extend(b"14"),
            ContAmtType::NetSettlementAmount => v.extend(b"15"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorporateAction {
    ExDividend = b'A' as isize,
    ExDistribution = b'B' as isize,
    ExRights = b'C' as isize,
    New = b'D' as isize,
    ExInterest = b'E' as isize,
}

impl FIXValue for CorporateAction {
    fn from_bytes(bytes: &[u8]) -> Option<CorporateAction> {
        match bytes {
            b"A" => Some(CorporateAction::ExDividend),
            b"B" => Some(CorporateAction::ExDistribution),
            b"C" => Some(CorporateAction::ExRights),
            b"D" => Some(CorporateAction::New),
            b"E" => Some(CorporateAction::ExInterest),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoveredOrUncovered {
    Covered = b'0' as isize,
    Uncovered = b'1' as isize,
}

impl FIXValue for CoveredOrUncovered {
    fn from_bytes(bytes: &[u8]) -> Option<CoveredOrUncovered> {
        match bytes {
            b"0" => Some(CoveredOrUncovered::Covered),
            b"1" => Some(CoveredOrUncovered::Uncovered),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomerOrFirm {
    Customer = b'0' as isize,
    Firm = b'1' as isize,
}

impl FIXValue for CustomerOrFirm {
    fn from_bytes(bytes: &[u8]) -> Option<CustomerOrFirm> {
        match bytes {
            b"0" => Some(CustomerOrFirm::Customer),
            b"1" => Some(CustomerOrFirm::Firm),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CxlRejResponseTo {
    OrderCancelRequest = b'1' as isize,
    OrderCancelReplaceRequest = b'2' as isize,
}

impl FIXValue for CxlRejResponseTo {
    fn from_bytes(bytes: &[u8]) -> Option<CxlRejResponseTo> {
        match bytes {
            b"1" => Some(CxlRejResponseTo::OrderCancelRequest),
            b"2" => Some(CxlRejResponseTo::OrderCancelReplaceRequest),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteReason {
    Cancelation = b'0' as isize,
    Error = b'1' as isize,
}

impl FIXValue for DeleteReason {
    fn from_bytes(bytes: &[u8]) -> Option<DeleteReason> {
        match bytes {
            b"0" => Some(DeleteReason::Cancelation),
            b"1" => Some(DeleteReason::Error),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DueToRelated {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for DueToRelated {
    fn from_bytes(bytes: &[u8]) -> Option<DueToRelated> {
        match bytes {
            b"N" => Some(DueToRelated::No),
            b"Y" => Some(DueToRelated::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailType {
    New = b'0' as isize,
    Reply = b'1' as isize,
    AdminReply = b'2' as isize,
}

impl FIXValue for EmailType {
    fn from_bytes(bytes: &[u8]) -> Option<EmailType> {
        match bytes {
            b"0" => Some(EmailType::New),
            b"1" => Some(EmailType::Reply),
            b"2" => Some(EmailType::AdminReply),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptMethod {
    None = b'0' as isize,
    Pkcs = b'1' as isize,
    Des = b'2' as isize,
    PkcsDes = b'3' as isize,
    PgpDes = b'4' as isize,
    PgpDesMd5 = b'5' as isize,
    PemDesMd5 = b'6' as isize,
}

impl FIXValue for EncryptMethod {
    fn from_bytes(bytes: &[u8]) -> Option<EncryptMethod> {
        match bytes {
            b"0" => Some(EncryptMethod::None),
            b"1" => Some(EncryptMethod::Pkcs),
            b"2" => Some(EncryptMethod::Des),
            b"3" => Some(EncryptMethod::PkcsDes),
            b"4" => Some(EncryptMethod::PgpDes),
            b"5" => Some(EncryptMethod::PgpDesMd5),
            b"6" => Some(EncryptMethod::PemDesMd5),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeForPhysical {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for ExchangeForPhysical {
    fn from_bytes(bytes: &[u8]) -> Option<ExchangeForPhysical> {
        match bytes {
            b"N" => Some(ExchangeForPhysical::No),
            b"Y" => Some(ExchangeForPhysical::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecPriceType {
    SinglePrice = b'S' as isize,
    OfferPriceMinusAdjustmentAmount = b'Q' as isize,
    OfferPriceMinusAdjustment = b'P' as isize,
    OfferPrice = b'O' as isize,
    CreationPricePlusAdjustmentAmount = b'E' as isize,
    CreationPricePlusAdjustment = b'D' as isize,
    CreationPrice = b'C' as isize,
    BidPrice = b'B' as isize,
}

impl FIXValue for ExecPriceType {
    fn from_bytes(bytes: &[u8]) -> Option<ExecPriceType> {
        match bytes {
            b"S" => Some(ExecPriceType::SinglePrice),
            b"Q" => Some(ExecPriceType::OfferPriceMinusAdjustmentAmount),
            b"P" => Some(ExecPriceType::OfferPriceMinusAdjustment),
            b"O" => Some(ExecPriceType::OfferPrice),
            b"E" => Some(ExecPriceType::CreationPricePlusAdjustmentAmount),
            b"D" => Some(ExecPriceType::CreationPricePlusAdjustment),
            b"C" => Some(ExecPriceType::CreationPrice),
            b"B" => Some(ExecPriceType::BidPrice),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecTransType {
    New = b'0' as isize,
    Cancel = b'1' as isize,
    Correct = b'2' as isize,
    Status = b'3' as isize,
}

impl FIXValue for ExecTransType {
    fn from_bytes(bytes: &[u8]) -> Option<ExecTransType> {
        match bytes {
            b"0" => Some(ExecTransType::New),
            b"1" => Some(ExecTransType::Cancel),
            b"2" => Some(ExecTransType::Correct),
            b"3" => Some(ExecTransType::Status),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForexReq {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for ForexReq {
    fn from_bytes(bytes: &[u8]) -> Option<ForexReq> {
        match bytes {
            b"N" => Some(ForexReq::No),
            b"Y" => Some(ForexReq::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FundRenewWaiv {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for FundRenewWaiv {
    fn from_bytes(bytes: &[u8]) -> Option<FundRenewWaiv> {
        match bytes {
            b"N" => Some(FundRenewWaiv::No),
            b"Y" => Some(FundRenewWaiv::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GTBookingInst {
    BookOutAllTradesOnDayOfExecution = b'0' as isize,
    AccumulateExecutionsUntilOrderIsFilledOrExpires = b'1' as isize,
    AccumulateUntilVerballyNotifiedOtherwise = b'2' as isize,
}

impl FIXValue for GTBookingInst {
    fn from_bytes(bytes: &[u8]) -> Option<GTBookingInst> {
        match bytes {
            b"0" => Some(GTBookingInst::BookOutAllTradesOnDayOfExecution),
            b"1" => Some(GTBookingInst::AccumulateExecutionsUntilOrderIsFilledOrExpires),
            b"2" => Some(GTBookingInst::AccumulateUntilVerballyNotifiedOtherwise),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapFillFlag {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for GapFillFlag {
    fn from_bytes(bytes: &[u8]) -> Option<GapFillFlag> {
        match bytes {
            b"N" => Some(GapFillFlag::No),
            b"Y" => Some(GapFillFlag::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HaltReasonChar {
    NewsDissemination = b'D' as isize,
    OrderInflux = b'E' as isize,
    OrderImbalance = b'I' as isize,
    AdditionalInformation = b'M' as isize,
    NewsPending = b'P' as isize,
    EquipmentChangeover = b'X' as isize,
}

impl FIXValue for HaltReasonChar {
    fn from_bytes(bytes: &[u8]) -> Option<HaltReasonChar> {
        match bytes {
            b"D" => Some(HaltReasonChar::NewsDissemination),
            b"E" => Some(HaltReasonChar::OrderInflux),
            b"I" => Some(HaltReasonChar::OrderImbalance),
            b"M" => Some(HaltReasonChar::AdditionalInformation),
            b"P" => Some(HaltReasonChar::NewsPending),
            b"X" => Some(HaltReasonChar::EquipmentChangeover),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlInst {
    AutomatedExecutionOrderPrivateNoBrokerIntervention = b'1' as isize,
    AutomatedExecutionOrderPublicBrokerInterventionOk = b'2' as isize,
    ManualOrderBestExecution = b'3' as isize,
}

impl FIXValue for HandlInst {
    fn from_bytes(bytes: &[u8]) -> Option<HandlInst> {
        match bytes {
            b"1" => Some(HandlInst::AutomatedExecutionOrderPrivateNoBrokerIntervention),
            b"2" => Some(HandlInst::AutomatedExecutionOrderPublicBrokerInterventionOk),
            b"3" => Some(HandlInst::ManualOrderBestExecution),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOINaturalFlag {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for IOINaturalFlag {
    fn from_bytes(bytes: &[u8]) -> Option<IOINaturalFlag> {
        match bytes {
            b"N" => Some(IOINaturalFlag::No),
            b"Y" => Some(IOINaturalFlag::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOIQltyInd {
    High = b'H' as isize,
    Low = b'L' as isize,
    Medium = b'M' as isize,
}

impl FIXValue for IOIQltyInd {
    fn from_bytes(bytes: &[u8]) -> Option<IOIQltyInd> {
        match bytes {
            b"H" => Some(IOIQltyInd::High),
            b"L" => Some(IOIQltyInd::Low),
            b"M" => Some(IOIQltyInd::Medium),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOIQty {
    Large = b'L' as isize,
    Medium = b'M' as isize,
    Small = b'S' as isize,
}

impl FIXValue for IOIQty {
    fn from_bytes(bytes: &[u8]) -> Option<IOIQty> {
        match bytes {
            b"L" => Some(IOIQty::Large),
            b"M" => Some(IOIQty::Medium),
            b"S" => Some(IOIQty::Small),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOIShares {
    Large = b'L' as isize,
    Medium = b'M' as isize,
    Small = b'S' as isize,
}

impl FIXValue for IOIShares {
    fn from_bytes(bytes: &[u8]) -> Option<IOIShares> {
        match bytes {
            b"L" => Some(IOIShares::Large),
            b"M" => Some(IOIShares::Medium),
            b"S" => Some(IOIShares::Small),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOITransType {
    Cancel = b'C' as isize,
    New = b'N' as isize,
    Replace = b'R' as isize,
}

impl FIXValue for IOITransType {
    fn from_bytes(bytes: &[u8]) -> Option<IOITransType> {
        match bytes {
            b"C" => Some(IOITransType::Cancel),
            b"N" => Some(IOITransType::New),
            b"R" => Some(IOITransType::Replace),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InViewOfCommon {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for InViewOfCommon {
    fn from_bytes(bytes: &[u8]) -> Option<InViewOfCommon> {
        match bytes {
            b"N" => Some(InViewOfCommon::No),
            b"Y" => Some(InViewOfCommon::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncTaxInd {
    Net = b'1' as isize,
    Gross = b'2' as isize,
}

impl FIXValue for IncTaxInd {
    fn from_bytes(bytes: &[u8]) -> Option<IncTaxInd> {
        match bytes {
            b"1" => Some(IncTaxInd::Net),
            b"2" => Some(IncTaxInd::Gross),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastCapacity {
    Agent = b'1' as isize,
    CrossAsAgent = b'2' as isize,
    CrossAsPrincipal = b'3' as isize,
    Principal = b'4' as isize,
}

impl FIXValue for LastCapacity {
    fn from_bytes(bytes: &[u8]) -> Option<LastCapacity> {
        match bytes {
            b"1" => Some(LastCapacity::Agent),
            b"2" => Some(LastCapacity::CrossAsAgent),
            b"3" => Some(LastCapacity::CrossAsPrincipal),
            b"4" => Some(LastCapacity::Principal),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegalConfirm {
    Yes = b'Y' as isize,
    No = b'N' as isize,
}

impl FIXValue for LegalConfirm {
    fn from_bytes(bytes: &[u8]) -> Option<LegalConfirm> {
        match bytes {
            b"Y" => Some(LegalConfirm::Yes),
            b"N" => Some(LegalConfirm::No),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocateReqd {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for LocateReqd {
    fn from_bytes(bytes: &[u8]) -> Option<LocateReqd> {
        match bytes {
            b"N" => Some(LocateReqd::No),
            b"Y" => Some(LocateReqd::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MDImplicitDelete {
    Yes = b'Y' as isize,
    No = b'N' as isize,
}

impl FIXValue for MDImplicitDelete {
    fn from_bytes(bytes: &[u8]) -> Option<MDImplicitDelete> {
        match bytes {
            b"Y" => Some(MDImplicitDelete::Yes),
            b"N" => Some(MDImplicitDelete::No),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MDUpdateAction {
    New = b'0' as isize,
    Change = b'1' as isize,
    Delete = b'2' as isize,
}

impl FIXValue for MDUpdateAction {
    fn from_bytes(bytes: &[u8]) -> Option<MDUpdateAction> {
        match bytes {
            b"0" => Some(MDUpdateAction::New),
            b"1" => Some(MDUpdateAction::Change),
            b"2" => Some(MDUpdateAction::Delete),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MDUpdateType {
    FullRefresh = b'0' as isize,
    IncrementalRefresh = b'1' as isize,
}

impl FIXValue for MDUpdateType {
    fn from_bytes(bytes: &[u8]) -> Option<MDUpdateType> {
        match bytes {
            b"0" => Some(MDUpdateType::FullRefresh),
            b"1" => Some(MDUpdateType::IncrementalRefresh),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassCancelRequestType {
    CancelOrdersForASecurity = b'1' as isize,
    CancelAllOrders = b'7' as isize,
    CancelOrdersForATradingSession = b'6' as isize,
    CancelOrdersForASecuritytype = b'5' as isize,
    CancelOrdersForACficode = b'4' as isize,
    CancelOrdersForAnUnderlyingSecurity = b'2' as isize,
    CancelOrdersForAProduct = b'3' as isize,
}

impl FIXValue for MassCancelRequestType {
    fn from_bytes(bytes: &[u8]) -> Option<MassCancelRequestType> {
        match bytes {
            b"1" => Some(MassCancelRequestType::CancelOrdersForASecurity),
            b"7" => Some(MassCancelRequestType::CancelAllOrders),
            b"6" => Some(MassCancelRequestType::CancelOrdersForATradingSession),
            b"5" => Some(MassCancelRequestType::CancelOrdersForASecuritytype),
            b"4" => Some(MassCancelRequestType::CancelOrdersForACficode),
            b"2" => Some(MassCancelRequestType::CancelOrdersForAnUnderlyingSecurity),
            b"3" => Some(MassCancelRequestType::CancelOrdersForAProduct),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassCancelResponse {
    CancelOrdersForATradingSession = b'6' as isize,
    CancelRequestRejected = b'0' as isize,
    CancelAllOrders = b'7' as isize,
    CancelOrdersForAProduct = b'3' as isize,
    CancelOrdersForASecuritytype = b'5' as isize,
    CancelOrdersForACficode = b'4' as isize,
    CancelOrdersForASecurity = b'1' as isize,
    CancelOrdersForAnUnderlyingSecurity = b'2' as isize,
}

impl FIXValue for MassCancelResponse {
    fn from_bytes(bytes: &[u8]) -> Option<MassCancelResponse> {
        match bytes {
            b"6" => Some(MassCancelResponse::CancelOrdersForATradingSession),
            b"0" => Some(MassCancelResponse::CancelRequestRejected),
            b"7" => Some(MassCancelResponse::CancelAllOrders),
            b"3" => Some(MassCancelResponse::CancelOrdersForAProduct),
            b"5" => Some(MassCancelResponse::CancelOrdersForASecuritytype),
            b"4" => Some(MassCancelResponse::CancelOrdersForACficode),
            b"1" => Some(MassCancelResponse::CancelOrdersForASecurity),
            b"2" => Some(MassCancelResponse::CancelOrdersForAnUnderlyingSecurity),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassStatusReqType {
    StatusForOrdersForASecurity = b'1' as isize,
    StatusForOrdersForAnUnderlyingSecurity = b'2' as isize,
    StatusForOrdersForAProduct = b'3' as isize,
    StatusForOrdersForACficode = b'4' as isize,
    StatusForOrdersForASecuritytype = b'5' as isize,
    StatusForOrdersForATradingSession = b'6' as isize,
    StatusForOrdersForAPartyid = b'8' as isize,
    StatusForAllOrders = b'7' as isize,
}

impl FIXValue for MassStatusReqType {
    fn from_bytes(bytes: &[u8]) -> Option<MassStatusReqType> {
        match bytes {
            b"1" => Some(MassStatusReqType::StatusForOrdersForASecurity),
            b"2" => Some(MassStatusReqType::StatusForOrdersForAnUnderlyingSecurity),
            b"3" => Some(MassStatusReqType::StatusForOrdersForAProduct),
            b"4" => Some(MassStatusReqType::StatusForOrdersForACficode),
            b"5" => Some(MassStatusReqType::StatusForOrdersForASecuritytype),
            b"6" => Some(MassStatusReqType::StatusForOrdersForATradingSession),
            b"8" => Some(MassStatusReqType::StatusForOrdersForAPartyid),
            b"7" => Some(MassStatusReqType::StatusForAllOrders),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchStatus {
    ComparedMatchedOrAffirmed = b'0' as isize,
    UncomparedUnmatchedOrUnaffirmed = b'1' as isize,
    AdvisoryOrAlert = b'2' as isize,
}

impl FIXValue for MatchStatus {
    fn from_bytes(bytes: &[u8]) -> Option<MatchStatus> {
        match bytes {
            b"0" => Some(MatchStatus::ComparedMatchedOrAffirmed),
            b"1" => Some(MatchStatus::UncomparedUnmatchedOrUnaffirmed),
            b"2" => Some(MatchStatus::AdvisoryOrAlert),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoneyLaunderingStatus {
    ExemptAuthorisedCreditOrFinancialInstitution = b'3' as isize,
    ExemptClientMoneyTypeExemption = b'2' as isize,
    ExemptBelowTheLimit = b'1' as isize,
    Passed = b'Y' as isize,
    NotChecked = b'N' as isize,
}

impl FIXValue for MoneyLaunderingStatus {
    fn from_bytes(bytes: &[u8]) -> Option<MoneyLaunderingStatus> {
        match bytes {
            b"3" => Some(MoneyLaunderingStatus::ExemptAuthorisedCreditOrFinancialInstitution),
            b"2" => Some(MoneyLaunderingStatus::ExemptClientMoneyTypeExemption),
            b"1" => Some(MoneyLaunderingStatus::ExemptBelowTheLimit),
            b"Y" => Some(MoneyLaunderingStatus::Passed),
            b"N" => Some(MoneyLaunderingStatus::NotChecked),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgDirection {
    Receive = b'R' as isize,
    Send = b'S' as isize,
}

impl FIXValue for MsgDirection {
    fn from_bytes(bytes: &[u8]) -> Option<MsgDirection> {
        match bytes {
            b"R" => Some(MsgDirection::Receive),
            b"S" => Some(MsgDirection::Send),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiLegReportingType {
    SingleSecurity = b'1' as isize,
    IndividualLegOfAMultiLegSecurity = b'2' as isize,
    MultiLegSecurity = b'3' as isize,
}

impl FIXValue for MultiLegReportingType {
    fn from_bytes(bytes: &[u8]) -> Option<MultiLegReportingType> {
        match bytes {
            b"1" => Some(MultiLegReportingType::SingleSecurity),
            b"2" => Some(MultiLegReportingType::IndividualLegOfAMultiLegSecurity),
            b"3" => Some(MultiLegReportingType::MultiLegSecurity),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetGrossInd {
    Net = b'1' as isize,
    Gross = b'2' as isize,
}

impl FIXValue for NetGrossInd {
    fn from_bytes(bytes: &[u8]) -> Option<NetGrossInd> {
        match bytes {
            b"1" => Some(NetGrossInd::Net),
            b"2" => Some(NetGrossInd::Gross),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoSides {
    OneSide = b'1' as isize,
    BothSides = b'2' as isize,
}

impl FIXValue for NoSides {
    fn from_bytes(bytes: &[u8]) -> Option<NoSides> {
        match bytes {
            b"1" => Some(NoSides::OneSide),
            b"2" => Some(NoSides::BothSides),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifyBrokerOfCredit {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for NotifyBrokerOfCredit {
    fn from_bytes(bytes: &[u8]) -> Option<NotifyBrokerOfCredit> {
        match bytes {
            b"N" => Some(NotifyBrokerOfCredit::No),
            b"Y" => Some(NotifyBrokerOfCredit::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderCapacity {
    RisklessPrincipal = b'R' as isize,
    Individual = b'I' as isize,
    Principal = b'P' as isize,
    AgentForOtherMember = b'W' as isize,
    Agency = b'A' as isize,
    Proprietary = b'G' as isize,
}

impl FIXValue for OrderCapacity {
    fn from_bytes(bytes: &[u8]) -> Option<OrderCapacity> {
        match bytes {
            b"R" => Some(OrderCapacity::RisklessPrincipal),
            b"I" => Some(OrderCapacity::Individual),
            b"P" => Some(OrderCapacity::Principal),
            b"W" => Some(OrderCapacity::AgentForOtherMember),
            b"A" => Some(OrderCapacity::Agency),
            b"G" => Some(OrderCapacity::Proprietary),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderRestrictions {
    ForeignEntity = b'7' as isize,
    RisklessArbitrage = b'A' as isize,
    ProgramTrade = b'1' as isize,
    ExternalMarketParticipant = b'8' as isize,
    ActingAsMarketMakerOrSpecialistInTheUnderlyingSecurityOfADerivativeSecurity = b'6' as isize,
    ActingAsMarketMakerOrSpecialistInTheSecurity = b'5' as isize,
    NonIndexArbitrage = b'3' as isize,
    IndexArbitrage = b'2' as isize,
    CompetingMarketMaker = b'4' as isize,
    ExternalInterConnectedMarketLinkage = b'9' as isize,
}

impl FIXValue for OrderRestrictions {
    fn from_bytes(bytes: &[u8]) -> Option<OrderRestrictions> {
        match bytes {
            b"7" => Some(OrderRestrictions::ForeignEntity),
            b"A" => Some(OrderRestrictions::RisklessArbitrage),
            b"1" => Some(OrderRestrictions::ProgramTrade),
            b"8" => Some(OrderRestrictions::ExternalMarketParticipant),
            b"6" => Some(OrderRestrictions::ActingAsMarketMakerOrSpecialistInTheUnderlyingSecurityOfADerivativeSecurity),
            b"5" => Some(OrderRestrictions::ActingAsMarketMakerOrSpecialistInTheSecurity),
            b"3" => Some(OrderRestrictions::NonIndexArbitrage),
            b"2" => Some(OrderRestrictions::IndexArbitrage),
            b"4" => Some(OrderRestrictions::CompetingMarketMaker),
            b"9" => Some(OrderRestrictions::ExternalInterConnectedMarketLinkage),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerType {
    IndividualInvestor = b'1' as isize,
    PublicCompany = b'2' as isize,
    PrivateCompany = b'3' as isize,
    IndividualTrustee = b'4' as isize,
    CompanyTrustee = b'5' as isize,
    PensionPlan = b'6' as isize,
    CustodianUnderGiftsToMinorsAct = b'7' as isize,
    Trusts = b'8' as isize,
    Fiduciaries = b'9' as isize,
    NetworkingSubAccount,
    NonProfitOrganization,
    CorporateBody,
    Nominee,
}

impl FIXValue for OwnerType {
    fn from_bytes(bytes: &[u8]) -> Option<OwnerType> {
        match bytes {
            b"1" => Some(OwnerType::IndividualInvestor),
            b"2" => Some(OwnerType::PublicCompany),
            b"3" => Some(OwnerType::PrivateCompany),
            b"4" => Some(OwnerType::IndividualTrustee),
            b"5" => Some(OwnerType::CompanyTrustee),
            b"6" => Some(OwnerType::PensionPlan),
            b"7" => Some(OwnerType::CustodianUnderGiftsToMinorsAct),
            b"8" => Some(OwnerType::Trusts),
            b"9" => Some(OwnerType::Fiduciaries),
            b"10" => Some(OwnerType::NetworkingSubAccount),
            b"11" => Some(OwnerType::NonProfitOrganization),
            b"12" => Some(OwnerType::CorporateBody),
            b"13" => Some(OwnerType::Nominee),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            OwnerType::NetworkingSubAccount => v.extend(b"10"),
            OwnerType::NonProfitOrganization => v.extend(b"11"),
            OwnerType::CorporateBody => v.extend(b"12"),
            OwnerType::Nominee => v.extend(b"13"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentMethod {
    Crest = b'1' as isize,
    Nscc = b'2' as isize,
    Euroclear = b'3' as isize,
    Clearstream = b'4' as isize,
    Cheque = b'5' as isize,
    TelegraphicTransfer = b'6' as isize,
    Fedwire = b'7' as isize,
    DebitCard = b'8' as isize,
    DirectDebit = b'9' as isize,
    DirectCredit,
    CreditCard,
    AchDebit,
    AchCredit,
    Bpay,
    HighValueClearingSystem,
}

impl FIXValue for PaymentMethod {
    fn from_bytes(bytes: &[u8]) -> Option<PaymentMethod> {
        match bytes {
            b"1" => Some(PaymentMethod::Crest),
            b"2" => Some(PaymentMethod::Nscc),
            b"3" => Some(PaymentMethod::Euroclear),
            b"4" => Some(PaymentMethod::Clearstream),
            b"5" => Some(PaymentMethod::Cheque),
            b"6" => Some(PaymentMethod::TelegraphicTransfer),
            b"7" => Some(PaymentMethod::Fedwire),
            b"8" => Some(PaymentMethod::DebitCard),
            b"9" => Some(PaymentMethod::DirectDebit),
            b"10" => Some(PaymentMethod::DirectCredit),
            b"11" => Some(PaymentMethod::CreditCard),
            b"12" => Some(PaymentMethod::AchDebit),
            b"13" => Some(PaymentMethod::AchCredit),
            b"14" => Some(PaymentMethod::Bpay),
            b"15" => Some(PaymentMethod::HighValueClearingSystem),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            PaymentMethod::DirectCredit => v.extend(b"10"),
            PaymentMethod::CreditCard => v.extend(b"11"),
            PaymentMethod::AchDebit => v.extend(b"12"),
            PaymentMethod::AchCredit => v.extend(b"13"),
            PaymentMethod::Bpay => v.extend(b"14"),
            PaymentMethod::HighValueClearingSystem => v.extend(b"15"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionEffect {
    Fifo = b'F' as isize,
    Rolled = b'R' as isize,
    Close = b'C' as isize,
    Open = b'O' as isize,
}

impl FIXValue for PositionEffect {
    fn from_bytes(bytes: &[u8]) -> Option<PositionEffect> {
        match bytes {
            b"F" => Some(PositionEffect::Fifo),
            b"R" => Some(PositionEffect::Rolled),
            b"C" => Some(PositionEffect::Close),
            b"O" => Some(PositionEffect::Open),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PossDupFlag {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for PossDupFlag {
    fn from_bytes(bytes: &[u8]) -> Option<PossDupFlag> {
        match bytes {
            b"N" => Some(PossDupFlag::No),
            b"Y" => Some(PossDupFlag::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreallocMethod {
    ProRata = b'0' as isize,
    DoNotProRataDiscussFirst = b'1' as isize,
}

impl FIXValue for PreallocMethod {
    fn from_bytes(bytes: &[u8]) -> Option<PreallocMethod> {
        match bytes {
            b"0" => Some(PreallocMethod::ProRata),
            b"1" => Some(PreallocMethod::DoNotProRataDiscussFirst),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviouslyReported {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for PreviouslyReported {
    fn from_bytes(bytes: &[u8]) -> Option<PreviouslyReported> {
        match bytes {
            b"N" => Some(PreviouslyReported::No),
            b"Y" => Some(PreviouslyReported::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityIndicator {
    PriorityUnchanged = b'0' as isize,
    LostPriorityAsResultOfOrderChange = b'1' as isize,
}

impl FIXValue for PriorityIndicator {
    fn from_bytes(bytes: &[u8]) -> Option<PriorityIndicator> {
        match bytes {
            b"0" => Some(PriorityIndicator::PriorityUnchanged),
            b"1" => Some(PriorityIndicator::LostPriorityAsResultOfOrderChange),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessCode {
    Regular = b'0' as isize,
    SoftDollar = b'1' as isize,
    StepIn = b'2' as isize,
    StepOut = b'3' as isize,
    SoftDollarStepIn = b'4' as isize,
    SoftDollarStepOut = b'5' as isize,
    PlanSponsor = b'6' as isize,
}

impl FIXValue for ProcessCode {
    fn from_bytes(bytes: &[u8]) -> Option<ProcessCode> {
        match bytes {
            b"0" => Some(ProcessCode::Regular),
            b"1" => Some(ProcessCode::SoftDollar),
            b"2" => Some(ProcessCode::StepIn),
            b"3" => Some(ProcessCode::StepOut),
            b"4" => Some(ProcessCode::SoftDollarStepIn),
            b"5" => Some(ProcessCode::SoftDollarStepOut),
            b"6" => Some(ProcessCode::PlanSponsor),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgRptReqs {
    BuysideExplicitlyRequestsStatusUsingStatusrequest = b'1' as isize,
    SellsidePeriodicallySendsStatusUsingListstatusPeriodOptionallySpecifiedInProgressperiod = b'2' as isize,
    RealTimeExecutionReports = b'3' as isize,
}

impl FIXValue for ProgRptReqs {
    fn from_bytes(bytes: &[u8]) -> Option<ProgRptReqs> {
        match bytes {
            b"1" => Some(ProgRptReqs::BuysideExplicitlyRequestsStatusUsingStatusrequest),
            b"2" => Some(ProgRptReqs::SellsidePeriodicallySendsStatusUsingListstatusPeriodOptionallySpecifiedInProgressperiod),
            b"3" => Some(ProgRptReqs::RealTimeExecutionReports),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PutOrCall {
    Put = b'0' as isize,
    Call = b'1' as isize,
}

impl FIXValue for PutOrCall {
    fn from_bytes(bytes: &[u8]) -> Option<PutOrCall> {
        match bytes {
            b"0" => Some(PutOrCall::Put),
            b"1" => Some(PutOrCall::Call),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteCondition {
    Open = b'A' as isize,
    Closed = b'B' as isize,
    ExchangeBest = b'C' as isize,
    ConsolidatedBest = b'D' as isize,
    Locked = b'E' as isize,
    Crossed = b'F' as isize,
    Depth = b'G' as isize,
    FastTrading = b'H' as isize,
    NonFirm = b'I' as isize,
}

impl FIXValue for QuoteCondition {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteCondition> {
        match bytes {
            b"A" => Some(QuoteCondition::Open),
            b"B" => Some(QuoteCondition::Closed),
            b"C" => Some(QuoteCondition::ExchangeBest),
            b"D" => Some(QuoteCondition::ConsolidatedBest),
            b"E" => Some(QuoteCondition::Locked),
            b"F" => Some(QuoteCondition::Crossed),
            b"G" => Some(QuoteCondition::Depth),
            b"H" => Some(QuoteCondition::FastTrading),
            b"I" => Some(QuoteCondition::NonFirm),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteRequestType {
    Manual = b'1' as isize,
    Automatic = b'2' as isize,
}

impl FIXValue for QuoteRequestType {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteRequestType> {
        match bytes {
            b"1" => Some(QuoteRequestType::Manual),
            b"2" => Some(QuoteRequestType::Automatic),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteResponseLevel {
    NoAcknowledgement = b'0' as isize,
    AcknowledgeOnlyNegativeOrErroneousQuotes = b'1' as isize,
    AcknowledgeEachQuoteMessages = b'2' as isize,
}

impl FIXValue for QuoteResponseLevel {
    fn from_bytes(bytes: &[u8]) -> Option<QuoteResponseLevel> {
        match bytes {
            b"0" => Some(QuoteResponseLevel::NoAcknowledgement),
            b"1" => Some(QuoteResponseLevel::AcknowledgeOnlyNegativeOrErroneousQuotes),
            b"2" => Some(QuoteResponseLevel::AcknowledgeEachQuoteMessages),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistTransType {
    Cancel = b'2' as isize,
    New = b'0' as isize,
    Replace = b'1' as isize,
}

impl FIXValue for RegistTransType {
    fn from_bytes(bytes: &[u8]) -> Option<RegistTransType> {
        match bytes {
            b"2" => Some(RegistTransType::Cancel),
            b"0" => Some(RegistTransType::New),
            b"1" => Some(RegistTransType::Replace),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportToExch {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for ReportToExch {
    fn from_bytes(bytes: &[u8]) -> Option<ReportToExch> {
        match bytes {
            b"N" => Some(ReportToExch::No),
            b"Y" => Some(ReportToExch::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetSeqNumFlag {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for ResetSeqNumFlag {
    fn from_bytes(bytes: &[u8]) -> Option<ResetSeqNumFlag> {
        match bytes {
            b"N" => Some(ResetSeqNumFlag::No),
            b"Y" => Some(ResetSeqNumFlag::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingDirection {
    RoundToNearest = b'0' as isize,
    RoundDown = b'1' as isize,
    RoundUp = b'2' as isize,
}

impl FIXValue for RoundingDirection {
    fn from_bytes(bytes: &[u8]) -> Option<RoundingDirection> {
        match bytes {
            b"0" => Some(RoundingDirection::RoundToNearest),
            b"1" => Some(RoundingDirection::RoundDown),
            b"2" => Some(RoundingDirection::RoundUp),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingType {
    TargetFirm = b'1' as isize,
    TargetList = b'2' as isize,
    BlockFirm = b'3' as isize,
    BlockList = b'4' as isize,
}

impl FIXValue for RoutingType {
    fn from_bytes(bytes: &[u8]) -> Option<RoutingType> {
        match bytes {
            b"1" => Some(RoutingType::TargetFirm),
            b"2" => Some(RoutingType::TargetList),
            b"3" => Some(RoutingType::BlockFirm),
            b"4" => Some(RoutingType::BlockList),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Local = b'1' as isize,
    National = b'2' as isize,
    Global = b'3' as isize,
}

impl FIXValue for Scope {
    fn from_bytes(bytes: &[u8]) -> Option<Scope> {
        match bytes {
            b"1" => Some(Scope::Local),
            b"2" => Some(Scope::National),
            b"3" => Some(Scope::Global),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityListRequestType {
    SecuritytypeAndOrCficode = b'1' as isize,
    Product = b'2' as isize,
    Tradingsessionid = b'3' as isize,
    AllSecurities = b'4' as isize,
    Symbol = b'0' as isize,
}

impl FIXValue for SecurityListRequestType {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityListRequestType> {
        match bytes {
            b"1" => Some(SecurityListRequestType::SecuritytypeAndOrCficode),
            b"2" => Some(SecurityListRequestType::Product),
            b"3" => Some(SecurityListRequestType::Tradingsessionid),
            b"4" => Some(SecurityListRequestType::AllSecurities),
            b"0" => Some(SecurityListRequestType::Symbol),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityRequestResult {
    InstrumentDataTemporarilyUnavailable = b'4' as isize,
    ValidRequest = b'0' as isize,
    InvalidOrUnsupportedRequest = b'1' as isize,
    RequestForInstrumentDataNotSupported = b'5' as isize,
    NotAuthorizedToRetrieveInstrumentData = b'3' as isize,
    NoInstrumentsFoundThatMatchSelectionCriteria = b'2' as isize,
}

impl FIXValue for SecurityRequestResult {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityRequestResult> {
        match bytes {
            b"4" => Some(SecurityRequestResult::InstrumentDataTemporarilyUnavailable),
            b"0" => Some(SecurityRequestResult::ValidRequest),
            b"1" => Some(SecurityRequestResult::InvalidOrUnsupportedRequest),
            b"5" => Some(SecurityRequestResult::RequestForInstrumentDataNotSupported),
            b"3" => Some(SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData),
            b"2" => Some(SecurityRequestResult::NoInstrumentsFoundThatMatchSelectionCriteria),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityRequestType {
    RequestSecurityIdentityAndSpecifications = b'0' as isize,
    RequestSecurityIdentityForTheSpecificationsProvided = b'1' as isize,
    RequestListSecurityTypes = b'2' as isize,
    RequestListSecurities = b'3' as isize,
}

impl FIXValue for SecurityRequestType {
    fn from_bytes(bytes: &[u8]) -> Option<SecurityRequestType> {
        match bytes {
            b"0" => Some(SecurityRequestType::RequestSecurityIdentityAndSpecifications),
            b"1" => Some(SecurityRequestType::RequestSecurityIdentityForTheSpecificationsProvided),
            b"2" => Some(SecurityRequestType::RequestListSecurityTypes),
            b"3" => Some(SecurityRequestType::RequestListSecurities),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlCurrFxRateCalc {
    Multiply = b'M' as isize,
    Divide = b'D' as isize,
}

impl FIXValue for SettlCurrFxRateCalc {
    fn from_bytes(bytes: &[u8]) -> Option<SettlCurrFxRateCalc> {
        match bytes {
            b"M" => Some(SettlCurrFxRateCalc::Multiply),
            b"D" => Some(SettlCurrFxRateCalc::Divide),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlLocation {
    Cedel,
    DepositoryTrustCompany,
    Euroclear,
    FederalBookEntry,
    LocalMarketSettleLocation,
    Physical,
    ParticipantTrustCompany,
}

impl FIXValue for SettlLocation {
    fn from_bytes(bytes: &[u8]) -> Option<SettlLocation> {
        match bytes {
            b"CED" => Some(SettlLocation::Cedel),
            b"DTC" => Some(SettlLocation::DepositoryTrustCompany),
            b"EUR" => Some(SettlLocation::Euroclear),
            b"FED" => Some(SettlLocation::FederalBookEntry),
            b"ISO Country Code" => Some(SettlLocation::LocalMarketSettleLocation),
            b"PNY" => Some(SettlLocation::Physical),
            b"PTC" => Some(SettlLocation::ParticipantTrustCompany),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            SettlLocation::Cedel => v.extend(b"CED"),
            SettlLocation::DepositoryTrustCompany => v.extend(b"DTC"),
            SettlLocation::Euroclear => v.extend(b"EUR"),
            SettlLocation::FederalBookEntry => v.extend(b"FED"),
            SettlLocation::LocalMarketSettleLocation => v.extend(b"ISO Country Code"),
            SettlLocation::Physical => v.extend(b"PNY"),
            SettlLocation::ParticipantTrustCompany => v.extend(b"PTC"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolicitedFlag {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for SolicitedFlag {
    fn from_bytes(bytes: &[u8]) -> Option<SolicitedFlag> {
        match bytes {
            b"N" => Some(SolicitedFlag::No),
            b"Y" => Some(SolicitedFlag::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionRequestType {
    Snapshot = b'0' as isize,
    SnapshotPlusUpdates = b'1' as isize,
    DisablePreviousSnapshotPlusUpdateRequest = b'2' as isize,
}

impl FIXValue for SubscriptionRequestType {
    fn from_bytes(bytes: &[u8]) -> Option<SubscriptionRequestType> {
        match bytes {
            b"0" => Some(SubscriptionRequestType::Snapshot),
            b"1" => Some(SubscriptionRequestType::SnapshotPlusUpdates),
            b"2" => Some(SubscriptionRequestType::DisablePreviousSnapshotPlusUpdateRequest),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxAdvantageType {
    NoneNotApplicable = b'0' as isize,
    MaxiIsa = b'1' as isize,
    Tessa = b'2' as isize,
    MiniCashIsa = b'3' as isize,
    MiniStocksAndSharesIsa = b'4' as isize,
    MiniInsuranceIsa = b'5' as isize,
    CurrentYearPayment = b'6' as isize,
    PriorYearPayment = b'7' as isize,
    AssetTransfer = b'8' as isize,
    Employee = b'9' as isize,
    EmployeeCurrentYear,
    Employer,
    EmployerCurrentYear,
    NonFundPrototypeIra,
    NonFundQualifiedPlan,
    DefinedContributionPlan,
    IndividualRetirementAccount,
    IndividualRetirementAccountRollover,
    Keogh,
    ProfitSharingPlan,
    Fourhundredandonek,
    SelfDirectedIra,
    Fourhundredandthree,
    Fourhundredandfiftyseven,
    RothIra24,
    RothIra25,
    RothConversionIra26,
    RothConversionIra27,
    EducationIra28,
    EducationIra29,
}

impl FIXValue for TaxAdvantageType {
    fn from_bytes(bytes: &[u8]) -> Option<TaxAdvantageType> {
        match bytes {
            b"0" => Some(TaxAdvantageType::NoneNotApplicable),
            b"1" => Some(TaxAdvantageType::MaxiIsa),
            b"2" => Some(TaxAdvantageType::Tessa),
            b"3" => Some(TaxAdvantageType::MiniCashIsa),
            b"4" => Some(TaxAdvantageType::MiniStocksAndSharesIsa),
            b"5" => Some(TaxAdvantageType::MiniInsuranceIsa),
            b"6" => Some(TaxAdvantageType::CurrentYearPayment),
            b"7" => Some(TaxAdvantageType::PriorYearPayment),
            b"8" => Some(TaxAdvantageType::AssetTransfer),
            b"9" => Some(TaxAdvantageType::Employee),
            b"10" => Some(TaxAdvantageType::EmployeeCurrentYear),
            b"11" => Some(TaxAdvantageType::Employer),
            b"12" => Some(TaxAdvantageType::EmployerCurrentYear),
            b"13" => Some(TaxAdvantageType::NonFundPrototypeIra),
            b"14" => Some(TaxAdvantageType::NonFundQualifiedPlan),
            b"15" => Some(TaxAdvantageType::DefinedContributionPlan),
            b"16" => Some(TaxAdvantageType::IndividualRetirementAccount),
            b"17" => Some(TaxAdvantageType::IndividualRetirementAccountRollover),
            b"18" => Some(TaxAdvantageType::Keogh),
            b"19" => Some(TaxAdvantageType::ProfitSharingPlan),
            b"20" => Some(TaxAdvantageType::Fourhundredandonek),
            b"21" => Some(TaxAdvantageType::SelfDirectedIra),
            b"22" => Some(TaxAdvantageType::Fourhundredandthree),
            b"23" => Some(TaxAdvantageType::Fourhundredandfiftyseven),
            b"24" => Some(TaxAdvantageType::RothIra24),
            b"25" => Some(TaxAdvantageType::RothIra25),
            b"26" => Some(TaxAdvantageType::RothConversionIra26),
            b"27" => Some(TaxAdvantageType::RothConversionIra27),
            b"28" => Some(TaxAdvantageType::EducationIra28),
            b"29" => Some(TaxAdvantageType::EducationIra29),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        match *self {
            TaxAdvantageType::EmployeeCurrentYear => v.extend(b"10"),
            TaxAdvantageType::Employer => v.extend(b"11"),
            TaxAdvantageType::EmployerCurrentYear => v.extend(b"12"),
            TaxAdvantageType::NonFundPrototypeIra => v.extend(b"13"),
            TaxAdvantageType::NonFundQualifiedPlan => v.extend(b"14"),
            TaxAdvantageType::DefinedContributionPlan => v.extend(b"15"),
            TaxAdvantageType::IndividualRetirementAccount => v.extend(b"16"),
            TaxAdvantageType::IndividualRetirementAccountRollover => v.extend(b"17"),
            TaxAdvantageType::Keogh => v.extend(b"18"),
            TaxAdvantageType::ProfitSharingPlan => v.extend(b"19"),
            TaxAdvantageType::Fourhundredandonek => v.extend(b"20"),
            TaxAdvantageType::SelfDirectedIra => v.extend(b"21"),
            TaxAdvantageType::Fourhundredandthree => v.extend(b"22"),
            TaxAdvantageType::Fourhundredandfiftyseven => v.extend(b"23"),
            TaxAdvantageType::RothIra24 => v.extend(b"24"),
            TaxAdvantageType::RothIra25 => v.extend(b"25"),
            TaxAdvantageType::RothConversionIra26 => v.extend(b"26"),
            TaxAdvantageType::RothConversionIra27 => v.extend(b"27"),
            TaxAdvantageType::EducationIra28 => v.extend(b"28"),
            TaxAdvantageType::EducationIra29 => v.extend(b"29"),
            _ => v.push(*self as u8)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestMessageIndicator {
    Yes = b'Y' as isize,
    No = b'N' as isize,
}

impl FIXValue for TestMessageIndicator {
    fn from_bytes(bytes: &[u8]) -> Option<TestMessageIndicator> {
        match bytes {
            b"Y" => Some(TestMessageIndicator::Yes),
            b"N" => Some(TestMessageIndicator::No),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickDirection {
    PlusTick = b'0' as isize,
    ZeroPlusTick = b'1' as isize,
    MinusTick = b'2' as isize,
    ZeroMinusTick = b'3' as isize,
}

impl FIXValue for TickDirection {
    fn from_bytes(bytes: &[u8]) -> Option<TickDirection> {
        match bytes {
            b"0" => Some(TickDirection::PlusTick),
            b"1" => Some(TickDirection::ZeroPlusTick),
            b"2" => Some(TickDirection::MinusTick),
            b"3" => Some(TickDirection::ZeroMinusTick),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradSesMethod {
    Electronic = b'1' as isize,
    OpenOutcry = b'2' as isize,
    TwoParty = b'3' as isize,
}

impl FIXValue for TradSesMethod {
    fn from_bytes(bytes: &[u8]) -> Option<TradSesMethod> {
        match bytes {
            b"1" => Some(TradSesMethod::Electronic),
            b"2" => Some(TradSesMethod::OpenOutcry),
            b"3" => Some(TradSesMethod::TwoParty),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradSesMode {
    Testing = b'1' as isize,
    Simulated = b'2' as isize,
    Production = b'3' as isize,
}

impl FIXValue for TradSesMode {
    fn from_bytes(bytes: &[u8]) -> Option<TradSesMode> {
        match bytes {
            b"1" => Some(TradSesMode::Testing),
            b"2" => Some(TradSesMode::Simulated),
            b"3" => Some(TradSesMode::Production),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeRequestType {
    AdvisoriesThatMatchCriteria = b'4' as isize,
    UnreportedTradesThatMatchCriteria = b'3' as isize,
    UnmatchedTradesThatMatchCriteria = b'2' as isize,
    MatchedTradesMatchingCriteriaProvidedOnRequest = b'1' as isize,
    AllTrades = b'0' as isize,
}

impl FIXValue for TradeRequestType {
    fn from_bytes(bytes: &[u8]) -> Option<TradeRequestType> {
        match bytes {
            b"4" => Some(TradeRequestType::AdvisoriesThatMatchCriteria),
            b"3" => Some(TradeRequestType::UnreportedTradesThatMatchCriteria),
            b"2" => Some(TradeRequestType::UnmatchedTradesThatMatchCriteria),
            b"1" => Some(TradeRequestType::MatchedTradesMatchingCriteriaProvidedOnRequest),
            b"0" => Some(TradeRequestType::AllTrades),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeType {
    Agency = b'A' as isize,
    VwapGuarantee = b'G' as isize,
    GuaranteedClose = b'J' as isize,
    RiskTrade = b'R' as isize,
}

impl FIXValue for TradeType {
    fn from_bytes(bytes: &[u8]) -> Option<TradeType> {
        match bytes {
            b"A" => Some(TradeType::Agency),
            b"G" => Some(TradeType::VwapGuarantee),
            b"J" => Some(TradeType::GuaranteedClose),
            b"R" => Some(TradeType::RiskTrade),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradedFlatSwitch {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for TradedFlatSwitch {
    fn from_bytes(bytes: &[u8]) -> Option<TradedFlatSwitch> {
        match bytes {
            b"N" => Some(TradedFlatSwitch::No),
            b"Y" => Some(TradedFlatSwitch::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsolicitedIndicator {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for UnsolicitedIndicator {
    fn from_bytes(bytes: &[u8]) -> Option<UnsolicitedIndicator> {
        match bytes {
            b"N" => Some(UnsolicitedIndicator::No),
            b"Y" => Some(UnsolicitedIndicator::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Urgency {
    Normal = b'0' as isize,
    Flash = b'1' as isize,
    Background = b'2' as isize,
}

impl FIXValue for Urgency {
    fn from_bytes(bytes: &[u8]) -> Option<Urgency> {
        match bytes {
            b"0" => Some(Urgency::Normal),
            b"1" => Some(Urgency::Flash),
            b"2" => Some(Urgency::Background),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkingIndicator {
    No = b'N' as isize,
    Yes = b'Y' as isize,
}

impl FIXValue for WorkingIndicator {
    fn from_bytes(bytes: &[u8]) -> Option<WorkingIndicator> {
        match bytes {
            b"N" => Some(WorkingIndicator::No),
            b"Y" => Some(WorkingIndicator::Yes),
            _ => None
        }
    }

    fn to_bytes(&self, mut v: &mut Vec<u8>) {
        v.push(*self as u8);
    }
}

pub struct OrderQtyData {
    pub order_qty: Option<Qty>,
    pub cash_order_qty: Option<Qty>,
    pub order_percent: Option<Percentage>,
    pub rounding_direction: Option<RoundingDirection>,
    pub rounding_modulus: Option<FIXFloat>
}
