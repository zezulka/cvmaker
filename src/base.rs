use chrono::NaiveDate;
use fast_chemail::is_valid_email;
use isocountry::CountryCode;
use phonenumber::PhoneNumber;
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Display, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::slice::Iter;
use url::Url;
use url_serde;
use vfs::VPath;

// Coming up with an address scheme is a pain in itself. Let's at least
// define some format
// https://en.wikipedia.org/wiki/Address_(geography)
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub street_subunit: u32, // this is usually number of the building the address refers to
    pub postal_code: u32,
    pub country: CountryCode,
}

impl Hash for Address {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.street.hash(state);
        self.street_subunit.hash(state);
        self.postal_code.hash(state);
        format!("{}", self.country).hash(state);
    }
}

fn are_hash_equal<T>(first: &T, second: &T) -> bool
where
    T: Hash,
{
    let mut first_hasher = DefaultHasher::new();
    let mut second_hasher = DefaultHasher::new();
    first.hash(&mut first_hasher);
    second.hash(&mut second_hasher);
    first_hasher.finish() == second_hasher.finish()
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        are_hash_equal(self, other)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmailAddress {
    address: String,
}

// TODO: Note: It would be much more idiomatic to implement the TryFrom/Into trait instead but
// both traits only belong to nighly-only experimental API. Leave it this way for the time being.
impl EmailAddress {
    pub fn from(address: &str) -> Result<EmailAddress, String> {
        if !is_valid_email(address) {
            return Err("Not a valid address.".to_string());
        }
        Ok(EmailAddress {
            address: address.to_string(),
        })
    }
}

#[derive(Clone, Eq, Serialize, Deserialize)]
pub enum Contact {
    Email(EmailAddress),
    #[serde(with = "url_serde")]
    Website(Url),
    Address(Address),
    Phone(PhoneNumber),
}

//TODO
impl Debug for Contact {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use self::Contact::*;
        match self {
            Email(ref addr) => write!(f, "{}", addr.address),
            Website(ref url) => write!(f, "{}", url),
            Address(ref addr) => write!(f, "{:?}", addr),
            Phone(ref num) => write!(f, "{}", num),
        }
    }
}

impl Display for Contact {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use self::Contact::*;
        match self {
            Email(ref addr) => write!(f, "E-mail : {}", addr.address),
            Website(ref url) => write!(f, "Website : {}", url),
            Address(ref addr) => write!(f, "Address : {:?}", addr),
            Phone(ref num) => write!(f, "Phone : {}", num),
        }
    }
}

// Return all the possible types available for contacts as tuples (enum, str).
// TODO : this feels dirty and should be done better!
pub fn contact_types<'a>() -> Vec<&'a str> {
    vec!["email", "website", "address", "phone"]
}

impl Hash for Contact {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use self::Contact::*;
        match self {
            Email(ref addr) => addr.hash(state),
            Website(ref url) => url.hash(state),
            Address(ref addr) => addr.hash(state),
            Phone(ref num) => format!("{}", num).hash(state),
        }
    }
}

