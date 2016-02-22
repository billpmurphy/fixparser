use protocol::FIXValue;


/// Reuters Exchange Mnemonics representing a market or exchange.
///
/// Used in FIX Protocol Version 4.2.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReutersExchange {
    /// Abidjan Stock Exchange
    CI,
    /// American Stock Exchange
    A,
    /// Amman Stock Exchange
    AM,
    /// AEX Options and Futures Exchange
    E,
    /// AEX Stock Exchange
    AS,
    /// Australian Stock Exchange
    AX,
    /// Bahrain Stock Exchange
    BH,
    /// Barcelona Stock Exchange - Floor Trading
    BC,
    /// Barcelona Stock Exchange - CATS Feed
    MC,
    /// Beirut Stock Exchange
    BY,
    /// Belfox
    b,
    /// Berlin Stock Exchange
    BE,
    /// Berne Stock Exchange
    BN,
    /// Bilbao Stock Exchange
    BI,
    /// Bombay Stock Exchange
    BO,
    /// Boston Stock Exchange
    B,
    /// Botswana Share Market
    BT,
    /// Bremen Stock Exchange
    BM,
    /// Brussels Stock Exchange
    BR,
    /// Calcutta Stock Exchange
    CL,
    /// Canadian Ventures Exchange
    V,
    /// Channel Islands
    CH,
    /// Chicago Board Options Exchange
    W,
    /// Chicago Stock Exchange
    MW,
    /// Chile Electronic Exchange
    CE,
    /// Cincinnati Stock Exchange
    C,
    /// Colombo Stock Exchange
    CM,
    /// Copenhagen Stock Exchange
    CO,
    /// Dehli Stock Exchange
    DL,
    /// Doha Securities Market
    QA,
    /// Dubai Financial Market
    DU,
    /// Dusseldorf Stock Exchange
    D,
    /// Electronic Stock Exchange of Venezuela
    EB,
    /// Eurex Germany (DTB)
    d,
    /// Eurex Switzerland (SFF)
    Z,
    /// Frankfurt Stock Exchange
    F,
    /// Fukuoka Stock Exchange
    FU,
    /// Ghana Stock Exchange
    GH,
    /// Hamburg Stock Exchange
    H,
    /// Hanover Stock Exchange
    HA,
    /// Helsinki Stock Exchange
    HE,
    /// Hong Kong Stock Exchange
    HK,
    /// Iceland Stock Exchange
    IC,
    /// Interbolsa (Portugal)
    IN,
    /// Irish Stock Exchange
    I,
    /// Istanbul Stock Exchange
    IS,
    /// Jakarta Stock Exchange
    JK,
    /// Japanese Securities Dealers Association (JASDAQ)
    Q,
    /// Johannesburg Stock Exchange
    J,
    /// Karachi Stock Exchange
    KA,
    /// KASDAQ (Korea)
    KQ,
    /// Kazakhstan Stock Exchange
    KZ,
    /// Korea Stock Exchange
    KS,
    /// Kuala Lumpur Stock Exchange
    KL,
    /// Kuwait Stock Exchange
    KW,
    /// Kyoto Stock Exchange
    KY,
    /// Lagos Stock Exchange
    LG,
    /// Latin American Market in Spain (LATIBEX)
    LA,
    /// Le Nouveau Marche
    LN,
    /// Lima Stock Exchange
    LM,
    /// Lisbon Stock Exchange (Portugal)
    LS,
    /// London Stock Exchange
    L,
    /// Lusaka Stock Exchange
    LZ,
    /// Luxembourg Stock Exchange
    LU,
    /// Madras Stock Exchange
    MD,
    /// Madrid Stock Exchange – Floor Trading
    MA,
    /// Malta Stock Exchange
    MT,
    /// Mauritius Stock Exchange
    MZ,
    /// Medellin Stock Excahnge
    ML,
    /// Mexican Stock Exchange
    MX,
    /// Milan Stock Exchange
    MI,
    /// MONEP Paris Stock Options
    p,
    /// Montreal Exchange
    M,
    /// Moscow Inter Bank Currency Exchange
    MM,
    /// Moscow Stock Exchange
    MO,
    /// Munich Stock Exchange
    MU,
    /// Muscat Stock Exchange
    OM,
    /// Namibia Stock Exchange
    NM,
    /// Nagoya Stock Exchange
    NG,
    /// Nairobi Stock Exchange
    NR,
    /// NASDAQ
    O,
    /// NASDAQ Dealers – Bulletin Board
    OB,
    /// NASDAQ Japan
    OJ,
    /// National Stock Exchange of India
    NS,
    /// New York Stock Exchange
    N,
    /// New Zealand Stock Exchange
    NZ,
    /// NewEx (Austria)
    NW,
    /// Occidente Stock Exchange
    OD,
    /// Osaka Stock Exchange
    OS,
    /// Oslo Stock Exchange
    OL,
    /// Pacific Stock Exchange
    P,
    /// Paris Stock Exchange
    PA,
    /// Philadelphia Stock Exchange
    PH,
    /// Philadelphia Stock Exchange Options
    X,
    /// Philippine Stock Exchange
    PS,
    /// Prague Stock Exchange
    PR,
    /// Pink Sheets (National Quotation Bureau)
    PNK,
    /// RASDAQ (Romania)
    RQ,
    /// Riga Stock Exchange
    RI,
    /// Rio de Janeiro OTC Stock Exchange (SOMA)
    SO,
    /// Russian Trading System
    RTS,
    /// Santiago Stock Exchange
    SN,
    /// Sao Paulo Stock Exchange
    SA,
    /// Sapporo Stock Exchange
    SP,
    /// Saudi Stock Exchange
    SE,
    /// SBI Stock Exchange (Sweden)
    SBI,
    /// Singapore Stock Exchange
    SI,
    /// Shanghai Stock Exchange
    SS,
    /// Shenzhen Stock Exchange
    SZ,
    /// Stockholm Stock Exchange
    ST,
    /// Stuttgart Stock Exchange
    SG,
    /// St. Petersburg Stock Exchange
    PE,
    /// Surabaya Stock Exchange
    SU,
    /// SWX Swiss Exchange
    S,
    /// Taiwan OTC Securities Exchange
    TWO,
    /// Taiwan Stock Exchange
    TW,
    /// Tallinn Stock Exchange
    TL,
    /// Tel Aviv Stock Exchange
    TA,
    /// Thailand Stock Exchange
    BK,
    /// Third Market
    TH,
    /// Tokyo Stock Exchange
    T,
    /// Toronto Options Exchange
    K,
    /// Toronto Stock Exchange
    TO,
    /// Tradepoint Stock Exchange
    TP,
    /// Tunis Stock Exchange
    TN,
    /// Ukraine PFTS
    PFT,
    /// Valencia Stock Exchange
    VA,
    /// Vilnus Stock Exchange
    VL,
    /// virt-x
    VX,
    /// Vienna Stock Exchange
    VI,
    /// Zimbabwe Stock Exchange
    ZI,
    /// American Stock Exchange Options
    R1,
    /// Chicago Mercantile Exchange (CME) Futures Exchange (LIFFE)
    R2,
    /// Jiway
    R14,
    /// International Securities Market Association(ISMA)
    R15,
    /// London International Financial
    R3,
    /// London Traded Options Market
    R5,
    /// MEFF Renta Variable
    R16,
    /// Montreal Exchange Options (MOE)
    MOE,
    /// New York Mercantile Exchange (NYMEX)
    R12,
    /// Non-exchange-based Over The Counter Market
    R11,
    /// NYFIX Millennium
    R13,
    /// NYSE BBSS (broker booth system)
    R10,
    /// Pacific Stock Exchange Options (PAO)
    R8,
    /// POSIT
    R4,
    /// Stockholm Options Market
    R17,
    /// Vancouver Options Exchange (VAO)
    R9,
}


