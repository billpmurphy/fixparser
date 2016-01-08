use decoder::{Result, DecodeError};

/// ISO 3166 country code representing a country.
///
/// Used in FIX Protocol Versions 4.3, 4.4, 5.0, 5.0 SP1, and 5.0 SP2
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Country {
    ///Andorra
    AD,
    ///United Arab Emirates
    AE,
    ///Afghanistan
    AF,
    ///Antigua and Barbuda
    AG,
    ///Anguilla
    AI,
    ///Albania
    AL,
    ///Armenia
    AM,
    ///Angola
    AO,
    ///Antarctica
    AQ,
    ///Argentina
    AR,
    ///American Samoa
    AS,
    ///Austria
    AT,
    ///Australia
    AU,
    ///Aruba
    AW,
    ///Åland Islands
    AX,
    ///Azerbaijan
    AZ,
    ///Bosnia and Herzegovina
    BA,
    ///Barbados
    BB,
    ///Bangladesh
    BD,
    ///Belgium
    BE,
    ///Burkina Faso
    BF,
    ///Bulgaria
    BG,
    ///Bahrain
    BH,
    ///Burundi
    BI,
    ///Benin
    BJ,
    ///Saint Barthélemy
    BL,
    ///Bermuda
    BM,
    ///Brunei Darussalam
    BN,
    ///Bolivia, Plurinational State of
    BO,
    ///Bonaire, Sint Eustatius and Saba
    BQ,
    ///Brazil
    BR,
    ///Bahamas
    BS,
    ///Bhutan
    BT,
    ///Bouvet Island
    BV,
    ///Botswana
    BW,
    ///Belarus
    BY,
    ///Belize
    BZ,
    ///Canada
    CA,
    ///Cocos (Keeling) Islands
    CC,
    ///Congo, the Democratic Republic of the
    CD,
    ///Central African Republic
    CF,
    ///Congo
    CG,
    ///Switzerland
    CH,
    ///Côte d'Ivoire
    CI,
    ///Cook Islands
    CK,
    ///Chile
    CL,
    ///Cameroon
    CM,
    ///China
    CN,
    ///Colombia
    CO,
    ///Costa Rica
    CR,
    ///Cuba
    CU,
    ///Cabo Verde
    CV,
    ///Curaçao
    CW,
    ///Christmas Island
    CX,
    ///Cyprus
    CY,
    ///Czech Republic
    CZ,
    ///Germany
    DE,
    ///Djibouti
    DJ,
    ///Denmark
    DK,
    ///Dominica
    DM,
    ///Dominican Republic
    DO,
    ///Algeria
    DZ,
    ///Ecuador
    EC,
    ///Estonia
    EE,
    ///Egypt
    EG,
    ///Western Sahara
    EH,
    ///Eritrea
    ER,
    ///Spain
    ES,
    ///Ethiopia
    ET,
    ///Finland
    FI,
    ///Fiji
    FJ,
    ///Falkland Islands (Malvinas)
    FK,
    ///Micronesia, Federated States of
    FM,
    ///Faroe Islands
    FO,
    ///France
    FR,
    ///Gabon
    GA,
    ///United Kingdom of Great Britain and Northern Ireland
    GB,
    ///Grenada
    GD,
    ///Georgia
    GE,
    ///French Guiana
    GF,
    ///Guernsey
    GG,
    ///Ghana
    GH,
    ///Gibraltar
    GI,
    ///Greenland
    GL,
    ///Gambia
    GM,
    ///Guinea
    GN,
    ///Guadeloupe
    GP,
    ///Equatorial Guinea
    GQ,
    ///Greece
    GR,
    ///South Georgia and the South Sandwich Islands
    GS,
    ///Guatemala
    GT,
    ///Guam
    GU,
    ///Guinea-Bissau
    GW,
    ///Guyana
    GY,
    ///Hong Kong
    HK,
    ///Heard Island and McDonald Islands
    HM,
    ///Honduras
    HN,
    ///Croatia
    HR,
    ///Haiti
    HT,
    ///Hungary
    HU,
    ///Indonesia
    ID,
    ///Ireland
    IE,
    ///Israel
    IL,
    ///Isle of Man
    IM,
    ///India
    IN,
    ///British Indian Ocean Territory
    IO,
    ///Iraq
    IQ,
    ///Iran, Islamic Republic of
    IR,
    ///Iceland
    IS,
    ///Italy
    IT,
    ///Jersey
    JE,
    ///Jamaica
    JM,
    ///Jordan
    JO,
    ///Japan
    JP,
    ///Kenya
    KE,
    ///Kyrgyzstan
    KG,
    ///Cambodia
    KH,
    ///Kiribati
    KI,
    ///Comoros
    KM,
    ///Saint Kitts and Nevis
    KN,
    ///Korea, Democratic People's Republic of
    KP,
    ///Korea, Republic of
    KR,
    ///Kuwait
    KW,
    ///Cayman Islands
    KY,
    ///Kazakhstan
    KZ,
    ///Lao People's Democratic Republic
    LA,
    ///Lebanon
    LB,
    ///Saint Lucia
    LC,
    ///Liechtenstein
    LI,
    ///Sri Lanka
    LK,
    ///Liberia
    LR,
    ///Lesotho
    LS,
    ///Lithuania
    LT,
    ///Luxembourg
    LU,
    ///Latvia
    LV,
    ///Libya
    LY,
    ///Morocco
    MA,
    ///Monaco
    MC,
    ///Moldova, Republic of
    MD,
    ///Montenegro
    ME,
    ///Saint Martin (French part)
    MF,
    ///Madagascar
    MG,
    ///Marshall Islands
    MH,
    ///Macedonia, the former Yugoslav Republic of
    MK,
    ///Mali
    ML,
    ///Myanmar
    MM,
    ///Mongolia
    MN,
    ///Macao
    MO,
    ///Northern Mariana Islands
    MP,
    ///Martinique
    MQ,
    ///Mauritania
    MR,
    ///Montserrat
    MS,
    ///Malta
    MT,
    ///Mauritius
    MU,
    ///Maldives
    MV,
    ///Malawi
    MW,
    ///Mexico
    MX,
    ///Malaysia
    MY,
    ///Mozambique
    MZ,
    ///Namibia
    NA,
    ///New Caledonia
    NC,
    ///Niger
    NE,
    ///Norfolk Island
    NF,
    ///Nigeria
    NG,
    ///Nicaragua
    NI,
    ///Netherlands
    NL,
    ///Norway
    NO,
    ///Nepal
    NP,
    ///Nauru
    NR,
    ///Niue
    NU,
    ///New Zealand
    NZ,
    ///Oman
    OM,
    ///Panama
    PA,
    ///Peru
    PE,
    ///French Polynesia
    PF,
    ///Papua New Guinea
    PG,
    ///Philippines
    PH,
    ///Pakistan
    PK,
    ///Poland
    PL,
    ///Saint Pierre and Miquelon
    PM,
    ///Pitcairn
    PN,
    ///Puerto Rico
    PR,
    ///Palestine, State of
    PS,
    ///Portugal
    PT,
    ///Palau
    PW,
    ///Paraguay
    PY,
    ///Qatar
    QA,
    ///Réunion
    RE,
    ///Romania
    RO,
    ///Serbia
    RS,
    ///Russian Federation
    RU,
    ///Rwanda
    RW,
    ///Saudi Arabia
    SA,
    ///Solomon Islands
    SB,
    ///Seychelles
    SC,
    ///Sudan
    SD,
    ///Sweden
    SE,
    ///Singapore
    SG,
    ///Saint Helena, Ascension and Tristan da Cunha
    SH,
    ///Slovenia
    SI,
    ///Svalbard and Jan Mayen
    SJ,
    ///Slovakia
    SK,
    ///Sierra Leone
    SL,
    ///San Marino
    SM,
    ///Senegal
    SN,
    ///Somalia
    SO,
    ///Suriname
    SR,
    ///South Sudan
    SS,
    ///Sao Tome and Principe
    ST,
    ///El Salvador
    SV,
    ///Sint Maarten (Dutch part)
    SX,
    ///Syrian Arab Republic
    SY,
    ///Swaziland
    SZ,
    ///Turks and Caicos Islands
    TC,
    ///Chad
    TD,
    ///French Southern Territories
    TF,
    ///Togo
    TG,
    ///Thailand
    TH,
    ///Tajikistan
    TJ,
    ///Tokelau
    TK,
    ///Timor-Leste
    TL,
    ///Turkmenistan
    TM,
    ///Tunisia
    TN,
    ///Tonga
    TO,
    ///Turkey
    TR,
    ///Trinidad and Tobago
    TT,
    ///Tuvalu
    TV,
    ///Taiwan, Province of China
    TW,
    ///Tanzania, United Republic of
    TZ,
    ///Ukraine
    UA,
    ///Uganda
    UG,
    ///United States Minor Outlying Islands
    UM,
    ///United States of America
    US,
    ///Uruguay
    UY,
    ///Uzbekistan
    UZ,
    ///Holy See
    VA,
    ///Saint Vincent and the Grenadines
    VC,
    ///Venezuela, Bolivarian Republic of
    VE,
    ///Virgin Islands, British
    VG,
    ///Virgin Islands, U.S.
    VI,
    ///Viet Nam
    VN,
    ///Vanuatu
    VU,
    ///Wallis and Futuna
    WF,
    ///Samoa
    WS,
    ///Yemen
    YE,
    ///Mayotte
    YT,
    ///South Africa
    ZA,
    ///Zambia
    ZM,
    ///Zimbabwe
    ZW,
}


