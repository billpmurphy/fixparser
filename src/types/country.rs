use protocol::FIXValue;


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

impl FIXValue for Country {
    fn from_bytes(bytes: &[u8]) -> Option<Country> {
        match bytes {
            b"AD" => Some(Country::AD),
            b"AE" => Some(Country::AE),
            b"AF" => Some(Country::AF),
            b"AG" => Some(Country::AG),
            b"AI" => Some(Country::AI),
            b"AL" => Some(Country::AL),
            b"AM" => Some(Country::AM),
            b"AO" => Some(Country::AO),
            b"AQ" => Some(Country::AQ),
            b"AR" => Some(Country::AR),
            b"AS" => Some(Country::AS),
            b"AT" => Some(Country::AT),
            b"AU" => Some(Country::AU),
            b"AW" => Some(Country::AW),
            b"AX" => Some(Country::AX),
            b"AZ" => Some(Country::AZ),
            b"BA" => Some(Country::BA),
            b"BB" => Some(Country::BB),
            b"BD" => Some(Country::BD),
            b"BE" => Some(Country::BE),
            b"BF" => Some(Country::BF),
            b"BG" => Some(Country::BG),
            b"BH" => Some(Country::BH),
            b"BI" => Some(Country::BI),
            b"BJ" => Some(Country::BJ),
            b"BL" => Some(Country::BL),
            b"BM" => Some(Country::BM),
            b"BN" => Some(Country::BN),
            b"BO" => Some(Country::BO),
            b"BQ" => Some(Country::BQ),
            b"BR" => Some(Country::BR),
            b"BS" => Some(Country::BS),
            b"BT" => Some(Country::BT),
            b"BV" => Some(Country::BV),
            b"BW" => Some(Country::BW),
            b"BY" => Some(Country::BY),
            b"BZ" => Some(Country::BZ),
            b"CA" => Some(Country::CA),
            b"CC" => Some(Country::CC),
            b"CD" => Some(Country::CD),
            b"CF" => Some(Country::CF),
            b"CG" => Some(Country::CG),
            b"CH" => Some(Country::CH),
            b"CI" => Some(Country::CI),
            b"CK" => Some(Country::CK),
            b"CL" => Some(Country::CL),
            b"CM" => Some(Country::CM),
            b"CN" => Some(Country::CN),
            b"CO" => Some(Country::CO),
            b"CR" => Some(Country::CR),
            b"CU" => Some(Country::CU),
            b"CV" => Some(Country::CV),
            b"CW" => Some(Country::CW),
            b"CX" => Some(Country::CX),
            b"CY" => Some(Country::CY),
            b"CZ" => Some(Country::CZ),
            b"DE" => Some(Country::DE),
            b"DJ" => Some(Country::DJ),
            b"DK" => Some(Country::DK),
            b"DM" => Some(Country::DM),
            b"DO" => Some(Country::DO),
            b"DZ" => Some(Country::DZ),
            b"EC" => Some(Country::EC),
            b"EE" => Some(Country::EE),
            b"EG" => Some(Country::EG),
            b"EH" => Some(Country::EH),
            b"ER" => Some(Country::ER),
            b"ES" => Some(Country::ES),
            b"ET" => Some(Country::ET),
            b"FI" => Some(Country::FI),
            b"FJ" => Some(Country::FJ),
            b"FK" => Some(Country::FK),
            b"FM" => Some(Country::FM),
            b"FO" => Some(Country::FO),
            b"FR" => Some(Country::FR),
            b"GA" => Some(Country::GA),
            b"GB" => Some(Country::GB),
            b"GD" => Some(Country::GD),
            b"GE" => Some(Country::GE),
            b"GF" => Some(Country::GF),
            b"GG" => Some(Country::GG),
            b"GH" => Some(Country::GH),
            b"GI" => Some(Country::GI),
            b"GL" => Some(Country::GL),
            b"GM" => Some(Country::GM),
            b"GN" => Some(Country::GN),
            b"GP" => Some(Country::GP),
            b"GQ" => Some(Country::GQ),
            b"GR" => Some(Country::GR),
            b"GS" => Some(Country::GS),
            b"GT" => Some(Country::GT),
            b"GU" => Some(Country::GU),
            b"GW" => Some(Country::GW),
            b"GY" => Some(Country::GY),
            b"HK" => Some(Country::HK),
            b"HM" => Some(Country::HM),
            b"HN" => Some(Country::HN),
            b"HR" => Some(Country::HR),
            b"HT" => Some(Country::HT),
            b"HU" => Some(Country::HU),
            b"ID" => Some(Country::ID),
            b"IE" => Some(Country::IE),
            b"IL" => Some(Country::IL),
            b"IM" => Some(Country::IM),
            b"IN" => Some(Country::IN),
            b"IO" => Some(Country::IO),
            b"IQ" => Some(Country::IQ),
            b"IR" => Some(Country::IR),
            b"IS" => Some(Country::IS),
            b"IT" => Some(Country::IT),
            b"JE" => Some(Country::JE),
            b"JM" => Some(Country::JM),
            b"JO" => Some(Country::JO),
            b"JP" => Some(Country::JP),
            b"KE" => Some(Country::KE),
            b"KG" => Some(Country::KG),
            b"KH" => Some(Country::KH),
            b"KI" => Some(Country::KI),
            b"KM" => Some(Country::KM),
            b"KN" => Some(Country::KN),
            b"KP" => Some(Country::KP),
            b"KR" => Some(Country::KR),
            b"KW" => Some(Country::KW),
            b"KY" => Some(Country::KY),
            b"KZ" => Some(Country::KZ),
            b"LA" => Some(Country::LA),
            b"LB" => Some(Country::LB),
            b"LC" => Some(Country::LC),
            b"LI" => Some(Country::LI),
            b"LK" => Some(Country::LK),
            b"LR" => Some(Country::LR),
            b"LS" => Some(Country::LS),
            b"LT" => Some(Country::LT),
            b"LU" => Some(Country::LU),
            b"LV" => Some(Country::LV),
            b"LY" => Some(Country::LY),
            b"MA" => Some(Country::MA),
            b"MC" => Some(Country::MC),
            b"MD" => Some(Country::MD),
            b"ME" => Some(Country::ME),
            b"MF" => Some(Country::MF),
            b"MG" => Some(Country::MG),
            b"MH" => Some(Country::MH),
            b"MK" => Some(Country::MK),
            b"ML" => Some(Country::ML),
            b"MM" => Some(Country::MM),
            b"MN" => Some(Country::MN),
            b"MO" => Some(Country::MO),
            b"MP" => Some(Country::MP),
            b"MQ" => Some(Country::MQ),
            b"MR" => Some(Country::MR),
            b"MS" => Some(Country::MS),
            b"MT" => Some(Country::MT),
            b"MU" => Some(Country::MU),
            b"MV" => Some(Country::MV),
            b"MW" => Some(Country::MW),
            b"MX" => Some(Country::MX),
            b"MY" => Some(Country::MY),
            b"MZ" => Some(Country::MZ),
            b"NA" => Some(Country::NA),
            b"NC" => Some(Country::NC),
            b"NE" => Some(Country::NE),
            b"NF" => Some(Country::NF),
            b"NG" => Some(Country::NG),
            b"NI" => Some(Country::NI),
            b"NL" => Some(Country::NL),
            b"NO" => Some(Country::NO),
            b"NP" => Some(Country::NP),
            b"NR" => Some(Country::NR),
            b"NU" => Some(Country::NU),
            b"NZ" => Some(Country::NZ),
            b"OM" => Some(Country::OM),
            b"PA" => Some(Country::PA),
            b"PE" => Some(Country::PE),
            b"PF" => Some(Country::PF),
            b"PG" => Some(Country::PG),
            b"PH" => Some(Country::PH),
            b"PK" => Some(Country::PK),
            b"PL" => Some(Country::PL),
            b"PM" => Some(Country::PM),
            b"PN" => Some(Country::PN),
            b"PR" => Some(Country::PR),
            b"PS" => Some(Country::PS),
            b"PT" => Some(Country::PT),
            b"PW" => Some(Country::PW),
            b"PY" => Some(Country::PY),
            b"QA" => Some(Country::QA),
            b"RE" => Some(Country::RE),
            b"RO" => Some(Country::RO),
            b"RS" => Some(Country::RS),
            b"RU" => Some(Country::RU),
            b"RW" => Some(Country::RW),
            b"SA" => Some(Country::SA),
            b"SB" => Some(Country::SB),
            b"SC" => Some(Country::SC),
            b"SD" => Some(Country::SD),
            b"SE" => Some(Country::SE),
            b"SG" => Some(Country::SG),
            b"SH" => Some(Country::SH),
            b"SI" => Some(Country::SI),
            b"SJ" => Some(Country::SJ),
            b"SK" => Some(Country::SK),
            b"SL" => Some(Country::SL),
            b"SM" => Some(Country::SM),
            b"SN" => Some(Country::SN),
            b"SO" => Some(Country::SO),
            b"SR" => Some(Country::SR),
            b"SS" => Some(Country::SS),
            b"ST" => Some(Country::ST),
            b"SV" => Some(Country::SV),
            b"SX" => Some(Country::SX),
            b"SY" => Some(Country::SY),
            b"SZ" => Some(Country::SZ),
            b"TC" => Some(Country::TC),
            b"TD" => Some(Country::TD),
            b"TF" => Some(Country::TF),
            b"TG" => Some(Country::TG),
            b"TH" => Some(Country::TH),
            b"TJ" => Some(Country::TJ),
            b"TK" => Some(Country::TK),
            b"TL" => Some(Country::TL),
            b"TM" => Some(Country::TM),
            b"TN" => Some(Country::TN),
            b"TO" => Some(Country::TO),
            b"TR" => Some(Country::TR),
            b"TT" => Some(Country::TT),
            b"TV" => Some(Country::TV),
            b"TW" => Some(Country::TW),
            b"TZ" => Some(Country::TZ),
            b"UA" => Some(Country::UA),
            b"UG" => Some(Country::UG),
            b"UM" => Some(Country::UM),
            b"US" => Some(Country::US),
            b"UY" => Some(Country::UY),
            b"UZ" => Some(Country::UZ),
            b"VA" => Some(Country::VA),
            b"VC" => Some(Country::VC),
            b"VE" => Some(Country::VE),
            b"VG" => Some(Country::VG),
            b"VI" => Some(Country::VI),
            b"VN" => Some(Country::VN),
            b"VU" => Some(Country::VU),
            b"WF" => Some(Country::WF),
            b"WS" => Some(Country::WS),
            b"YE" => Some(Country::YE),
            b"YT" => Some(Country::YT),
            b"ZA" => Some(Country::ZA),
            b"ZM" => Some(Country::ZM),
            b"ZW" => Some(Country::ZW),
            _ => None
        }
    }

