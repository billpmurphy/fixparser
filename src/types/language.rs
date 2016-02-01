use decoder::{Result, DecodeError};

/// ISO 639-1 code representing a language.
///
/// Used in FIX Protocol Version 5.0 SP2
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Language {
    ///Afar
    AA,
    ///Abkhaz
    AB,
    ///Avestan
    AE,
    ///Afrikaans
    AF,
    ///Akan
    AK,
    ///Amharic
    AM,
    ///Aragonese
    AN,
    ///Arabic
    AR,
    ///Assamese
    AS,
    ///Avaric
    AV,
    ///Aymara
    AY,
    ///Azerbaijani
    AZ,
    ///Bashkir
    BA,
    ///Belarusian
    BE,
    ///Bulgarian
    BG,
    ///Bihari
    BH,
    ///Bislama
    BI,
    ///Bambara
    BM,
    ///Bengali, Bangla
    BN,
    ///Tibetan Standard, Tibetan, Central
    BO,
    ///Breton
    BR,
    ///Bosnian
    BS,
    ///Catalan
    CA,
    ///Chechen
    CE,
    ///Chamorro
    CH,
    ///Corsican
    CO,
    ///Cree
    CR,
    ///Czech
    CS,
    ///Old Church Slavonic, Church Slavonic, Old Bulgarian
    CU,
    ///Chuvash
    CV,
    ///Welsh
    CY,
    ///Danish
    DA,
    ///German
    DE,
    ///Divehi, Dhivehi, Maldivian
    DV,
    ///Dzongkha
    DZ,
    ///Ewe
    EE,
    ///Greek (modern)
    EL,
    ///English
    EN,
    ///Esperanto
    EO,
    ///Spanish
    ES,
    ///Estonian
    ET,
    ///Basque
    EU,
    ///Persian (Farsi)
    FA,
    ///Fula, Fulah, Pulaar, Pular
    FF,
    ///Finnish
    FI,
    ///Fijian
    FJ,
    ///Faroese
    FO,
    ///French
    FR,
    ///Western Frisian
    FY,
    ///Irish
    GA,
    ///Scottish Gaelic, Gaelic
    GD,
    ///Galician
    GL,
    ///Guaraní
    GN,
    ///Gujarati
    GU,
    ///Manx
    GV,
    ///Hausa
    HA,
    ///Hebrew (modern)
    HE,
    ///Hindi
    HI,
    ///Hiri Motu
    HO,
    ///Croatian
    HR,
    ///Haitian, Haitian Creole
    HT,
    ///Hungarian
    HU,
    ///Armenian
    HY,
    ///Herero
    HZ,
    ///Interlingua
    IA,
    ///Indonesian
    ID,
    ///Interlingue
    IE,
    ///Igbo
    IG,
    ///Nuosu
    II,
    ///Inupiaq
    IK,
    ///Ido
    IO,
    ///Icelandic
    IS,
    ///Italian
    IT,
    ///Inuktitut
    IU,
    ///Japanese
    JA,
    ///Javanese
    JV,
    ///Georgian
    KA,
    ///Kongo
    KG,
    ///Kikuyu, Gikuyu
    KI,
    ///Kwanyama, Kuanyama
    KJ,
    ///Kazakh
    KK,
    ///Kalaallisut, Greenlandic
    KL,
    ///Khmer
    KM,
    ///Kannada
    KN,
    ///Korean
    KO,
    ///Kanuri
    KR,
    ///Kashmiri
    KS,
    ///Kurdish
    KU,
    ///Komi
    KV,
    ///Cornish
    KW,
    ///Kyrgyz
    KY,
    ///Latin
    LA,
    ///Luxembourgish, Letzeburgesch
    LB,
    ///Ganda
    LG,
    ///Limburgish, Limburgan, Limburger
    LI,
    ///Lingala
    LN,
    ///Lao
    LO,
    ///Lithuanian
    LT,
    ///Luba-Katanga
    LU,
    ///Latvian
    LV,
    ///Malagasy
    MG,
    ///Marshallese
    MH,
    ///Māori
    MI,
    ///Macedonian
    MK,
    ///Malayalam
    ML,
    ///Mongolian
    MN,
    ///Marathi (Marāṭhī)
    MR,
    ///Malay
    MS,
    ///Maltese
    MT,
    ///Burmese
    MY,
    ///Nauruan
    NA,
    ///Norwegian Bokmål
    NB,
    ///Northern Ndebele
    ND,
    ///Nepali
    NE,
    ///Ndonga
    NG,
    ///Dutch
    NL,
    ///Norwegian Nynorsk
    NN,
    ///Norwegian
    NO,
    ///Southern Ndebele
    NR,
    ///Navajo, Navaho
    NV,
    ///Chichewa, Chewa, Nyanja
    NY,
    ///Occitan
    OC,
    ///Ojibwe, Ojibwa
    OJ,
    ///Oromo
    OM,
    ///Oriya
    OR,
    ///Ossetian, Ossetic
    OS,
    ///Panjabi, Punjabi
    PA,
    ///Pāli
    PI,
    ///Polish
    PL,
    ///Pashto, Pushto
    PS,
    ///Portuguese
    PT,
    ///Quechua
    QU,
    ///Romansh
    RM,
    ///Kirundi
    RN,
    ///Romanian
    RO,
    ///Russian
    RU,
    ///Kinyarwanda
    RW,
    ///Sanskrit (Saṁskṛta)
    SA,
    ///Sardinian
    SC,
    ///Sindhi
    SD,
    ///Northern Sami
    SE,
    ///Sango
    SG,
    ///Sinhala, Sinhalese
    SI,
    ///Slovak
    SK,
    ///Slovene
    SL,
    ///Samoan
    SM,
    ///Shona
    SN,
    ///Somali
    SO,
    ///Albanian
    SQ,
    ///Serbian
    SR,
    ///Swati
    SS,
    ///Southern Sotho
    ST,
    ///Sundanese
    SU,
    ///Swedish
    SV,
    ///Swahili
    SW,
    ///Tamil
    TA,
    ///Telugu
    TE,
    ///Tajik
    TG,
    ///Thai
    TH,
    ///Tigrinya
    TI,
    ///Turkmen
    TK,
    ///Tagalog
    TL,
    ///Tswana
    TN,
    ///Tonga (Tonga Islands)
    TO,
    ///Turkish
    TR,
    ///Tsonga
    TS,
    ///Tatar
    TT,
    ///Twi
    TW,
    ///Tahitian
    TY,
    ///Uyghur
    UG,
    ///Ukrainian
    UK,
    ///Urdu
    UR,
    ///Uzbek
    UZ,
    ///Venda
    VE,
    ///Vietnamese
    VI,
    ///Volapük
    VO,
    ///Walloon
    WA,
    ///Wolof
    WO,
    ///Xhosa
    XH,
    ///Yiddish
    YI,
    ///Yoruba
    YO,
    ///Zhuang, Chuang
    ZA,
    ///Chinese
    ZH,
    ///Zulu
    ZU
}