pub fn decode_country(msg: &[u8; 2]) -> Result<Country> {
    match msg {
        b"AD" => Ok(Country::AD),
        b"AE" => Ok(Country::AE),
        b"AF" => Ok(Country::AF),
        b"AG" => Ok(Country::AG),
        b"AI" => Ok(Country::AI),
        b"AL" => Ok(Country::AL),
        b"AM" => Ok(Country::AM),
        b"AO" => Ok(Country::AO),
        b"AQ" => Ok(Country::AQ),
        b"AR" => Ok(Country::AR),
        b"AS" => Ok(Country::AS),
        b"AT" => Ok(Country::AT),
        b"AU" => Ok(Country::AU),
        b"AW" => Ok(Country::AW),
        b"AX" => Ok(Country::AX),
        b"AZ" => Ok(Country::AZ),
        b"BA" => Ok(Country::BA),
        b"BB" => Ok(Country::BB),
        b"BD" => Ok(Country::BD),
        b"BE" => Ok(Country::BE),
        b"BF" => Ok(Country::BF),
        b"BG" => Ok(Country::BG),
        b"BH" => Ok(Country::BH),
        b"BI" => Ok(Country::BI),
        b"BJ" => Ok(Country::BJ),
        b"BL" => Ok(Country::BL),
        b"BM" => Ok(Country::BM),
        b"BN" => Ok(Country::BN),
        b"BO" => Ok(Country::BO),
        b"BQ" => Ok(Country::BQ),
        b"BR" => Ok(Country::BR),
        b"BS" => Ok(Country::BS),
        b"BT" => Ok(Country::BT),
        b"BV" => Ok(Country::BV),
        b"BW" => Ok(Country::BW),
        b"BY" => Ok(Country::BY),
        b"BZ" => Ok(Country::BZ),
        b"CA" => Ok(Country::CA),
        b"CC" => Ok(Country::CC),
        b"CD" => Ok(Country::CD),
        b"CF" => Ok(Country::CF),
        b"CG" => Ok(Country::CG),
        b"CH" => Ok(Country::CH),
        b"CI" => Ok(Country::CI),
        b"CK" => Ok(Country::CK),
        b"CL" => Ok(Country::CL),
        b"CM" => Ok(Country::CM),
        b"CN" => Ok(Country::CN),
        b"CO" => Ok(Country::CO),
        b"CR" => Ok(Country::CR),
        b"CU" => Ok(Country::CU),
        b"CV" => Ok(Country::CV),
        b"CW" => Ok(Country::CW),
        b"CX" => Ok(Country::CX),
        b"CY" => Ok(Country::CY),
        b"CZ" => Ok(Country::CZ),
        b"DE" => Ok(Country::DE),
        b"DJ" => Ok(Country::DJ),
        b"DK" => Ok(Country::DK),
        b"DM" => Ok(Country::DM),
        b"DO" => Ok(Country::DO),
        b"DZ" => Ok(Country::DZ),
        b"EC" => Ok(Country::EC),
        b"EE" => Ok(Country::EE),
        b"EG" => Ok(Country::EG),
        b"EH" => Ok(Country::EH),
        b"ER" => Ok(Country::ER),
        b"ES" => Ok(Country::ES),
        b"ET" => Ok(Country::ET),
        b"FI" => Ok(Country::FI),
        b"FJ" => Ok(Country::FJ),
        b"FK" => Ok(Country::FK),
        b"FM" => Ok(Country::FM),
        b"FO" => Ok(Country::FO),
        b"FR" => Ok(Country::FR),
        b"GA" => Ok(Country::GA),
        b"GB" => Ok(Country::GB),
        b"GD" => Ok(Country::GD),
        b"GE" => Ok(Country::GE),
        b"GF" => Ok(Country::GF),
        b"GG" => Ok(Country::GG),
        b"GH" => Ok(Country::GH),
        b"GI" => Ok(Country::GI),
        b"GL" => Ok(Country::GL),
        b"GM" => Ok(Country::GM),
        b"GN" => Ok(Country::GN),
        b"GP" => Ok(Country::GP),
        b"GQ" => Ok(Country::GQ),
        b"GR" => Ok(Country::GR),
        b"GS" => Ok(Country::GS),
        b"GT" => Ok(Country::GT),
        b"GU" => Ok(Country::GU),
        b"GW" => Ok(Country::GW),
        b"GY" => Ok(Country::GY),
        b"HK" => Ok(Country::HK),
        b"HM" => Ok(Country::HM),
        b"HN" => Ok(Country::HN),
        b"HR" => Ok(Country::HR),
        b"HT" => Ok(Country::HT),
        b"HU" => Ok(Country::HU),
        b"ID" => Ok(Country::ID),
        b"IE" => Ok(Country::IE),
        b"IL" => Ok(Country::IL),
        b"IM" => Ok(Country::IM),
        b"IN" => Ok(Country::IN),
        b"IO" => Ok(Country::IO),
        b"IQ" => Ok(Country::IQ),
        b"IR" => Ok(Country::IR),
        b"IS" => Ok(Country::IS),
        b"IT" => Ok(Country::IT),
        b"JE" => Ok(Country::JE),
        b"JM" => Ok(Country::JM),
        b"JO" => Ok(Country::JO),
        b"JP" => Ok(Country::JP),
        b"KE" => Ok(Country::KE),
        b"KG" => Ok(Country::KG),
        b"KH" => Ok(Country::KH),
        b"KI" => Ok(Country::KI),
        b"KM" => Ok(Country::KM),
        b"KN" => Ok(Country::KN),
        b"KP" => Ok(Country::KP),
        b"KR" => Ok(Country::KR),
        b"KW" => Ok(Country::KW),
        b"KY" => Ok(Country::KY),
        b"KZ" => Ok(Country::KZ),
        b"LA" => Ok(Country::LA),
        b"LB" => Ok(Country::LB),
        b"LC" => Ok(Country::LC),
        b"LI" => Ok(Country::LI),
        b"LK" => Ok(Country::LK),
        b"LR" => Ok(Country::LR),
        b"LS" => Ok(Country::LS),
        b"LT" => Ok(Country::LT),
        b"LU" => Ok(Country::LU),
        b"LV" => Ok(Country::LV),
        b"LY" => Ok(Country::LY),
        b"MA" => Ok(Country::MA),
        b"MC" => Ok(Country::MC),
        b"MD" => Ok(Country::MD),
        b"ME" => Ok(Country::ME),
        b"MF" => Ok(Country::MF),
        b"MG" => Ok(Country::MG),
        b"MH" => Ok(Country::MH),
        b"MK" => Ok(Country::MK),
        b"ML" => Ok(Country::ML),
        b"MM" => Ok(Country::MM),
        b"MN" => Ok(Country::MN),
        b"MO" => Ok(Country::MO),
        b"MP" => Ok(Country::MP),
        b"MQ" => Ok(Country::MQ),
        b"MR" => Ok(Country::MR),
        b"MS" => Ok(Country::MS),
        b"MT" => Ok(Country::MT),
        b"MU" => Ok(Country::MU),
        b"MV" => Ok(Country::MV),
        b"MW" => Ok(Country::MW),
        b"MX" => Ok(Country::MX),
        b"MY" => Ok(Country::MY),
        b"MZ" => Ok(Country::MZ),
        b"NA" => Ok(Country::NA),
        b"NC" => Ok(Country::NC),
        b"NE" => Ok(Country::NE),
        b"NF" => Ok(Country::NF),
        b"NG" => Ok(Country::NG),
        b"NI" => Ok(Country::NI),
        b"NL" => Ok(Country::NL),
        b"NO" => Ok(Country::NO),
        b"NP" => Ok(Country::NP),
        b"NR" => Ok(Country::NR),
        b"NU" => Ok(Country::NU),
        b"NZ" => Ok(Country::NZ),
        b"OM" => Ok(Country::OM),
        b"PA" => Ok(Country::PA),
        b"PE" => Ok(Country::PE),
        b"PF" => Ok(Country::PF),
        b"PG" => Ok(Country::PG),
        b"PH" => Ok(Country::PH),
        b"PK" => Ok(Country::PK),
        b"PL" => Ok(Country::PL),
        b"PM" => Ok(Country::PM),
        b"PN" => Ok(Country::PN),
        b"PR" => Ok(Country::PR),
        b"PS" => Ok(Country::PS),
        b"PT" => Ok(Country::PT),
        b"PW" => Ok(Country::PW),
        b"PY" => Ok(Country::PY),
        b"QA" => Ok(Country::QA),
        b"RE" => Ok(Country::RE),
        b"RO" => Ok(Country::RO),
        b"RS" => Ok(Country::RS),
        b"RU" => Ok(Country::RU),
        b"RW" => Ok(Country::RW),
        b"SA" => Ok(Country::SA),
        b"SB" => Ok(Country::SB),
        b"SC" => Ok(Country::SC),
        b"SD" => Ok(Country::SD),
        b"SE" => Ok(Country::SE),
        b"SG" => Ok(Country::SG),
        b"SH" => Ok(Country::SH),
        b"SI" => Ok(Country::SI),
        b"SJ" => Ok(Country::SJ),
        b"SK" => Ok(Country::SK),
        b"SL" => Ok(Country::SL),
        b"SM" => Ok(Country::SM),
        b"SN" => Ok(Country::SN),
        b"SO" => Ok(Country::SO),
        b"SR" => Ok(Country::SR),
        b"SS" => Ok(Country::SS),
        b"ST" => Ok(Country::ST),
        b"SV" => Ok(Country::SV),
        b"SX" => Ok(Country::SX),
        b"SY" => Ok(Country::SY),
        b"SZ" => Ok(Country::SZ),
        b"TC" => Ok(Country::TC),
        b"TD" => Ok(Country::TD),
        b"TF" => Ok(Country::TF),
        b"TG" => Ok(Country::TG),
        b"TH" => Ok(Country::TH),
        b"TJ" => Ok(Country::TJ),
        b"TK" => Ok(Country::TK),
        b"TL" => Ok(Country::TL),
        b"TM" => Ok(Country::TM),
        b"TN" => Ok(Country::TN),
        b"TO" => Ok(Country::TO),
        b"TR" => Ok(Country::TR),
        b"TT" => Ok(Country::TT),
        b"TV" => Ok(Country::TV),
        b"TW" => Ok(Country::TW),
        b"TZ" => Ok(Country::TZ),
        b"UA" => Ok(Country::UA),
        b"UG" => Ok(Country::UG),
        b"UM" => Ok(Country::UM),
        b"US" => Ok(Country::US),
        b"UY" => Ok(Country::UY),
        b"UZ" => Ok(Country::UZ),
        b"VA" => Ok(Country::VA),
        b"VC" => Ok(Country::VC),
        b"VE" => Ok(Country::VE),
        b"VG" => Ok(Country::VG),
        b"VI" => Ok(Country::VI),
        b"VN" => Ok(Country::VN),
        b"VU" => Ok(Country::VU),
        b"WF" => Ok(Country::WF),
        b"WS" => Ok(Country::WS),
        b"YE" => Ok(Country::YE),
        b"YT" => Ok(Country::YT),
        b"ZA" => Ok(Country::ZA),
        b"ZM" => Ok(Country::ZM),
        b"ZW" => Ok(Country::ZW),
        _ => Err(DecodeError::Overflow)
    }
}

