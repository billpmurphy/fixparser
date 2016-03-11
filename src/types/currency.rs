use protocol::FIXValue;


/// ISO 4271 currency code
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Currency {
    /// UAE Dirham
    AED,
    /// Afghani
    AFN,
    /// Lek
    ALL,
    /// Armenian Dram
    AMD,
    /// Netherlands Antillean Guilder
    ANG,
    /// Kwanza
    AOA,
    /// Argentine Peso
    ARS,
    /// Australian Dollar
    AUD,
    /// Aruban Florin
    AWG,
    /// Azerbaijanian Manat
    AZN,
    /// Convertible Mark
    BAM,
    /// Barbados Dollar
    BBD,
    /// Taka
    BDT,
    /// Bulgarian Lev
    BGN,
    /// Bahraini Dinar
    BHD,
    /// Burundi Franc
    BIF,
    /// Bermudian Dollar
    BMD,
    /// Brunei Dollar
    BND,
    /// Boliviano
    BOB,
    /// Mvdol
    BOV,
    /// Brazilian Real
    BRL,
    /// Bahamian Dollar
    BSD,
    /// Ngultrum
    BTN,
    /// Pula
    BWP,
    /// Belarussian Ruble
    BYR,
    /// Belize Dollar
    BZD,
    /// Canadian Dollar
    CAD,
    /// Congolese Franc
    CDF,
    /// WIR Euro
    CHE,
    /// Swiss Franc
    CHF,
    /// WIR Franc
    CHW,
    /// Unidad de Fomento
    CLF,
    /// Chilean Peso
    CLP,
    /// Yuan Renminbi
    CNY,
    /// Colombian Peso
    COP,
    /// Unidad de Valor Real
    COU,
    /// Costa Rican Colon
    CRC,
    /// Peso Convertible
    CUC,
    /// Cuban Peso
    CUP,
    /// Cabo Verde Escudo
    CVE,
    /// Czech Koruna
    CZK,
    /// Djibouti Franc
    DJF,
    /// Danish Krone
    DKK,
    /// Dominican Peso
    DOP,
    /// Algerian Dinar
    DZD,
    /// Egyptian Pound
    EGP,
    /// Nakfa
    ERN,
    /// Ethiopian Birr
    ETB,
    /// Euro
    EUR,
    /// Fiji Dollar
    FJD,
    /// Falkland Islands Pound
    FKP,
    /// Pound Sterling
    GBP,
    /// Lari
    GEL,
    /// Ghana Cedi
    GHS,
    /// Gibraltar Pound
    GIP,
    /// Dalasi
    GMD,
    /// Guinea Franc
    GNF,
    /// Quetzal
    GTQ,
    /// Guyana Dollar
    GYD,
    /// Hong Kong Dollar
    HKD,
    /// Lempira
    HNL,
    /// Kuna
    HRK,
    /// Gourde
    HTG,
    /// Forint
    HUF,
    /// Rupiah
    IDR,
    /// New Israeli Sheqel
    ILS,
    /// Indian Rupee
    INR,
    /// Iraqi Dinar
    IQD,
    /// Iranian Rial
    IRR,
    /// Iceland Krona
    ISK,
    /// Jamaican Dollar
    JMD,
    /// Jordanian Dinar
    JOD,
    /// Yen
    JPY,
    /// Kenyan Shilling
    KES,
    /// Som
    KGS,
    /// Riel
    KHR,
    /// Comoro Franc
    KMF,
    /// North Korean Won
    KPW,
    /// Won
    KRW,
    /// Kuwaiti Dinar
    KWD,
    /// Cayman Islands Dollar
    KYD,
    /// Tenge
    KZT,
    /// Kip
    LAK,
    /// Lebanese Pound
    LBP,
    /// Sri Lanka Rupee
    LKR,
    /// Liberian Dollar
    LRD,
    /// Loti
    LSL,
    /// Libyan Dinar
    LYD,
    /// Moroccan Dirham
    MAD,
    /// Moldovan Leu
    MDL,
    /// Malagasy Ariary
    MGA,
    /// Denar
    MKD,
    /// Kyat
    MMK,
    /// Tugrik
    MNT,
    /// Pataca
    MOP,
    /// Ouguiya
    MRO,
    /// Mauritius Rupee
    MUR,
    /// Rufiyaa
    MVR,
    /// Kwacha
    MWK,
    /// Mexican Peso
    MXN,
    /// Mexican Unidad de Inversion (UDI)
    MXV,
    /// Malaysian Ringgit
    MYR,
    /// Mozambique Metical
    MZN,
    /// Namibia Dollar
    NAD,
    /// Naira
    NGN,
    /// Cordoba Oro
    NIO,
    /// Norwegian Krone
    NOK,
    /// Nepalese Rupee
    NPR,
    /// New Zealand Dollar
    NZD,
    /// Rial Omani
    OMR,
    /// Balboa
    PAB,
    /// Nuevo Sol
    PEN,
    /// Kina
    PGK,
    /// Philippine Peso
    PHP,
    /// Pakistan Rupee
    PKR,
    /// Zloty
    PLN,
    /// Guarani
    PYG,
    /// Qatari Rial
    QAR,
    /// Romanian Leu
    RON,
    /// Serbian Dinar
    RSD,
    /// Russian Ruble
    RUB,
    /// Rwanda Franc
    RWF,
    /// Saudi Riyal
    SAR,
    /// Solomon Islands Dollar
    SBD,
    /// Seychelles Rupee
    SCR,
    /// Sudanese Pound
    SDG,
    /// Swedish Krona
    SEK,
    /// Singapore Dollar
    SGD,
    /// Saint Helena Pound
    SHP,
    /// Leone
    SLL,
    /// Somali Shilling
    SOS,
    /// Surinam Dollar
    SRD,
    /// South Sudanese Pound
    SSP,
    /// Dobra
    STD,
    /// El Salvador Colon
    SVC,
    /// Syrian Pound
    SYP,
    /// Lilangeni
    SZL,
    /// Baht
    THB,
    /// Somoni
    TJS,
    /// Turkmenistan New Manat
    TMT,
    /// Tunisian Dinar
    TND,
    /// Pa’anga
    TOP,
    /// Turkish Lira
    TRY,
    /// Trinidad and Tobago Dollar
    TTD,
    /// New Taiwan Dollar
    TWD,
    /// Tanzanian Shilling
    TZS,
    /// Hryvnia
    UAH,
    /// Uganda Shilling
    UGX,
    /// US Dollar
    USD,
    /// US Dollar (Next day)
    USN,
    /// Uruguay Peso en Unidades Indexadas (URUIURUI)
    UYI,
    /// Peso Uruguayo
    UYU,
    /// Uzbekistan Sum
    UZS,
    /// Bolivar
    VEF,
    /// Dong
    VND,
    /// Vatu
    VUV,
    /// Tala
    WST,
    /// CFA Franc BEAC
    XAF,
    /// Silver
    XAG,
    /// Gold
    XAU,
    /// Bond Markets Unit European Composite Unit (EURCO)
    XBA,
    /// Bond Markets Unit European Monetary Unit (E.M.U.-6)
    XBB,
    /// Bond Markets Unit European Unit of Account 9 (E.U.A.-9)
    XBC,
    /// Bond Markets Unit European Unit of Account 17 (E.U.A.-17)
    XBD,
    /// East Caribbean Dollar
    XCD,
    /// SDR (Special Drawing Right)
    XDR,
    /// CFA Franc BCEAO
    XOF,
    /// Palladium
    XPD,
    /// CFP Franc
    XPF,
    /// Platinum
    XPT,
    /// Sucre
    XSU,
    /// Codes specifically reserved for testing purposes
    XTS,
    /// ADB Unit of Account
    XUA,
    /// The codes assigned for transactions where no currency is involved
    XXX,
    /// Yemeni Rial
    YER,
    /// Rand
    ZAR,
    /// Zambian Kwacha
    ZMW,
    /// Zimbabwe Dollar
    ZWL
}


