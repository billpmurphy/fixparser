use protocol::FIXValue;


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

impl FIXValue for Language {
    fn from_bytes(bytes: &[u8]) -> Option<Language> {
        match bytes {
            b"AA" => Some(Language::AA),
            b"AB" => Some(Language::AB),
            b"AE" => Some(Language::AE),
            b"AF" => Some(Language::AF),
            b"AK" => Some(Language::AK),
            b"AM" => Some(Language::AM),
            b"AN" => Some(Language::AN),
            b"AR" => Some(Language::AR),
            b"AS" => Some(Language::AS),
            b"AV" => Some(Language::AV),
            b"AY" => Some(Language::AY),
            b"AZ" => Some(Language::AZ),
            b"BA" => Some(Language::BA),
            b"BE" => Some(Language::BE),
            b"BG" => Some(Language::BG),
            b"BH" => Some(Language::BH),
            b"BI" => Some(Language::BI),
            b"BM" => Some(Language::BM),
            b"BN" => Some(Language::BN),
            b"BO" => Some(Language::BO),
            b"BR" => Some(Language::BR),
            b"BS" => Some(Language::BS),
            b"CA" => Some(Language::CA),
            b"CE" => Some(Language::CE),
            b"CH" => Some(Language::CH),
            b"CO" => Some(Language::CO),
            b"CR" => Some(Language::CR),
            b"CS" => Some(Language::CS),
            b"CU" => Some(Language::CU),
            b"CV" => Some(Language::CV),
            b"CY" => Some(Language::CY),
            b"DA" => Some(Language::DA),
            b"DE" => Some(Language::DE),
            b"DV" => Some(Language::DV),
            b"DZ" => Some(Language::DZ),
            b"EE" => Some(Language::EE),
            b"EL" => Some(Language::EL),
            b"EN" => Some(Language::EN),
            b"EO" => Some(Language::EO),
            b"ES" => Some(Language::ES),
            b"ET" => Some(Language::ET),
            b"EU" => Some(Language::EU),
            b"FA" => Some(Language::FA),
            b"FF" => Some(Language::FF),
            b"FI" => Some(Language::FI),
            b"FJ" => Some(Language::FJ),
            b"FO" => Some(Language::FO),
            b"FR" => Some(Language::FR),
            b"FY" => Some(Language::FY),
            b"GA" => Some(Language::GA),
            b"GD" => Some(Language::GD),
            b"GL" => Some(Language::GL),
            b"GN" => Some(Language::GN),
            b"GU" => Some(Language::GU),
            b"GV" => Some(Language::GV),
            b"HA" => Some(Language::HA),
            b"HE" => Some(Language::HE),
            b"HI" => Some(Language::HI),
            b"HO" => Some(Language::HO),
            b"HR" => Some(Language::HR),
            b"HT" => Some(Language::HT),
            b"HU" => Some(Language::HU),
            b"HY" => Some(Language::HY),
            b"HZ" => Some(Language::HZ),
            b"IA" => Some(Language::IA),
            b"ID" => Some(Language::ID),
            b"IE" => Some(Language::IE),
            b"IG" => Some(Language::IG),
            b"II" => Some(Language::II),
            b"IK" => Some(Language::IK),
            b"IO" => Some(Language::IO),
            b"IS" => Some(Language::IS),
            b"IT" => Some(Language::IT),
            b"IU" => Some(Language::IU),
            b"JA" => Some(Language::JA),
            b"JV" => Some(Language::JV),
            b"KA" => Some(Language::KA),
            b"KG" => Some(Language::KG),
            b"KI" => Some(Language::KI),
            b"KJ" => Some(Language::KJ),
            b"KK" => Some(Language::KK),
            b"KL" => Some(Language::KL),
            b"KM" => Some(Language::KM),
            b"KN" => Some(Language::KN),
            b"KO" => Some(Language::KO),
            b"KR" => Some(Language::KR),
            b"KS" => Some(Language::KS),
            b"KU" => Some(Language::KU),
            b"KV" => Some(Language::KV),
            b"KW" => Some(Language::KW),
            b"KY" => Some(Language::KY),
            b"LA" => Some(Language::LA),
            b"LB" => Some(Language::LB),
            b"LG" => Some(Language::LG),
            b"LI" => Some(Language::LI),
            b"LN" => Some(Language::LN),
            b"LO" => Some(Language::LO),
            b"LT" => Some(Language::LT),
            b"LU" => Some(Language::LU),
            b"LV" => Some(Language::LV),
            b"MG" => Some(Language::MG),
            b"MH" => Some(Language::MH),
            b"MI" => Some(Language::MI),
            b"MK" => Some(Language::MK),
            b"ML" => Some(Language::ML),
            b"MN" => Some(Language::MN),
            b"MR" => Some(Language::MR),
            b"MS" => Some(Language::MS),
            b"MT" => Some(Language::MT),
            b"MY" => Some(Language::MY),
            b"NA" => Some(Language::NA),
            b"NB" => Some(Language::NB),
            b"ND" => Some(Language::ND),
            b"NE" => Some(Language::NE),
            b"NG" => Some(Language::NG),
            b"NL" => Some(Language::NL),
            b"NN" => Some(Language::NN),
            b"NO" => Some(Language::NO),
            b"NR" => Some(Language::NR),
            b"NV" => Some(Language::NV),
            b"NY" => Some(Language::NY),
            b"OC" => Some(Language::OC),
            b"OJ" => Some(Language::OJ),
            b"OM" => Some(Language::OM),
            b"OR" => Some(Language::OR),
            b"OS" => Some(Language::OS),
            b"PA" => Some(Language::PA),
            b"PI" => Some(Language::PI),
            b"PL" => Some(Language::PL),
            b"PS" => Some(Language::PS),
            b"PT" => Some(Language::PT),
            b"QU" => Some(Language::QU),
            b"RM" => Some(Language::RM),
            b"RN" => Some(Language::RN),
            b"RO" => Some(Language::RO),
            b"RU" => Some(Language::RU),
            b"RW" => Some(Language::RW),
            b"SA" => Some(Language::SA),
            b"SC" => Some(Language::SC),
            b"SD" => Some(Language::SD),
            b"SE" => Some(Language::SE),
            b"SG" => Some(Language::SG),
            b"SI" => Some(Language::SI),
            b"SK" => Some(Language::SK),
            b"SL" => Some(Language::SL),
            b"SM" => Some(Language::SM),
            b"SN" => Some(Language::SN),
            b"SO" => Some(Language::SO),
            b"SQ" => Some(Language::SQ),
            b"SR" => Some(Language::SR),
            b"SS" => Some(Language::SS),
            b"ST" => Some(Language::ST),
            b"SU" => Some(Language::SU),
            b"SV" => Some(Language::SV),
            b"SW" => Some(Language::SW),
            b"TA" => Some(Language::TA),
            b"TE" => Some(Language::TE),
            b"TG" => Some(Language::TG),
            b"TH" => Some(Language::TH),
            b"TI" => Some(Language::TI),
            b"TK" => Some(Language::TK),
            b"TL" => Some(Language::TL),
            b"TN" => Some(Language::TN),
            b"TO" => Some(Language::TO),
            b"TR" => Some(Language::TR),
            b"TS" => Some(Language::TS),
            b"TT" => Some(Language::TT),
            b"TW" => Some(Language::TW),
            b"TY" => Some(Language::TY),
            b"UG" => Some(Language::UG),
            b"UK" => Some(Language::UK),
            b"UR" => Some(Language::UR),
            b"UZ" => Some(Language::UZ),
            b"VE" => Some(Language::VE),
            b"VI" => Some(Language::VI),
            b"VO" => Some(Language::VO),
            b"WA" => Some(Language::WA),
            b"WO" => Some(Language::WO),
            b"XH" => Some(Language::XH),
            b"YI" => Some(Language::YI),
            b"YO" => Some(Language::YO),
            b"ZA" => Some(Language::ZA),
            b"ZH" => Some(Language::ZH),
            b"ZU" => Some(Language::ZU),
            _ => None
        }
    }