pub fn encode_country(c: Country) -> [u8; 2] {
    match c {
        Country::AD => *b"AD",
        Country::AE => *b"AE",
        Country::AF => *b"AF",
        Country::AG => *b"AG",
        Country::AI => *b"AI",
        Country::AL => *b"AL",
        Country::AM => *b"AM",
        Country::AO => *b"AO",
        Country::AQ => *b"AQ",
        Country::AR => *b"AR",
        Country::AS => *b"AS",
        Country::AT => *b"AT",
        Country::AU => *b"AU",
        Country::AW => *b"AW",
        Country::AX => *b"AX",
        Country::AZ => *b"AZ",
        Country::BA => *b"BA",
        Country::BB => *b"BB",
        Country::BD => *b"BD",
        Country::BE => *b"BE",
        Country::BF => *b"BF",
        Country::BG => *b"BG",
        Country::BH => *b"BH",
        Country::BI => *b"BI",
        Country::BJ => *b"BJ",
        Country::BL => *b"BL",
        Country::BM => *b"BM",
        Country::BN => *b"BN",
        Country::BO => *b"BO",
        Country::BQ => *b"BQ",
        Country::BR => *b"BR",
        Country::BS => *b"BS",
        Country::BT => *b"BT",
        Country::BV => *b"BV",
        Country::BW => *b"BW",
        Country::BY => *b"BY",
        Country::BZ => *b"BZ",
        Country::CA => *b"CA",
        Country::CC => *b"CC",
        Country::CD => *b"CD",
        Country::CF => *b"CF",
        Country::CG => *b"CG",
        Country::CH => *b"CH",
        Country::CI => *b"CI",
        Country::CK => *b"CK",
        Country::CL => *b"CL",
        Country::CM => *b"CM",
        Country::CN => *b"CN",
        Country::CO => *b"CO",
        Country::CR => *b"CR",
        Country::CU => *b"CU",
        Country::CV => *b"CV",
        Country::CW => *b"CW",
        Country::CX => *b"CX",
        Country::CY => *b"CY",
        Country::CZ => *b"CZ",
        Country::DE => *b"DE",
        Country::DJ => *b"DJ",
        Country::DK => *b"DK",
        Country::DM => *b"DM",
        Country::DO => *b"DO",
        Country::DZ => *b"DZ",
        Country::EC => *b"EC",
        Country::EE => *b"EE",
        Country::EG => *b"EG",
        Country::EH => *b"EH",
        Country::ER => *b"ER",
        Country::ES => *b"ES",
        Country::ET => *b"ET",
        Country::FI => *b"FI",
        Country::FJ => *b"FJ",
        Country::FK => *b"FK",
        Country::FM => *b"FM",
        Country::FO => *b"FO",
        Country::FR => *b"FR",
        Country::GA => *b"GA",
        Country::GB => *b"GB",
        Country::GD => *b"GD",
        Country::GE => *b"GE",
        Country::GF => *b"GF",
        Country::GG => *b"GG",
        Country::GH => *b"GH",
        Country::GI => *b"GI",
        Country::GL => *b"GL",
        Country::GM => *b"GM",
        Country::GN => *b"GN",
        Country::GP => *b"GP",
        Country::GQ => *b"GQ",
        Country::GR => *b"GR",
        Country::GS => *b"GS",
        Country::GT => *b"GT",
        Country::GU => *b"GU",
        Country::GW => *b"GW",
        Country::GY => *b"GY",
        Country::HK => *b"HK",
        Country::HM => *b"HM",
        Country::HN => *b"HN",
        Country::HR => *b"HR",
        Country::HT => *b"HT",
        Country::HU => *b"HU",
        Country::ID => *b"ID",
        Country::IE => *b"IE",
        Country::IL => *b"IL",
        Country::IM => *b"IM",
        Country::IN => *b"IN",
        Country::IO => *b"IO",
        Country::IQ => *b"IQ",
        Country::IR => *b"IR",
        Country::IS => *b"IS",
        Country::IT => *b"IT",
        Country::JE => *b"JE",
        Country::JM => *b"JM",
        Country::JO => *b"JO",
        Country::JP => *b"JP",
        Country::KE => *b"KE",
        Country::KG => *b"KG",
        Country::KH => *b"KH",
        Country::KI => *b"KI",
        Country::KM => *b"KM",
        Country::KN => *b"KN",
        Country::KP => *b"KP",
        Country::KR => *b"KR",
        Country::KW => *b"KW",
        Country::KY => *b"KY",
        Country::KZ => *b"KZ",
        Country::LA => *b"LA",
        Country::LB => *b"LB",
        Country::LC => *b"LC",
        Country::LI => *b"LI",
        Country::LK => *b"LK",
        Country::LR => *b"LR",
        Country::LS => *b"LS",
        Country::LT => *b"LT",
        Country::LU => *b"LU",
        Country::LV => *b"LV",
        Country::LY => *b"LY",
        Country::MA => *b"MA",
        Country::MC => *b"MC",
        Country::MD => *b"MD",
        Country::ME => *b"ME",
        Country::MF => *b"MF",
        Country::MG => *b"MG",
        Country::MH => *b"MH",
        Country::MK => *b"MK",
        Country::ML => *b"ML",
        Country::MM => *b"MM",
        Country::MN => *b"MN",
        Country::MO => *b"MO",
        Country::MP => *b"MP",
        Country::MQ => *b"MQ",
        Country::MR => *b"MR",
        Country::MS => *b"MS",
        Country::MT => *b"MT",
        Country::MU => *b"MU",
        Country::MV => *b"MV",
        Country::MW => *b"MW",
        Country::MX => *b"MX",
        Country::MY => *b"MY",
        Country::MZ => *b"MZ",
        Country::NA => *b"NA",
        Country::NC => *b"NC",
        Country::NE => *b"NE",
        Country::NF => *b"NF",
        Country::NG => *b"NG",
        Country::NI => *b"NI",
        Country::NL => *b"NL",
        Country::NO => *b"NO",
        Country::NP => *b"NP",
        Country::NR => *b"NR",
        Country::NU => *b"NU",
        Country::NZ => *b"NZ",
        Country::OM => *b"OM",
        Country::PA => *b"PA",
        Country::PE => *b"PE",
        Country::PF => *b"PF",
        Country::PG => *b"PG",
        Country::PH => *b"PH",
        Country::PK => *b"PK",
        Country::PL => *b"PL",
        Country::PM => *b"PM",
        Country::PN => *b"PN",
        Country::PR => *b"PR",
        Country::PS => *b"PS",
        Country::PT => *b"PT",
        Country::PW => *b"PW",
        Country::PY => *b"PY",
        Country::QA => *b"QA",
        Country::RE => *b"RE",
        Country::RO => *b"RO",
        Country::RS => *b"RS",
        Country::RU => *b"RU",
        Country::RW => *b"RW",
        Country::SA => *b"SA",
        Country::SB => *b"SB",
        Country::SC => *b"SC",
        Country::SD => *b"SD",
        Country::SE => *b"SE",
        Country::SG => *b"SG",
        Country::SH => *b"SH",
        Country::SI => *b"SI",
        Country::SJ => *b"SJ",
        Country::SK => *b"SK",
        Country::SL => *b"SL",
        Country::SM => *b"SM",
        Country::SN => *b"SN",
        Country::SO => *b"SO",
        Country::SR => *b"SR",
        Country::SS => *b"SS",
        Country::ST => *b"ST",
        Country::SV => *b"SV",
        Country::SX => *b"SX",
        Country::SY => *b"SY",
        Country::SZ => *b"SZ",
        Country::TC => *b"TC",
        Country::TD => *b"TD",
        Country::TF => *b"TF",
        Country::TG => *b"TG",
        Country::TH => *b"TH",
        Country::TJ => *b"TJ",
        Country::TK => *b"TK",
        Country::TL => *b"TL",
        Country::TM => *b"TM",
        Country::TN => *b"TN",
        Country::TO => *b"TO",
        Country::TR => *b"TR",
        Country::TT => *b"TT",
        Country::TV => *b"TV",
        Country::TW => *b"TW",
        Country::TZ => *b"TZ",
        Country::UA => *b"UA",
        Country::UG => *b"UG",
        Country::UM => *b"UM",
        Country::US => *b"US",
        Country::UY => *b"UY",
        Country::UZ => *b"UZ",
        Country::VA => *b"VA",
        Country::VC => *b"VC",
        Country::VE => *b"VE",
        Country::VG => *b"VG",
        Country::VI => *b"VI",
        Country::VN => *b"VN",
        Country::VU => *b"VU",
        Country::WF => *b"WF",
        Country::WS => *b"WS",
        Country::YE => *b"YE",
        Country::YT => *b"YT",
        Country::ZA => *b"ZA",
        Country::ZM => *b"ZM",
        Country::ZW => *b"ZW",
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_country() {
        assert_eq!(decode_country(b"US").unwrap(), Country::US);
        assert!(decode_country(b"UX").is_err());
    }

    #[test]
    fn test_encode_country() {
        assert_eq!(encode_country(Country::US), *b"US");
        assert_eq!(encode_country(Country::ZW), *b"ZW");
    }
}