pub fn decode_language(msg: &[u8; 2]) -> Result<Language> {
    match msg {
        b"AA" => Ok(Language::AA),
        b"AB" => Ok(Language::AB),
        b"AE" => Ok(Language::AE),
        b"AF" => Ok(Language::AF),
        b"AK" => Ok(Language::AK),
        b"AM" => Ok(Language::AM),
        b"AN" => Ok(Language::AN),
        b"AR" => Ok(Language::AR),
        b"AS" => Ok(Language::AS),
        b"AV" => Ok(Language::AV),
        b"AY" => Ok(Language::AY),
        b"AZ" => Ok(Language::AZ),
        b"BA" => Ok(Language::BA),
        b"BE" => Ok(Language::BE),
        b"BG" => Ok(Language::BG),
        b"BH" => Ok(Language::BH),
        b"BI" => Ok(Language::BI),
        b"BM" => Ok(Language::BM),
        b"BN" => Ok(Language::BN),
        b"BO" => Ok(Language::BO),
        b"BR" => Ok(Language::BR),
        b"BS" => Ok(Language::BS),
        b"CA" => Ok(Language::CA),
        b"CE" => Ok(Language::CE),
        b"CH" => Ok(Language::CH),
        b"CO" => Ok(Language::CO),
        b"CR" => Ok(Language::CR),
        b"CS" => Ok(Language::CS),
        b"CU" => Ok(Language::CU),
        b"CV" => Ok(Language::CV),
        b"CY" => Ok(Language::CY),
        b"DA" => Ok(Language::DA),
        b"DE" => Ok(Language::DE),
        b"DV" => Ok(Language::DV),
        b"DZ" => Ok(Language::DZ),
        b"EE" => Ok(Language::EE),
        b"EL" => Ok(Language::EL),
        b"EN" => Ok(Language::EN),
        b"EO" => Ok(Language::EO),
        b"ES" => Ok(Language::ES),
        b"ET" => Ok(Language::ET),
        b"EU" => Ok(Language::EU),
        b"FA" => Ok(Language::FA),
        b"FF" => Ok(Language::FF),
        b"FI" => Ok(Language::FI),
        b"FJ" => Ok(Language::FJ),
        b"FO" => Ok(Language::FO),
        b"FR" => Ok(Language::FR),
        b"FY" => Ok(Language::FY),
        b"GA" => Ok(Language::GA),
        b"GD" => Ok(Language::GD),
        b"GL" => Ok(Language::GL),
        b"GN" => Ok(Language::GN),
        b"GU" => Ok(Language::GU),
        b"GV" => Ok(Language::GV),
        b"HA" => Ok(Language::HA),
        b"HE" => Ok(Language::HE),
        b"HI" => Ok(Language::HI),
        b"HO" => Ok(Language::HO),
        b"HR" => Ok(Language::HR),
        b"HT" => Ok(Language::HT),
        b"HU" => Ok(Language::HU),
        b"HY" => Ok(Language::HY),
        b"HZ" => Ok(Language::HZ),
        b"IA" => Ok(Language::IA),
        b"ID" => Ok(Language::ID),
        b"IE" => Ok(Language::IE),
        b"IG" => Ok(Language::IG),
        b"II" => Ok(Language::II),
        b"IK" => Ok(Language::IK),
        b"IO" => Ok(Language::IO),
        b"IS" => Ok(Language::IS),
        b"IT" => Ok(Language::IT),
        b"IU" => Ok(Language::IU),
        b"JA" => Ok(Language::JA),
        b"JV" => Ok(Language::JV),
        b"KA" => Ok(Language::KA),
        b"KG" => Ok(Language::KG),
        b"KI" => Ok(Language::KI),
        b"KJ" => Ok(Language::KJ),
        b"KK" => Ok(Language::KK),
        b"KL" => Ok(Language::KL),
        b"KM" => Ok(Language::KM),
        b"KN" => Ok(Language::KN),
        b"KO" => Ok(Language::KO),
        b"KR" => Ok(Language::KR),
        b"KS" => Ok(Language::KS),
        b"KU" => Ok(Language::KU),
        b"KV" => Ok(Language::KV),
        b"KW" => Ok(Language::KW),
        b"KY" => Ok(Language::KY),
        b"LA" => Ok(Language::LA),
        b"LB" => Ok(Language::LB),
        b"LG" => Ok(Language::LG),
        b"LI" => Ok(Language::LI),
        b"LN" => Ok(Language::LN),
        b"LO" => Ok(Language::LO),
        b"LT" => Ok(Language::LT),
        b"LU" => Ok(Language::LU),
        b"LV" => Ok(Language::LV),
        b"MG" => Ok(Language::MG),
        b"MH" => Ok(Language::MH),
        b"MI" => Ok(Language::MI),
        b"MK" => Ok(Language::MK),
        b"ML" => Ok(Language::ML),
        b"MN" => Ok(Language::MN),
        b"MR" => Ok(Language::MR),
        b"MS" => Ok(Language::MS),
        b"MT" => Ok(Language::MT),
        b"MY" => Ok(Language::MY),
        b"NA" => Ok(Language::NA),
        b"NB" => Ok(Language::NB),
        b"ND" => Ok(Language::ND),
        b"NE" => Ok(Language::NE),
        b"NG" => Ok(Language::NG),
        b"NL" => Ok(Language::NL),
        b"NN" => Ok(Language::NN),
        b"NO" => Ok(Language::NO),
        b"NR" => Ok(Language::NR),
        b"NV" => Ok(Language::NV),
        b"NY" => Ok(Language::NY),
        b"OC" => Ok(Language::OC),
        b"OJ" => Ok(Language::OJ),
        b"OM" => Ok(Language::OM),
        b"OR" => Ok(Language::OR),
        b"OS" => Ok(Language::OS),
        b"PA" => Ok(Language::PA),
        b"PI" => Ok(Language::PI),
        b"PL" => Ok(Language::PL),
        b"PS" => Ok(Language::PS),
        b"PT" => Ok(Language::PT),
        b"QU" => Ok(Language::QU),
        b"RM" => Ok(Language::RM),
        b"RN" => Ok(Language::RN),
        b"RO" => Ok(Language::RO),
        b"RU" => Ok(Language::RU),
        b"RW" => Ok(Language::RW),
        b"SA" => Ok(Language::SA),
        b"SC" => Ok(Language::SC),
        b"SD" => Ok(Language::SD),
        b"SE" => Ok(Language::SE),
        b"SG" => Ok(Language::SG),
        b"SI" => Ok(Language::SI),
        b"SK" => Ok(Language::SK),
        b"SL" => Ok(Language::SL),
        b"SM" => Ok(Language::SM),
        b"SN" => Ok(Language::SN),
        b"SO" => Ok(Language::SO),
        b"SQ" => Ok(Language::SQ),
        b"SR" => Ok(Language::SR),
        b"SS" => Ok(Language::SS),
        b"ST" => Ok(Language::ST),
        b"SU" => Ok(Language::SU),
        b"SV" => Ok(Language::SV),
        b"SW" => Ok(Language::SW),
        b"TA" => Ok(Language::TA),
        b"TE" => Ok(Language::TE),
        b"TG" => Ok(Language::TG),
        b"TH" => Ok(Language::TH),
        b"TI" => Ok(Language::TI),
        b"TK" => Ok(Language::TK),
        b"TL" => Ok(Language::TL),
        b"TN" => Ok(Language::TN),
        b"TO" => Ok(Language::TO),
        b"TR" => Ok(Language::TR),
        b"TS" => Ok(Language::TS),
        b"TT" => Ok(Language::TT),
        b"TW" => Ok(Language::TW),
        b"TY" => Ok(Language::TY),
        b"UG" => Ok(Language::UG),
        b"UK" => Ok(Language::UK),
        b"UR" => Ok(Language::UR),
        b"UZ" => Ok(Language::UZ),
        b"VE" => Ok(Language::VE),
        b"VI" => Ok(Language::VI),
        b"VO" => Ok(Language::VO),
        b"WA" => Ok(Language::WA),
        b"WO" => Ok(Language::WO),
        b"XH" => Ok(Language::XH),
        b"YI" => Ok(Language::YI),
        b"YO" => Ok(Language::YO),
        b"ZA" => Ok(Language::ZA),
        b"ZH" => Ok(Language::ZH),
        b"ZU" => Ok(Language::ZU),
        _ => Err(DecodeError::Overflow)
    }
}