impl FIXValue for Currency {
    fn from_bytes(bytes: &[u8]) -> Option<Currency> {
        match bytes {
            b"AED" => Some(Currency::AED),
            b"AFN" => Some(Currency::AFN),
            b"ALL" => Some(Currency::ALL),
            b"AMD" => Some(Currency::AMD),
            b"ANG" => Some(Currency::ANG),
            b"AOA" => Some(Currency::AOA),
            b"ARS" => Some(Currency::ARS),
            b"AUD" => Some(Currency::AUD),
            b"AWG" => Some(Currency::AWG),
            b"AZN" => Some(Currency::AZN),
            b"BAM" => Some(Currency::BAM),
            b"BBD" => Some(Currency::BBD),
            b"BDT" => Some(Currency::BDT),
            b"BGN" => Some(Currency::BGN),
            b"BHD" => Some(Currency::BHD),
            b"BIF" => Some(Currency::BIF),
            b"BMD" => Some(Currency::BMD),
            b"BND" => Some(Currency::BND),
            b"BOB" => Some(Currency::BOB),
            b"BOV" => Some(Currency::BOV),
            b"BRL" => Some(Currency::BRL),
            b"BSD" => Some(Currency::BSD),
            b"BTN" => Some(Currency::BTN),
            b"BWP" => Some(Currency::BWP),
            b"BYR" => Some(Currency::BYR),
            b"BZD" => Some(Currency::BZD),
            b"CAD" => Some(Currency::CAD),
            b"CDF" => Some(Currency::CDF),
            b"CHE" => Some(Currency::CHE),
            b"CHF" => Some(Currency::CHF),
            b"CHW" => Some(Currency::CHW),
            b"CLF" => Some(Currency::CLF),
            b"CLP" => Some(Currency::CLP),
            b"CNY" => Some(Currency::CNY),
            b"COP" => Some(Currency::COP),
            b"COU" => Some(Currency::COU),
            b"CRC" => Some(Currency::CRC),
            b"CUC" => Some(Currency::CUC),
            b"CUP" => Some(Currency::CUP),
            b"CVE" => Some(Currency::CVE),
            b"CZK" => Some(Currency::CZK),
            b"DJF" => Some(Currency::DJF),
            b"DKK" => Some(Currency::DKK),
            b"DOP" => Some(Currency::DOP),
            b"DZD" => Some(Currency::DZD),
            b"EGP" => Some(Currency::EGP),
            b"ERN" => Some(Currency::ERN),
            b"ETB" => Some(Currency::ETB),
            b"EUR" => Some(Currency::EUR),
            b"FJD" => Some(Currency::FJD),
            b"FKP" => Some(Currency::FKP),
            b"GBP" => Some(Currency::GBP),
            b"GEL" => Some(Currency::GEL),
            b"GHS" => Some(Currency::GHS),
            b"GIP" => Some(Currency::GIP),
            b"GMD" => Some(Currency::GMD),
            b"GNF" => Some(Currency::GNF),
            b"GTQ" => Some(Currency::GTQ),
            b"GYD" => Some(Currency::GYD),
            b"HKD" => Some(Currency::HKD),
            b"HNL" => Some(Currency::HNL),
            b"HRK" => Some(Currency::HRK),
            b"HTG" => Some(Currency::HTG),
            b"HUF" => Some(Currency::HUF),
            b"IDR" => Some(Currency::IDR),
            b"ILS" => Some(Currency::ILS),
            b"INR" => Some(Currency::INR),
            b"IQD" => Some(Currency::IQD),
            b"IRR" => Some(Currency::IRR),
            b"ISK" => Some(Currency::ISK),
            b"JMD" => Some(Currency::JMD),
            b"JOD" => Some(Currency::JOD),
            b"JPY" => Some(Currency::JPY),
            b"KES" => Some(Currency::KES),
            b"KGS" => Some(Currency::KGS),
            b"KHR" => Some(Currency::KHR),
            b"KMF" => Some(Currency::KMF),
            b"KPW" => Some(Currency::KPW),
            b"KRW" => Some(Currency::KRW),
            b"KWD" => Some(Currency::KWD),
            b"KYD" => Some(Currency::KYD),
            b"KZT" => Some(Currency::KZT),
            b"LAK" => Some(Currency::LAK),
            b"LBP" => Some(Currency::LBP),
            b"LKR" => Some(Currency::LKR),
            b"LRD" => Some(Currency::LRD),
            b"LSL" => Some(Currency::LSL),
            b"LYD" => Some(Currency::LYD),
            b"MAD" => Some(Currency::MAD),
            b"MDL" => Some(Currency::MDL),
            b"MGA" => Some(Currency::MGA),
            b"MKD" => Some(Currency::MKD),
            b"MMK" => Some(Currency::MMK),
            b"MNT" => Some(Currency::MNT),
            b"MOP" => Some(Currency::MOP),
            b"MRO" => Some(Currency::MRO),
            b"MUR" => Some(Currency::MUR),
            b"MVR" => Some(Currency::MVR),
            b"MWK" => Some(Currency::MWK),
            b"MXN" => Some(Currency::MXN),
            b"MXV" => Some(Currency::MXV),
            b"MYR" => Some(Currency::MYR),
            b"MZN" => Some(Currency::MZN),
            b"NAD" => Some(Currency::NAD),
            b"NGN" => Some(Currency::NGN),
            b"NIO" => Some(Currency::NIO),
            b"NOK" => Some(Currency::NOK),
            b"NPR" => Some(Currency::NPR),
            b"NZD" => Some(Currency::NZD),
            b"OMR" => Some(Currency::OMR),
            b"PAB" => Some(Currency::PAB),
            b"PEN" => Some(Currency::PEN),
            b"PGK" => Some(Currency::PGK),
            b"PHP" => Some(Currency::PHP),
            b"PKR" => Some(Currency::PKR),
            b"PLN" => Some(Currency::PLN),
            b"PYG" => Some(Currency::PYG),
            b"QAR" => Some(Currency::QAR),
            b"RON" => Some(Currency::RON),
            b"RSD" => Some(Currency::RSD),
            b"RUB" => Some(Currency::RUB),
            b"RWF" => Some(Currency::RWF),
            b"SAR" => Some(Currency::SAR),
            b"SBD" => Some(Currency::SBD),
            b"SCR" => Some(Currency::SCR),
            b"SDG" => Some(Currency::SDG),
            b"SEK" => Some(Currency::SEK),
            b"SGD" => Some(Currency::SGD),
            b"SHP" => Some(Currency::SHP),
            b"SLL" => Some(Currency::SLL),
            b"SOS" => Some(Currency::SOS),
            b"SRD" => Some(Currency::SRD),
            b"SSP" => Some(Currency::SSP),
            b"STD" => Some(Currency::STD),
            b"SVC" => Some(Currency::SVC),
            b"SYP" => Some(Currency::SYP),
            b"SZL" => Some(Currency::SZL),
            b"THB" => Some(Currency::THB),
            b"TJS" => Some(Currency::TJS),
            b"TMT" => Some(Currency::TMT),
            b"TND" => Some(Currency::TND),
            b"TOP" => Some(Currency::TOP),
            b"TRY" => Some(Currency::TRY),
            b"TTD" => Some(Currency::TTD),
            b"TWD" => Some(Currency::TWD),
            b"TZS" => Some(Currency::TZS),
            b"UAH" => Some(Currency::UAH),
            b"UGX" => Some(Currency::UGX),
            b"USD" => Some(Currency::USD),
            b"USN" => Some(Currency::USN),
            b"UYI" => Some(Currency::UYI),
            b"UYU" => Some(Currency::UYU),
            b"UZS" => Some(Currency::UZS),
            b"VEF" => Some(Currency::VEF),
            b"VND" => Some(Currency::VND),
            b"VUV" => Some(Currency::VUV),
            b"WST" => Some(Currency::WST),
            b"XAF" => Some(Currency::XAF),
            b"XAG" => Some(Currency::XAG),
            b"XAU" => Some(Currency::XAU),
            b"XBA" => Some(Currency::XBA),
            b"XBB" => Some(Currency::XBB),
            b"XBC" => Some(Currency::XBC),
            b"XBD" => Some(Currency::XBD),
            b"XCD" => Some(Currency::XCD),
            b"XDR" => Some(Currency::XDR),
            b"XOF" => Some(Currency::XOF),
            b"XPD" => Some(Currency::XPD),
            b"XPF" => Some(Currency::XPF),
            b"XPT" => Some(Currency::XPT),
            b"XSU" => Some(Currency::XSU),
            b"XTS" => Some(Currency::XTS),
            b"XUA" => Some(Currency::XUA),
            b"XXX" => Some(Currency::XXX),
            b"YER" => Some(Currency::YER),
            b"ZAR" => Some(Currency::ZAR),
            b"ZMW" => Some(Currency::ZMW),
            b"ZWL" => Some(Currency::ZWL),
            _ => None
        }
    }