impl FIXValue for ReutersExchange {
    fn from_bytes(bytes: &[u8]) -> Option<ReutersExchange> {
        match bytes {
            b"CI" => Some(ReutersExchange::CI),
            b"A" => Some(ReutersExchange::A),
            b"AM" => Some(ReutersExchange::AM),
            b"E" => Some(ReutersExchange::E),
            b"AS" => Some(ReutersExchange::AS),
            b"AX" => Some(ReutersExchange::AX),
            b"BH" => Some(ReutersExchange::BH),
            b"BC" => Some(ReutersExchange::BC),
            b"MC" => Some(ReutersExchange::MC),
            b"BY" => Some(ReutersExchange::BY),
            b"b" => Some(ReutersExchange::b),
            b"BE" => Some(ReutersExchange::BE),
            b"BN" => Some(ReutersExchange::BN),
            b"BI" => Some(ReutersExchange::BI),
            b"BO" => Some(ReutersExchange::BO),
            b"B" => Some(ReutersExchange::B),
            b"BT" => Some(ReutersExchange::BT),
            b"BM" => Some(ReutersExchange::BM),
            b"BR" => Some(ReutersExchange::BR),
            b"CL" => Some(ReutersExchange::CL),
            b"V" => Some(ReutersExchange::V),
            b"CH" => Some(ReutersExchange::CH),
            b"W" => Some(ReutersExchange::W),
            b"MW" => Some(ReutersExchange::MW),
            b"CE" => Some(ReutersExchange::CE),
            b"C" => Some(ReutersExchange::C),
            b"CM" => Some(ReutersExchange::CM),
            b"CO" => Some(ReutersExchange::CO),
            b"DL" => Some(ReutersExchange::DL),
            b"QA" => Some(ReutersExchange::QA),
            b"DU" => Some(ReutersExchange::DU),
            b"D" => Some(ReutersExchange::D),
            b"EB" => Some(ReutersExchange::EB),
            b"d" => Some(ReutersExchange::d),
            b"Z" => Some(ReutersExchange::Z),
            b"F" => Some(ReutersExchange::F),
            b"FU" => Some(ReutersExchange::FU),
            b"GH" => Some(ReutersExchange::GH),
            b"H" => Some(ReutersExchange::H),
            b"HA" => Some(ReutersExchange::HA),
            b"HE" => Some(ReutersExchange::HE),
            b"HK" => Some(ReutersExchange::HK),
            b"IC" => Some(ReutersExchange::IC),
            b"IN" => Some(ReutersExchange::IN),
            b"I" => Some(ReutersExchange::I),
            b"IS" => Some(ReutersExchange::IS),
            b"JK" => Some(ReutersExchange::JK),
            b"Q" => Some(ReutersExchange::Q),
            b"J" => Some(ReutersExchange::J),
            b"KA" => Some(ReutersExchange::KA),
            b"KQ" => Some(ReutersExchange::KQ),
            b"KZ" => Some(ReutersExchange::KZ),
            b"KS" => Some(ReutersExchange::KS),
            b"KL" => Some(ReutersExchange::KL),
            b"KW" => Some(ReutersExchange::KW),
            b"KY" => Some(ReutersExchange::KY),
            b"LG" => Some(ReutersExchange::LG),
            b"LA" => Some(ReutersExchange::LA),
            b"LN" => Some(ReutersExchange::LN),
            b"LM" => Some(ReutersExchange::LM),
            b"LS" => Some(ReutersExchange::LS),
            b"L" => Some(ReutersExchange::L),
            b"LZ" => Some(ReutersExchange::LZ),
            b"LU" => Some(ReutersExchange::LU),
            b"MD" => Some(ReutersExchange::MD),
            b"MA" => Some(ReutersExchange::MA),
            b"MT" => Some(ReutersExchange::MT),
            b"MZ" => Some(ReutersExchange::MZ),
            b"ML" => Some(ReutersExchange::ML),
            b"MX" => Some(ReutersExchange::MX),
            b"MI" => Some(ReutersExchange::MI),
            b"p" => Some(ReutersExchange::p),
            b"M" => Some(ReutersExchange::M),
            b"MM" => Some(ReutersExchange::MM),
            b"MO" => Some(ReutersExchange::MO),
            b"MU" => Some(ReutersExchange::MU),
            b"OM" => Some(ReutersExchange::OM),
            b"NM" => Some(ReutersExchange::NM),
            b"NG" => Some(ReutersExchange::NG),
            b"NR" => Some(ReutersExchange::NR),
            b"O" => Some(ReutersExchange::O),
            b"OB" => Some(ReutersExchange::OB),
            b"OJ" => Some(ReutersExchange::OJ),
            b"NS" => Some(ReutersExchange::NS),
            b"N" => Some(ReutersExchange::N),
            b"NZ" => Some(ReutersExchange::NZ),
            b"NW" => Some(ReutersExchange::NW),
            b"OD" => Some(ReutersExchange::OD),
            b"OS" => Some(ReutersExchange::OS),
            b"OL" => Some(ReutersExchange::OL),
            b"P" => Some(ReutersExchange::P),
            b"PA" => Some(ReutersExchange::PA),
            b"PH" => Some(ReutersExchange::PH),
            b"X" => Some(ReutersExchange::X),
            b"PS" => Some(ReutersExchange::PS),
            b"PR" => Some(ReutersExchange::PR),
            b"PNK" => Some(ReutersExchange::PNK),
            b"RQ" => Some(ReutersExchange::RQ),
            b"RI" => Some(ReutersExchange::RI),
            b"SO" => Some(ReutersExchange::SO),
            b"RTS" => Some(ReutersExchange::RTS),
            b"SN" => Some(ReutersExchange::SN),
            b"SA" => Some(ReutersExchange::SA),
            b"SP" => Some(ReutersExchange::SP),
            b"SE" => Some(ReutersExchange::SE),
            b"SBI" => Some(ReutersExchange::SBI),
            b"SI" => Some(ReutersExchange::SI),
            b"SS" => Some(ReutersExchange::SS),
            b"SZ" => Some(ReutersExchange::SZ),
            b"ST" => Some(ReutersExchange::ST),
            b"SG" => Some(ReutersExchange::SG),
            b"PE" => Some(ReutersExchange::PE),
            b"SU" => Some(ReutersExchange::SU),
            b"S" => Some(ReutersExchange::S),
            b"TWO" => Some(ReutersExchange::TWO),
            b"TW" => Some(ReutersExchange::TW),
            b"TL" => Some(ReutersExchange::TL),
            b"TA" => Some(ReutersExchange::TA),
            b"BK" => Some(ReutersExchange::BK),
            b"TH" => Some(ReutersExchange::TH),
            b"T" => Some(ReutersExchange::T),
            b"K" => Some(ReutersExchange::K),
            b"TO" => Some(ReutersExchange::TO),
            b"TP" => Some(ReutersExchange::TP),
            b"TN" => Some(ReutersExchange::TN),
            b"PFT" => Some(ReutersExchange::PFT),
            b"VA" => Some(ReutersExchange::VA),
            b"VL" => Some(ReutersExchange::VL),
            b"VX" => Some(ReutersExchange::VX),
            b"VI" => Some(ReutersExchange::VI),
            b"ZI" => Some(ReutersExchange::ZI),
            b"R1" => Some(ReutersExchange::R1),
            b"R2" => Some(ReutersExchange::R2),
            b"R14" => Some(ReutersExchange::R14),
            b"R15" => Some(ReutersExchange::R15),
            b"R3" => Some(ReutersExchange::R3),
            b"R5" => Some(ReutersExchange::R5),
            b"R16" => Some(ReutersExchange::R16),
            b"MOE" => Some(ReutersExchange::MOE),
            b"R12" => Some(ReutersExchange::R12),
            b"R11" => Some(ReutersExchange::R11),
            b"R13" => Some(ReutersExchange::R13),
            b"R10" => Some(ReutersExchange::R10),
            b"R8" => Some(ReutersExchange::R8),
            b"R4" => Some(ReutersExchange::R4),
            b"R17" => Some(ReutersExchange::R17),
            b"R9" => Some(ReutersExchange::R9),
            _ => None
        }
    }