    fn to_bytes(&self, v: &mut Vec<u8>) {
        match *self {
            Language::AA => v.extend(b"AA"),
            Language::AB => v.extend(b"AB"),
            Language::AE => v.extend(b"AE"),
            Language::AF => v.extend(b"AF"),
            Language::AK => v.extend(b"AK"),
            Language::AM => v.extend(b"AM"),
            Language::AN => v.extend(b"AN"),
            Language::AR => v.extend(b"AR"),
            Language::AS => v.extend(b"AS"),
            Language::AV => v.extend(b"AV"),
            Language::AY => v.extend(b"AY"),
            Language::AZ => v.extend(b"AZ"),
            Language::BA => v.extend(b"BA"),
            Language::BE => v.extend(b"BE"),
            Language::BG => v.extend(b"BG"),
            Language::BH => v.extend(b"BH"),
            Language::BI => v.extend(b"BI"),
            Language::BM => v.extend(b"BM"),
            Language::BN => v.extend(b"BN"),
            Language::BO => v.extend(b"BO"),
            Language::BR => v.extend(b"BR"),
            Language::BS => v.extend(b"BS"),
            Language::CA => v.extend(b"CA"),
            Language::CE => v.extend(b"CE"),
            Language::CH => v.extend(b"CH"),
            Language::CO => v.extend(b"CO"),
            Language::CR => v.extend(b"CR"),
            Language::CS => v.extend(b"CS"),
            Language::CU => v.extend(b"CU"),
            Language::CV => v.extend(b"CV"),
            Language::CY => v.extend(b"CY"),
            Language::DA => v.extend(b"DA"),
            Language::DE => v.extend(b"DE"),
            Language::DV => v.extend(b"DV"),
            Language::DZ => v.extend(b"DZ"),
            Language::EE => v.extend(b"EE"),
            Language::EL => v.extend(b"EL"),
            Language::EN => v.extend(b"EN"),
            Language::EO => v.extend(b"EO"),
            Language::ES => v.extend(b"ES"),
            Language::ET => v.extend(b"ET"),
            Language::EU => v.extend(b"EU"),
            Language::FA => v.extend(b"FA"),
            Language::FF => v.extend(b"FF"),
            Language::FI => v.extend(b"FI"),
            Language::FJ => v.extend(b"FJ"),
            Language::FO => v.extend(b"FO"),
            Language::FR => v.extend(b"FR"),
            Language::FY => v.extend(b"FY"),
            Language::GA => v.extend(b"GA"),
            Language::GD => v.extend(b"GD"),
            Language::GL => v.extend(b"GL"),
            Language::GN => v.extend(b"GN"),
            Language::GU => v.extend(b"GU"),
            Language::GV => v.extend(b"GV"),
            Language::HA => v.extend(b"HA"),
            Language::HE => v.extend(b"HE"),
            Language::HI => v.extend(b"HI"),
            Language::HO => v.extend(b"HO"),
            Language::HR => v.extend(b"HR"),
            Language::HT => v.extend(b"HT"),
            Language::HU => v.extend(b"HU"),
            Language::HY => v.extend(b"HY"),
            Language::HZ => v.extend(b"HZ"),
            Language::IA => v.extend(b"IA"),
            Language::ID => v.extend(b"ID"),
            Language::IE => v.extend(b"IE"),
            Language::IG => v.extend(b"IG"),
            Language::II => v.extend(b"II"),
            Language::IK => v.extend(b"IK"),
            Language::IO => v.extend(b"IO"),
            Language::IS => v.extend(b"IS"),
            Language::IT => v.extend(b"IT"),
            Language::IU => v.extend(b"IU"),
            Language::JA => v.extend(b"JA"),
            Language::JV => v.extend(b"JV"),
            Language::KA => v.extend(b"KA"),
            Language::KG => v.extend(b"KG"),
            Language::KI => v.extend(b"KI"),
            Language::KJ => v.extend(b"KJ"),
            Language::KK => v.extend(b"KK"),
            Language::KL => v.extend(b"KL"),
            Language::KM => v.extend(b"KM"),
            Language::KN => v.extend(b"KN"),
            Language::KO => v.extend(b"KO"),
            Language::KR => v.extend(b"KR"),
            Language::KS => v.extend(b"KS"),
            Language::KU => v.extend(b"KU"),
            Language::KV => v.extend(b"KV"),
            Language::KW => v.extend(b"KW"),
            Language::KY => v.extend(b"KY"),
            Language::LA => v.extend(b"LA"),
            Language::LB => v.extend(b"LB"),
            Language::LG => v.extend(b"LG"),
            Language::LI => v.extend(b"LI"),
            Language::LN => v.extend(b"LN"),
            Language::LO => v.extend(b"LO"),
            Language::LT => v.extend(b"LT"),
            Language::LU => v.extend(b"LU"),
            Language::LV => v.extend(b"LV"),
            Language::MG => v.extend(b"MG"),
            Language::MH => v.extend(b"MH"),
            Language::MI => v.extend(b"MI"),
            Language::MK => v.extend(b"MK"),
            Language::ML => v.extend(b"ML"),
            Language::MN => v.extend(b"MN"),
            Language::MR => v.extend(b"MR"),
            Language::MS => v.extend(b"MS"),
            Language::MT => v.extend(b"MT"),
            Language::MY => v.extend(b"MY"),
            Language::NA => v.extend(b"NA"),
            Language::NB => v.extend(b"NB"),
            Language::ND => v.extend(b"ND"),
            Language::NE => v.extend(b"NE"),
            Language::NG => v.extend(b"NG"),
            Language::NL => v.extend(b"NL"),
            Language::NN => v.extend(b"NN"),
            Language::NO => v.extend(b"NO"),
            Language::NR => v.extend(b"NR"),
            Language::NV => v.extend(b"NV"),
            Language::NY => v.extend(b"NY"),
            Language::OC => v.extend(b"OC"),
            Language::OJ => v.extend(b"OJ"),
            Language::OM => v.extend(b"OM"),
            Language::OR => v.extend(b"OR"),
            Language::OS => v.extend(b"OS"),
            Language::PA => v.extend(b"PA"),
            Language::PI => v.extend(b"PI"),
            Language::PL => v.extend(b"PL"),
            Language::PS => v.extend(b"PS"),
            Language::PT => v.extend(b"PT"),
            Language::QU => v.extend(b"QU"),
            Language::RM => v.extend(b"RM"),
            Language::RN => v.extend(b"RN"),
            Language::RO => v.extend(b"RO"),
            Language::RU => v.extend(b"RU"),
            Language::RW => v.extend(b"RW"),
            Language::SA => v.extend(b"SA"),
            Language::SC => v.extend(b"SC"),
            Language::SD => v.extend(b"SD"),
            Language::SE => v.extend(b"SE"),
            Language::SG => v.extend(b"SG"),
            Language::SI => v.extend(b"SI"),
            Language::SK => v.extend(b"SK"),
            Language::SL => v.extend(b"SL"),
            Language::SM => v.extend(b"SM"),
            Language::SN => v.extend(b"SN"),
            Language::SO => v.extend(b"SO"),
            Language::SQ => v.extend(b"SQ"),
            Language::SR => v.extend(b"SR"),
            Language::SS => v.extend(b"SS"),
            Language::ST => v.extend(b"ST"),
            Language::SU => v.extend(b"SU"),
            Language::SV => v.extend(b"SV"),
            Language::SW => v.extend(b"SW"),
            Language::TA => v.extend(b"TA"),
            Language::TE => v.extend(b"TE"),
            Language::TG => v.extend(b"TG"),
            Language::TH => v.extend(b"TH"),
            Language::TI => v.extend(b"TI"),
            Language::TK => v.extend(b"TK"),
            Language::TL => v.extend(b"TL"),
            Language::TN => v.extend(b"TN"),
            Language::TO => v.extend(b"TO"),
            Language::TR => v.extend(b"TR"),
            Language::TS => v.extend(b"TS"),
            Language::TT => v.extend(b"TT"),
            Language::TW => v.extend(b"TW"),
            Language::TY => v.extend(b"TY"),
            Language::UG => v.extend(b"UG"),
            Language::UK => v.extend(b"UK"),
            Language::UR => v.extend(b"UR"),
            Language::UZ => v.extend(b"UZ"),
            Language::VE => v.extend(b"VE"),
            Language::VI => v.extend(b"VI"),
            Language::VO => v.extend(b"VO"),
            Language::WA => v.extend(b"WA"),
            Language::WO => v.extend(b"WO"),
            Language::XH => v.extend(b"XH"),
            Language::YI => v.extend(b"YI"),
            Language::YO => v.extend(b"YO"),
            Language::ZA => v.extend(b"ZA"),
            Language::ZH => v.extend(b"ZH"),
            Language::ZU => v.extend(b"ZU"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::testutils::{check_decode_encode, check_no_decode};

    #[test]
    fn test_language() {
        check_decode_encode(b"EN", Language::EN);
        check_decode_encode(b"UK", Language::UK);
        check_no_decode::<Language>(b"UX");
    }
}