    fn to_bytes(&self, v: &mut Vec<u8>) {
        match *self {
            Country::AD => v.extend(b"AD"),
            Country::AE => v.extend(b"AE"),
            Country::AF => v.extend(b"AF"),
            Country::AG => v.extend(b"AG"),
            Country::AI => v.extend(b"AI"),
            Country::AL => v.extend(b"AL"),
            Country::AM => v.extend(b"AM"),
            Country::AO => v.extend(b"AO"),
            Country::AQ => v.extend(b"AQ"),
            Country::AR => v.extend(b"AR"),
            Country::AS => v.extend(b"AS"),
            Country::AT => v.extend(b"AT"),
            Country::AU => v.extend(b"AU"),
            Country::AW => v.extend(b"AW"),
            Country::AX => v.extend(b"AX"),
            Country::AZ => v.extend(b"AZ"),
            Country::BA => v.extend(b"BA"),
            Country::BB => v.extend(b"BB"),
            Country::BD => v.extend(b"BD"),
            Country::BE => v.extend(b"BE"),
            Country::BF => v.extend(b"BF"),
            Country::BG => v.extend(b"BG"),
            Country::BH => v.extend(b"BH"),
            Country::BI => v.extend(b"BI"),
            Country::BJ => v.extend(b"BJ"),
            Country::BL => v.extend(b"BL"),
            Country::BM => v.extend(b"BM"),
            Country::BN => v.extend(b"BN"),
            Country::BO => v.extend(b"BO"),
            Country::BQ => v.extend(b"BQ"),
            Country::BR => v.extend(b"BR"),
            Country::BS => v.extend(b"BS"),
            Country::BT => v.extend(b"BT"),
            Country::BV => v.extend(b"BV"),
            Country::BW => v.extend(b"BW"),
            Country::BY => v.extend(b"BY"),
            Country::BZ => v.extend(b"BZ"),
            Country::CA => v.extend(b"CA"),
            Country::CC => v.extend(b"CC"),
            Country::CD => v.extend(b"CD"),
            Country::CF => v.extend(b"CF"),
            Country::CG => v.extend(b"CG"),
            Country::CH => v.extend(b"CH"),
            Country::CI => v.extend(b"CI"),
            Country::CK => v.extend(b"CK"),
            Country::CL => v.extend(b"CL"),
            Country::CM => v.extend(b"CM"),
            Country::CN => v.extend(b"CN"),
            Country::CO => v.extend(b"CO"),
            Country::CR => v.extend(b"CR"),
            Country::CU => v.extend(b"CU"),
            Country::CV => v.extend(b"CV"),
            Country::CW => v.extend(b"CW"),
            Country::CX => v.extend(b"CX"),
            Country::CY => v.extend(b"CY"),
            Country::CZ => v.extend(b"CZ"),
            Country::DE => v.extend(b"DE"),
            Country::DJ => v.extend(b"DJ"),
            Country::DK => v.extend(b"DK"),
            Country::DM => v.extend(b"DM"),
            Country::DO => v.extend(b"DO"),
            Country::DZ => v.extend(b"DZ"),
            Country::EC => v.extend(b"EC"),
            Country::EE => v.extend(b"EE"),
            Country::EG => v.extend(b"EG"),
            Country::EH => v.extend(b"EH"),
            Country::ER => v.extend(b"ER"),
            Country::ES => v.extend(b"ES"),
            Country::ET => v.extend(b"ET"),
            Country::FI => v.extend(b"FI"),
            Country::FJ => v.extend(b"FJ"),
            Country::FK => v.extend(b"FK"),
            Country::FM => v.extend(b"FM"),
            Country::FO => v.extend(b"FO"),
            Country::FR => v.extend(b"FR"),
            Country::GA => v.extend(b"GA"),
            Country::GB => v.extend(b"GB"),
            Country::GD => v.extend(b"GD"),
            Country::GE => v.extend(b"GE"),
            Country::GF => v.extend(b"GF"),
            Country::GG => v.extend(b"GG"),
            Country::GH => v.extend(b"GH"),
            Country::GI => v.extend(b"GI"),
            Country::GL => v.extend(b"GL"),
            Country::GM => v.extend(b"GM"),
            Country::GN => v.extend(b"GN"),
            Country::GP => v.extend(b"GP"),
            Country::GQ => v.extend(b"GQ"),
            Country::GR => v.extend(b"GR"),
            Country::GS => v.extend(b"GS"),
            Country::GT => v.extend(b"GT"),
            Country::GU => v.extend(b"GU"),
            Country::GW => v.extend(b"GW"),
            Country::GY => v.extend(b"GY"),
            Country::HK => v.extend(b"HK"),
            Country::HM => v.extend(b"HM"),
            Country::HN => v.extend(b"HN"),
            Country::HR => v.extend(b"HR"),
            Country::HT => v.extend(b"HT"),
            Country::HU => v.extend(b"HU"),
            Country::ID => v.extend(b"ID"),
            Country::IE => v.extend(b"IE"),
            Country::IL => v.extend(b"IL"),
            Country::IM => v.extend(b"IM"),
            Country::IN => v.extend(b"IN"),
            Country::IO => v.extend(b"IO"),
            Country::IQ => v.extend(b"IQ"),
            Country::IR => v.extend(b"IR"),
            Country::IS => v.extend(b"IS"),
            Country::IT => v.extend(b"IT"),
            Country::JE => v.extend(b"JE"),
            Country::JM => v.extend(b"JM"),
            Country::JO => v.extend(b"JO"),
            Country::JP => v.extend(b"JP"),
            Country::KE => v.extend(b"KE"),
            Country::KG => v.extend(b"KG"),
            Country::KH => v.extend(b"KH"),
            Country::KI => v.extend(b"KI"),
            Country::KM => v.extend(b"KM"),
            Country::KN => v.extend(b"KN"),
            Country::KP => v.extend(b"KP"),
            Country::KR => v.extend(b"KR"),
            Country::KW => v.extend(b"KW"),
            Country::KY => v.extend(b"KY"),
            Country::KZ => v.extend(b"KZ"),
            Country::LA => v.extend(b"LA"),
            Country::LB => v.extend(b"LB"),
            Country::LC => v.extend(b"LC"),
            Country::LI => v.extend(b"LI"),
            Country::LK => v.extend(b"LK"),
            Country::LR => v.extend(b"LR"),
            Country::LS => v.extend(b"LS"),
            Country::LT => v.extend(b"LT"),
            Country::LU => v.extend(b"LU"),
            Country::LV => v.extend(b"LV"),
            Country::LY => v.extend(b"LY"),
            Country::MA => v.extend(b"MA"),
            Country::MC => v.extend(b"MC"),
            Country::MD => v.extend(b"MD"),
            Country::ME => v.extend(b"ME"),
            Country::MF => v.extend(b"MF"),
            Country::MG => v.extend(b"MG"),
            Country::MH => v.extend(b"MH"),
            Country::MK => v.extend(b"MK"),
            Country::ML => v.extend(b"ML"),
            Country::MM => v.extend(b"MM"),
            Country::MN => v.extend(b"MN"),
            Country::MO => v.extend(b"MO"),
            Country::MP => v.extend(b"MP"),
            Country::MQ => v.extend(b"MQ"),
            Country::MR => v.extend(b"MR"),
            Country::MS => v.extend(b"MS"),
            Country::MT => v.extend(b"MT"),
            Country::MU => v.extend(b"MU"),
            Country::MV => v.extend(b"MV"),
            Country::MW => v.extend(b"MW"),
            Country::MX => v.extend(b"MX"),
            Country::MY => v.extend(b"MY"),
            Country::MZ => v.extend(b"MZ"),
            Country::NA => v.extend(b"NA"),
            Country::NC => v.extend(b"NC"),
            Country::NE => v.extend(b"NE"),
            Country::NF => v.extend(b"NF"),
            Country::NG => v.extend(b"NG"),
            Country::NI => v.extend(b"NI"),
            Country::NL => v.extend(b"NL"),
            Country::NO => v.extend(b"NO"),
            Country::NP => v.extend(b"NP"),
            Country::NR => v.extend(b"NR"),
            Country::NU => v.extend(b"NU"),
            Country::NZ => v.extend(b"NZ"),
            Country::OM => v.extend(b"OM"),
            Country::PA => v.extend(b"PA"),
            Country::PE => v.extend(b"PE"),
            Country::PF => v.extend(b"PF"),
            Country::PG => v.extend(b"PG"),
            Country::PH => v.extend(b"PH"),
            Country::PK => v.extend(b"PK"),
            Country::PL => v.extend(b"PL"),
            Country::PM => v.extend(b"PM"),
            Country::PN => v.extend(b"PN"),
            Country::PR => v.extend(b"PR"),
            Country::PS => v.extend(b"PS"),
            Country::PT => v.extend(b"PT"),
            Country::PW => v.extend(b"PW"),
            Country::PY => v.extend(b"PY"),
            Country::QA => v.extend(b"QA"),
            Country::RE => v.extend(b"RE"),
            Country::RO => v.extend(b"RO"),
            Country::RS => v.extend(b"RS"),
            Country::RU => v.extend(b"RU"),
            Country::RW => v.extend(b"RW"),
            Country::SA => v.extend(b"SA"),
            Country::SB => v.extend(b"SB"),
            Country::SC => v.extend(b"SC"),
            Country::SD => v.extend(b"SD"),
            Country::SE => v.extend(b"SE"),
            Country::SG => v.extend(b"SG"),
            Country::SH => v.extend(b"SH"),
            Country::SI => v.extend(b"SI"),
            Country::SJ => v.extend(b"SJ"),
            Country::SK => v.extend(b"SK"),
            Country::SL => v.extend(b"SL"),
            Country::SM => v.extend(b"SM"),
            Country::SN => v.extend(b"SN"),
            Country::SO => v.extend(b"SO"),
            Country::SR => v.extend(b"SR"),
            Country::SS => v.extend(b"SS"),
            Country::ST => v.extend(b"ST"),
            Country::SV => v.extend(b"SV"),
            Country::SX => v.extend(b"SX"),
            Country::SY => v.extend(b"SY"),
            Country::SZ => v.extend(b"SZ"),
            Country::TC => v.extend(b"TC"),
            Country::TD => v.extend(b"TD"),
            Country::TF => v.extend(b"TF"),
            Country::TG => v.extend(b"TG"),
            Country::TH => v.extend(b"TH"),
            Country::TJ => v.extend(b"TJ"),
            Country::TK => v.extend(b"TK"),
            Country::TL => v.extend(b"TL"),
            Country::TM => v.extend(b"TM"),
            Country::TN => v.extend(b"TN"),
            Country::TO => v.extend(b"TO"),
            Country::TR => v.extend(b"TR"),
            Country::TT => v.extend(b"TT"),
            Country::TV => v.extend(b"TV"),
            Country::TW => v.extend(b"TW"),
            Country::TZ => v.extend(b"TZ"),
            Country::UA => v.extend(b"UA"),
            Country::UG => v.extend(b"UG"),
            Country::UM => v.extend(b"UM"),
            Country::US => v.extend(b"US"),
            Country::UY => v.extend(b"UY"),
            Country::UZ => v.extend(b"UZ"),
            Country::VA => v.extend(b"VA"),
            Country::VC => v.extend(b"VC"),
            Country::VE => v.extend(b"VE"),
            Country::VG => v.extend(b"VG"),
            Country::VI => v.extend(b"VI"),
            Country::VN => v.extend(b"VN"),
            Country::VU => v.extend(b"VU"),
            Country::WF => v.extend(b"WF"),
            Country::WS => v.extend(b"WS"),
            Country::YE => v.extend(b"YE"),
            Country::YT => v.extend(b"YT"),
            Country::ZA => v.extend(b"ZA"),
            Country::ZM => v.extend(b"ZM"),
            Country::ZW => v.extend(b"ZW"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::testutils::{check_decode_encode, check_no_decode};

    #[test]
    fn test_country() {
        check_decode_encode(b"US", Country::US);
        check_decode_encode(b"ZW", Country::ZW);
        check_no_decode::<Country>(b"UX");
    }
}