impl PartialEq for Contact {
    fn eq(&self, other: &Self) -> bool {
        are_hash_equal(self, other)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicInfo {
    pub name: String,
    pub surname: String,
    // In order to generate the builder for BasicInfo (and, transitively, for CV), we cannot
    // have the "raw" NaiveDate, because Default trait implementation is required (and, obviously,
    // there is no such date which could be considered as the default one)
    pub dob: Option<NaiveDate>,
    // One caveat : we want at least one contact present in the contacts. Tests should catch this.
    pub contacts: Vec<Contact>,
}

impl<'a> BasicInfo {
    pub fn new(
        name: &'a str,
        surname: &'a str,
        dob: NaiveDate,
        contacts: Vec<Contact>,
    ) -> BasicInfo {
        if contacts.is_empty() {
            panic!("Contacts cannot be empty. Please provide at least one contact.");
        }
        BasicInfo {
            name: name.to_string(),
            surname: surname.to_string(),
            dob: Some(dob),
            contacts,
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct TimeSpan {
    from: NaiveDate,
    to: NaiveDate,
}

impl TimeSpan {
    pub fn new(from: NaiveDate, to: NaiveDate) -> TimeSpan {
        if from > to {
            panic!("The lefthand boundary must be less or equal to the righthand one.");
        }
        TimeSpan { from, to }
    }
}

impl ToString for TimeSpan {
    fn to_string(&self) -> String {
        self.from.to_string() + "   -   " + &self.to.to_string()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Education {
    pub span: TimeSpan,
    pub uni_name: String,
    pub degree: String,
    pub field_of_study: String,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Experience {
    pub span: TimeSpan,
    pub employer: String,
    pub job_name: String,
    pub description: String,
}

// Based on the CEFR model.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum LanguageProficiency {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
}

impl Display for LanguageProficiency {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.description())
    }
}

impl LanguageProficiency {
    pub fn iterator() -> Iter<'static, Self> {
        use self::LanguageProficiency::*;
        static PROFS: [LanguageProficiency; 6] = [A1, A2, B1, B2, C1, C2];
        PROFS.into_iter()
    }

    fn description(&self) -> &'static str {
        use self::LanguageProficiency::*;
        match self {
            A1 => "Beginner",
            A2 => "Elementary",
            B1 => "Intermediate",
            B2 => "Upper Intermediate",
            C1 => "Advanced",
            C2 => "Proficiency",
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Czech,
    Slovak,
    English,
    Russian,
    German,
    Spanish,
    Chinese,
    Dutch,
    French,
    Polish,
    Italian,
    Arabic,
    Portugese,
    Korean,
    Other,
}

impl Language {
    pub fn iterator() -> Iter<'static, Self> {
        use self::Language::*;
        static LANGS: [Language; 15] = [
            Czech, Slovak, English, Russian, German, Spanish, Chinese, Dutch, French, Polish,
            Italian, Arabic, Portugese, Korean, Other,
        ];
        LANGS.into_iter()
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

// Language would be ambiguous
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Lang {
    pub language: Language,
    pub proficiency: LanguageProficiency,
    pub notes: String,
}

#[derive(Default, Builder, Debug, Serialize, Deserialize, PartialEq)]
pub struct CV {
    #[builder(default = "None")]
    pub path: Option<String>,
    pub basic: BasicInfo,
    #[builder(default = "vec![]")]
    pub education: Vec<Education>,
    #[builder(default = "vec![]")]
    pub experience: Vec<Experience>,
    #[builder(default = "vec![]")]
    pub languages: Vec<Lang>,
}

impl CV {
    pub fn set_path(&mut self, path: Box<'static + VPath>) {
        self.path = Some(path.to_string().to_string());
    }
}

impl CVBuilder {
    pub fn default(basic: BasicInfo) -> CVBuilder {
        CVBuilder {
            basic: Some(basic),
            ..Default::default()
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use chrono::Datelike;
    use serde_test::{assert_ser_tokens, assert_tokens, Token};

    pub fn basic_cv_factory() -> CV {
        CVBuilder::default(basic_info_factory()).build().unwrap()
    }

    fn basic_info_factory() -> BasicInfo {
        let email = Contact::Email(EmailAddress::from("peter@raskolnikov.ru").unwrap());
        BasicInfo::new(
            "Peter",
            "Raskolnikov",
            NaiveDate::from_ymd(2000, 1, 1),
            vec![email],
        )
    }

    #[test]
    #[should_panic]
    fn timespan_invalid() {
        TimeSpan::new(
            NaiveDate::from_ymd(2000, 1, 2),
            NaiveDate::from_ymd(2000, 1, 1),
        );
    }

    #[test]
    fn timespan_ok() {
        let ts = TimeSpan::new(
            NaiveDate::from_ymd(2000, 1, 1),
            NaiveDate::from_ymd(2000, 5, 1),
        );
        assert_eq!(ts.from.year(), ts.to.year());
        assert_eq!(ts.to.month() - ts.from.month(), 4);
    }

    // Builders are automatically generated using the derive_builder crate.
    // We should only test that the builder does not build structs which have
    // uninitialized mandatory attributes.
    #[test]
    #[should_panic]
    fn builder_basic_info_empty_contacts() {
        BasicInfo::new("Whata", "Pity", NaiveDate::from_ymd(2000, 1, 1), vec![]);
    }

    #[test]
    fn builder_cv_basic_ok() {
        #[allow(unused_variables)]
        let cv = basic_cv_factory();
    }

    #[test]
    fn ser_basic_info() {
        let basic_info = basic_info_factory();
        assert_ser_tokens(&basic_info, &basic_info_vec());
    }

    #[test]
    fn serde_basic_cv() {
        let cv = basic_cv_factory();
        let mut expected_toks = vec![
            Token::Struct { name: "CV", len: 5 },
            Token::Str("path"),
            Token::None,
            Token::Str("basic"),
        ];
        expected_toks.append(&mut basic_info_vec());
        for field in &["education", "experience", "languages"] {
            expected_toks.push(Token::String(field));
            expected_toks.push(Token::Seq { len: Some(0) });
            expected_toks.push(Token::SeqEnd);
        }
        expected_toks.push(Token::StructEnd);
        assert_tokens(&cv, &expected_toks);
    }

    fn complex_cv_education_toks() -> Vec<Token> {
        let mut res = vec![
            Token::Str("education"),
            Token::Seq { len: Some(1) },
            Token::Struct {
                name: "Education",
                len: 4,
            },
        ];
        res.append(&mut timespan_factory_toks());
        res.append(&mut vec![
            Token::Str("uni_name"),
            Token::Str("Oxford"),
            Token::Str("degree"),
            Token::Str("MA"),
            Token::Str("field_of_study"),
            Token::Str("Speech recognition"),
        ]);
        res.push(Token::StructEnd);
        res.push(Token::SeqEnd);
        res
    }

    fn complex_cv_experience_toks() -> Vec<Token> {
        let mut res = vec![
            Token::Str("experience"),
            Token::Seq { len: Some(1) },
            Token::Struct {
                name: "Experience",
                len: 4,
            },
        ];
        res.append(&mut timespan_factory_toks());
        res.append(&mut vec![
            Token::Str("employer"),
            Token::Str("ABC, corp."),
            Token::Str("job_name"),
            Token::Str("accountant"),
            Token::Str("description"),
            Token::Str("I loved it!"),
        ]);
        res.push(Token::StructEnd);
        res.push(Token::SeqEnd);
        res
    }

    fn complex_cv_languages_toks() -> Vec<Token> {
        vec![
            Token::Str("languages"),
            Token::Seq { len: Some(1) },
            Token::Struct {
                name: "Lang",
                len: 3,
            },
            Token::Str("language"),
            Token::UnitVariant {
                name: "Language",
                variant: "English",
            },
            Token::Str("proficiency"),
            Token::UnitVariant {
                name: "LanguageProficiency",
                variant: "A1",
            },
            Token::Str("notes"),
            Token::Str(""),
            Token::StructEnd,
            Token::SeqEnd,
        ]
    }

    //TODO
    #[test]
    fn serde_complex_cv() {
        let mut expected_toks = vec![
            Token::Struct { name: "CV", len: 5 },
            Token::Str("path"),
            Token::None,
            Token::Str("basic"),
        ];
        expected_toks.append(&mut basic_info_vec());
        expected_toks.append(&mut complex_cv_education_toks());
        expected_toks.append(&mut complex_cv_experience_toks());
        expected_toks.append(&mut complex_cv_languages_toks());
        expected_toks.push(Token::StructEnd);
        assert_tokens(
            &CVBuilder::default(basic_info_factory())
                .education(vec![Education {
                    span: timespan_factory(),
                    uni_name: "Oxford".to_string(),
                    degree: "MA".to_string(),
                    field_of_study: "Speech recognition".to_string(),
                }]).experience(vec![Experience {
                    span: timespan_factory(),
                    employer: "ABC, corp.".to_string(),
                    job_name: "accountant".to_string(),
                    description: "I loved it!".to_string(),
                }]).languages(vec![Lang {
                    language: Language::English,
                    proficiency: LanguageProficiency::A1,
                    notes: "".to_string(),
                }]).build()
                .unwrap(),
            &expected_toks,
        );
    }

    #[test]
    fn serde_timespan() {
        assert_tokens(
            &timespan_factory(),
            &[
                Token::Struct {
                    name: "TimeSpan",
                    len: 2,
                },
                Token::String("from"),
                Token::String("2000-01-01"),
                Token::String("to"),
                Token::String("2001-01-01"),
                Token::StructEnd,
            ],
        );
    }

    fn timespan_factory() -> TimeSpan {
        TimeSpan::new(
            NaiveDate::from_ymd(2000, 1, 1),
            NaiveDate::from_ymd(2001, 1, 1),
        )
    }

    fn timespan_factory_toks() -> Vec<Token> {
        vec![
            Token::Str("span"),
            Token::Struct {
                name: "TimeSpan",
                len: 2,
            },
            Token::Str("from"),
            Token::Str("2000-01-01"),
            Token::Str("to"),
            Token::Str("2001-01-01"),
            Token::StructEnd,
        ]
    }

    fn basic_info_vec() -> Vec<Token> {
        vec![
            Token::Struct {
                name: "BasicInfo",
                len: 4,
            },
            Token::String("name"),
            Token::String("Peter"),
            Token::String("surname"),
            Token::String("Raskolnikov"),
            Token::String("dob"),
            Token::Some,
            Token::String("2000-01-01"),
            Token::String("contacts"),
            Token::Seq { len: Some(1) },
            Token::NewtypeVariant {
                name: "Contact",
                variant: "Email",
            },
            Token::Struct {
                name: "EmailAddress",
                len: 1,
            },
            Token::Str("address"),
            Token::Str("peter@raskolnikov.ru"),
            Token::StructEnd,
            Token::SeqEnd,
            Token::StructEnd,
        ]
    }
}