    fn to_bytes(&self, v: &mut Vec<u8>) {
        match *self {
            ReutersExchange::CI => v.extend(b"CI"),
            ReutersExchange::A => v.extend(b"A"),
            ReutersExchange::AM => v.extend(b"AM"),
            ReutersExchange::E => v.extend(b"E"),
            ReutersExchange::AS => v.extend(b"AS"),
            ReutersExchange::AX => v.extend(b"AX"),
            ReutersExchange::BH => v.extend(b"BH"),
            ReutersExchange::BC => v.extend(b"BC"),
            ReutersExchange::MC => v.extend(b"MC"),
            ReutersExchange::BY => v.extend(b"BY"),
            ReutersExchange::b => v.extend(b"b"),
            ReutersExchange::BE => v.extend(b"BE"),
            ReutersExchange::BN => v.extend(b"BN"),
            ReutersExchange::BI => v.extend(b"BI"),
            ReutersExchange::BO => v.extend(b"BO"),
            ReutersExchange::B => v.extend(b"B"),
            ReutersExchange::BT => v.extend(b"BT"),
            ReutersExchange::BM => v.extend(b"BM"),
            ReutersExchange::BR => v.extend(b"BR"),
            ReutersExchange::CL => v.extend(b"CL"),
            ReutersExchange::V => v.extend(b"V"),
            ReutersExchange::CH => v.extend(b"CH"),
            ReutersExchange::W => v.extend(b"W"),
            ReutersExchange::MW => v.extend(b"MW"),
            ReutersExchange::CE => v.extend(b"CE"),
            ReutersExchange::C => v.extend(b"C"),
            ReutersExchange::CM => v.extend(b"CM"),
            ReutersExchange::CO => v.extend(b"CO"),
            ReutersExchange::DL => v.extend(b"DL"),
            ReutersExchange::QA => v.extend(b"QA"),
            ReutersExchange::DU => v.extend(b"DU"),
            ReutersExchange::D => v.extend(b"D"),
            ReutersExchange::EB => v.extend(b"EB"),
            ReutersExchange::d => v.extend(b"d"),
            ReutersExchange::Z => v.extend(b"Z"),
            ReutersExchange::F => v.extend(b"F"),
            ReutersExchange::FU => v.extend(b"FU"),
            ReutersExchange::GH => v.extend(b"GH"),
            ReutersExchange::H => v.extend(b"H"),
            ReutersExchange::HA => v.extend(b"HA"),
            ReutersExchange::HE => v.extend(b"HE"),
            ReutersExchange::HK => v.extend(b"HK"),
            ReutersExchange::IC => v.extend(b"IC"),
            ReutersExchange::IN => v.extend(b"IN"),
            ReutersExchange::I => v.extend(b"I"),
            ReutersExchange::IS => v.extend(b"IS"),
            ReutersExchange::JK => v.extend(b"JK"),
            ReutersExchange::Q => v.extend(b"Q"),
            ReutersExchange::J => v.extend(b"J"),
            ReutersExchange::KA => v.extend(b"KA"),
            ReutersExchange::KQ => v.extend(b"KQ"),
            ReutersExchange::KZ => v.extend(b"KZ"),
            ReutersExchange::KS => v.extend(b"KS"),
            ReutersExchange::KL => v.extend(b"KL"),
            ReutersExchange::KW => v.extend(b"KW"),
            ReutersExchange::KY => v.extend(b"KY"),
            ReutersExchange::LG => v.extend(b"LG"),
            ReutersExchange::LA => v.extend(b"LA"),
            ReutersExchange::LN => v.extend(b"LN"),
            ReutersExchange::LM => v.extend(b"LM"),
            ReutersExchange::LS => v.extend(b"LS"),
            ReutersExchange::L => v.extend(b"L"),
            ReutersExchange::LZ => v.extend(b"LZ"),
            ReutersExchange::LU => v.extend(b"LU"),
            ReutersExchange::MD => v.extend(b"MD"),
            ReutersExchange::MA => v.extend(b"MA"),
            ReutersExchange::MT => v.extend(b"MT"),
            ReutersExchange::MZ => v.extend(b"MZ"),
            ReutersExchange::ML => v.extend(b"ML"),
            ReutersExchange::MX => v.extend(b"MX"),
            ReutersExchange::MI => v.extend(b"MI"),
            ReutersExchange::p => v.extend(b"p"),
            ReutersExchange::M => v.extend(b"M"),
            ReutersExchange::MM => v.extend(b"MM"),
            ReutersExchange::MO => v.extend(b"MO"),
            ReutersExchange::MU => v.extend(b"MU"),
            ReutersExchange::OM => v.extend(b"OM"),
            ReutersExchange::NM => v.extend(b"NM"),
            ReutersExchange::NG => v.extend(b"NG"),
            ReutersExchange::NR => v.extend(b"NR"),
            ReutersExchange::O => v.extend(b"O"),
            ReutersExchange::OB => v.extend(b"OB"),
            ReutersExchange::OJ => v.extend(b"OJ"),
            ReutersExchange::NS => v.extend(b"NS"),
            ReutersExchange::N => v.extend(b"N"),
            ReutersExchange::NZ => v.extend(b"NZ"),
            ReutersExchange::NW => v.extend(b"NW"),
            ReutersExchange::OD => v.extend(b"OD"),
            ReutersExchange::OS => v.extend(b"OS"),
            ReutersExchange::OL => v.extend(b"OL"),
            ReutersExchange::P => v.extend(b"P"),
            ReutersExchange::PA => v.extend(b"PA"),
            ReutersExchange::PH => v.extend(b"PH"),
            ReutersExchange::X => v.extend(b"X"),
            ReutersExchange::PS => v.extend(b"PS"),
            ReutersExchange::PR => v.extend(b"PR"),
            ReutersExchange::PNK => v.extend(b"PNK"),
            ReutersExchange::RQ => v.extend(b"RQ"),
            ReutersExchange::RI => v.extend(b"RI"),
            ReutersExchange::SO => v.extend(b"SO"),
            ReutersExchange::RTS => v.extend(b"RTS"),
            ReutersExchange::SN => v.extend(b"SN"),
            ReutersExchange::SA => v.extend(b"SA"),
            ReutersExchange::SP => v.extend(b"SP"),
            ReutersExchange::SE => v.extend(b"SE"),
            ReutersExchange::SBI => v.extend(b"SBI"),
            ReutersExchange::SI => v.extend(b"SI"),
            ReutersExchange::SS => v.extend(b"SS"),
            ReutersExchange::SZ => v.extend(b"SZ"),
            ReutersExchange::ST => v.extend(b"ST"),
            ReutersExchange::SG => v.extend(b"SG"),
            ReutersExchange::PE => v.extend(b"PE"),
            ReutersExchange::SU => v.extend(b"SU"),
            ReutersExchange::S => v.extend(b"S"),
            ReutersExchange::TWO => v.extend(b"TWO"),
            ReutersExchange::TW => v.extend(b"TW"),
            ReutersExchange::TL => v.extend(b"TL"),
            ReutersExchange::TA => v.extend(b"TA"),
            ReutersExchange::BK => v.extend(b"BK"),
            ReutersExchange::TH => v.extend(b"TH"),
            ReutersExchange::T => v.extend(b"T"),
            ReutersExchange::K => v.extend(b"K"),
            ReutersExchange::TO => v.extend(b"TO"),
            ReutersExchange::TP => v.extend(b"TP"),
            ReutersExchange::TN => v.extend(b"TN"),
            ReutersExchange::PFT => v.extend(b"PFT"),
            ReutersExchange::VA => v.extend(b"VA"),
            ReutersExchange::VL => v.extend(b"VL"),
            ReutersExchange::VX => v.extend(b"VX"),
            ReutersExchange::VI => v.extend(b"VI"),
            ReutersExchange::ZI => v.extend(b"ZI"),
            ReutersExchange::R1 => v.extend(b"R1"),
            ReutersExchange::R2 => v.extend(b"R2"),
            ReutersExchange::R14 => v.extend(b"R14"),
            ReutersExchange::R15 => v.extend(b"R15"),
            ReutersExchange::R3 => v.extend(b"R3"),
            ReutersExchange::R5 => v.extend(b"R5"),
            ReutersExchange::R16 => v.extend(b"R16"),
            ReutersExchange::MOE => v.extend(b"MOE"),
            ReutersExchange::R12 => v.extend(b"R12"),
            ReutersExchange::R11 => v.extend(b"R11"),
            ReutersExchange::R13 => v.extend(b"R13"),
            ReutersExchange::R10 => v.extend(b"R10"),
            ReutersExchange::R8 => v.extend(b"R8"),
            ReutersExchange::R4 => v.extend(b"R4"),
            ReutersExchange::R17 => v.extend(b"R17"),
            ReutersExchange::R9 => v.extend(b"R9"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use protocol::FIXValue;

    #[test]
    fn test_rt_exchange() {
        // decode
        assert_eq!(ReutersExchange::from_bytes(b"AX").unwrap(), ReutersExchange::AX);
        assert_eq!(ReutersExchange::from_bytes(b"BO").unwrap(), ReutersExchange::BO);
        assert!(ReutersExchange::from_bytes(b"UX").is_none());

        // encode
        let mut v = Vec::new();
        ReutersExchange::AX.to_bytes(&mut v);
        ReutersExchange::BO.to_bytes(&mut v);
        assert_eq!(&v, b"AXBO");
    }
}