    fn to_bytes(&self, v: &mut Vec<u8>) {
        match *self {
            Currency::AED => v.extend(b"AED"),
            Currency::AFN => v.extend(b"AFN"),
            Currency::ALL => v.extend(b"ALL"),
            Currency::AMD => v.extend(b"AMD"),
            Currency::ANG => v.extend(b"ANG"),
            Currency::AOA => v.extend(b"AOA"),
            Currency::ARS => v.extend(b"ARS"),
            Currency::AUD => v.extend(b"AUD"),
            Currency::AWG => v.extend(b"AWG"),
            Currency::AZN => v.extend(b"AZN"),
            Currency::BAM => v.extend(b"BAM"),
            Currency::BBD => v.extend(b"BBD"),
            Currency::BDT => v.extend(b"BDT"),
            Currency::BGN => v.extend(b"BGN"),
            Currency::BHD => v.extend(b"BHD"),
            Currency::BIF => v.extend(b"BIF"),
            Currency::BMD => v.extend(b"BMD"),
            Currency::BND => v.extend(b"BND"),
            Currency::BOB => v.extend(b"BOB"),
            Currency::BOV => v.extend(b"BOV"),
            Currency::BRL => v.extend(b"BRL"),
            Currency::BSD => v.extend(b"BSD"),
            Currency::BTN => v.extend(b"BTN"),
            Currency::BWP => v.extend(b"BWP"),
            Currency::BYR => v.extend(b"BYR"),
            Currency::BZD => v.extend(b"BZD"),
            Currency::CAD => v.extend(b"CAD"),
            Currency::CDF => v.extend(b"CDF"),
            Currency::CHE => v.extend(b"CHE"),
            Currency::CHF => v.extend(b"CHF"),
            Currency::CHW => v.extend(b"CHW"),
            Currency::CLF => v.extend(b"CLF"),
            Currency::CLP => v.extend(b"CLP"),
            Currency::CNY => v.extend(b"CNY"),
            Currency::COP => v.extend(b"COP"),
            Currency::COU => v.extend(b"COU"),
            Currency::CRC => v.extend(b"CRC"),
            Currency::CUC => v.extend(b"CUC"),
            Currency::CUP => v.extend(b"CUP"),
            Currency::CVE => v.extend(b"CVE"),
            Currency::CZK => v.extend(b"CZK"),
            Currency::DJF => v.extend(b"DJF"),
            Currency::DKK => v.extend(b"DKK"),
            Currency::DOP => v.extend(b"DOP"),
            Currency::DZD => v.extend(b"DZD"),
            Currency::EGP => v.extend(b"EGP"),
            Currency::ERN => v.extend(b"ERN"),
            Currency::ETB => v.extend(b"ETB"),
            Currency::EUR => v.extend(b"EUR"),
            Currency::FJD => v.extend(b"FJD"),
            Currency::FKP => v.extend(b"FKP"),
            Currency::GBP => v.extend(b"GBP"),
            Currency::GEL => v.extend(b"GEL"),
            Currency::GHS => v.extend(b"GHS"),
            Currency::GIP => v.extend(b"GIP"),
            Currency::GMD => v.extend(b"GMD"),
            Currency::GNF => v.extend(b"GNF"),
            Currency::GTQ => v.extend(b"GTQ"),
            Currency::GYD => v.extend(b"GYD"),
            Currency::HKD => v.extend(b"HKD"),
            Currency::HNL => v.extend(b"HNL"),
            Currency::HRK => v.extend(b"HRK"),
            Currency::HTG => v.extend(b"HTG"),
            Currency::HUF => v.extend(b"HUF"),
            Currency::IDR => v.extend(b"IDR"),
            Currency::ILS => v.extend(b"ILS"),
            Currency::INR => v.extend(b"INR"),
            Currency::IQD => v.extend(b"IQD"),
            Currency::IRR => v.extend(b"IRR"),
            Currency::ISK => v.extend(b"ISK"),
            Currency::JMD => v.extend(b"JMD"),
            Currency::JOD => v.extend(b"JOD"),
            Currency::JPY => v.extend(b"JPY"),
            Currency::KES => v.extend(b"KES"),
            Currency::KGS => v.extend(b"KGS"),
            Currency::KHR => v.extend(b"KHR"),
            Currency::KMF => v.extend(b"KMF"),
            Currency::KPW => v.extend(b"KPW"),
            Currency::KRW => v.extend(b"KRW"),
            Currency::KWD => v.extend(b"KWD"),
            Currency::KYD => v.extend(b"KYD"),
            Currency::KZT => v.extend(b"KZT"),
            Currency::LAK => v.extend(b"LAK"),
            Currency::LBP => v.extend(b"LBP"),
            Currency::LKR => v.extend(b"LKR"),
            Currency::LRD => v.extend(b"LRD"),
            Currency::LSL => v.extend(b"LSL"),
            Currency::LYD => v.extend(b"LYD"),
            Currency::MAD => v.extend(b"MAD"),
            Currency::MDL => v.extend(b"MDL"),
            Currency::MGA => v.extend(b"MGA"),
            Currency::MKD => v.extend(b"MKD"),
            Currency::MMK => v.extend(b"MMK"),
            Currency::MNT => v.extend(b"MNT"),
            Currency::MOP => v.extend(b"MOP"),
            Currency::MRO => v.extend(b"MRO"),
            Currency::MUR => v.extend(b"MUR"),
            Currency::MVR => v.extend(b"MVR"),
            Currency::MWK => v.extend(b"MWK"),
            Currency::MXN => v.extend(b"MXN"),
            Currency::MXV => v.extend(b"MXV"),
            Currency::MYR => v.extend(b"MYR"),
            Currency::MZN => v.extend(b"MZN"),
            Currency::NAD => v.extend(b"NAD"),
            Currency::NGN => v.extend(b"NGN"),
            Currency::NIO => v.extend(b"NIO"),
            Currency::NOK => v.extend(b"NOK"),
            Currency::NPR => v.extend(b"NPR"),
            Currency::NZD => v.extend(b"NZD"),
            Currency::OMR => v.extend(b"OMR"),
            Currency::PAB => v.extend(b"PAB"),
            Currency::PEN => v.extend(b"PEN"),
            Currency::PGK => v.extend(b"PGK"),
            Currency::PHP => v.extend(b"PHP"),
            Currency::PKR => v.extend(b"PKR"),
            Currency::PLN => v.extend(b"PLN"),
            Currency::PYG => v.extend(b"PYG"),
            Currency::QAR => v.extend(b"QAR"),
            Currency::RON => v.extend(b"RON"),
            Currency::RSD => v.extend(b"RSD"),
            Currency::RUB => v.extend(b"RUB"),
            Currency::RWF => v.extend(b"RWF"),
            Currency::SAR => v.extend(b"SAR"),
            Currency::SBD => v.extend(b"SBD"),
            Currency::SCR => v.extend(b"SCR"),
            Currency::SDG => v.extend(b"SDG"),
            Currency::SEK => v.extend(b"SEK"),
            Currency::SGD => v.extend(b"SGD"),
            Currency::SHP => v.extend(b"SHP"),
            Currency::SLL => v.extend(b"SLL"),
            Currency::SOS => v.extend(b"SOS"),
            Currency::SRD => v.extend(b"SRD"),
            Currency::SSP => v.extend(b"SSP"),
            Currency::STD => v.extend(b"STD"),
            Currency::SVC => v.extend(b"SVC"),
            Currency::SYP => v.extend(b"SYP"),
            Currency::SZL => v.extend(b"SZL"),
            Currency::THB => v.extend(b"THB"),
            Currency::TJS => v.extend(b"TJS"),
            Currency::TMT => v.extend(b"TMT"),
            Currency::TND => v.extend(b"TND"),
            Currency::TOP => v.extend(b"TOP"),
            Currency::TRY => v.extend(b"TRY"),
            Currency::TTD => v.extend(b"TTD"),
            Currency::TWD => v.extend(b"TWD"),
            Currency::TZS => v.extend(b"TZS"),
            Currency::UAH => v.extend(b"UAH"),
            Currency::UGX => v.extend(b"UGX"),
            Currency::USD => v.extend(b"USD"),
            Currency::USN => v.extend(b"USN"),
            Currency::UYI => v.extend(b"UYI"),
            Currency::UYU => v.extend(b"UYU"),
            Currency::UZS => v.extend(b"UZS"),
            Currency::VEF => v.extend(b"VEF"),
            Currency::VND => v.extend(b"VND"),
            Currency::VUV => v.extend(b"VUV"),
            Currency::WST => v.extend(b"WST"),
            Currency::XAF => v.extend(b"XAF"),
            Currency::XAG => v.extend(b"XAG"),
            Currency::XAU => v.extend(b"XAU"),
            Currency::XBA => v.extend(b"XBA"),
            Currency::XBB => v.extend(b"XBB"),
            Currency::XBC => v.extend(b"XBC"),
            Currency::XBD => v.extend(b"XBD"),
            Currency::XCD => v.extend(b"XCD"),
            Currency::XDR => v.extend(b"XDR"),
            Currency::XOF => v.extend(b"XOF"),
            Currency::XPD => v.extend(b"XPD"),
            Currency::XPF => v.extend(b"XPF"),
            Currency::XPT => v.extend(b"XPT"),
            Currency::XSU => v.extend(b"XSU"),
            Currency::XTS => v.extend(b"XTS"),
            Currency::XUA => v.extend(b"XUA"),
            Currency::XXX => v.extend(b"XXX"),
            Currency::YER => v.extend(b"YER"),
            Currency::ZAR => v.extend(b"ZAR"),
            Currency::ZMW => v.extend(b"ZMW"),
            Currency::ZWL => v.extend(b"ZWL"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::testutils::{check_decode_encode, check_no_decode};

    #[test]
    fn test_currency() {
        check_decode_encode(b"USD", Currency::USD);
        check_decode_encode(b"ZWL", Currency::ZWL);
        check_no_decode::<Currency>(b"AXA");
    }
}
