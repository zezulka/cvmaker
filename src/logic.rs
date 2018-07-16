use chrono::{NaiveDate, Datelike};
use phonenumber::PhoneNumber;
use url::Url;
use iso_country::Country;
use isolang::Language;
use fast_chemail::is_valid_email;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn timespan_invalid() {
        TimeSpan::new(NaiveDate::from_ymd(2000, 1, 2), NaiveDate::from_ymd(2000, 1, 1));
    }

    #[test]
    fn timespan_ok() {
        let ts = TimeSpan::new(NaiveDate::from_ymd(2000, 1, 1), NaiveDate::from_ymd(2000, 5, 1));
        assert_eq!(ts.from.year(), ts.to.year());
        assert_eq!(ts.to.month() - ts.from.month(), 4);
    }

    // Builders are automatically generated using the derive_builder crate.
    // We should only test that the builder does not build structs which have
    // uninitialized mandatory attributes.
    #[test]
    #[should_panic]
    fn builder_cv_basic_empty_contacts() {
        let basic_info = BasicInfoBuilder::default()
            .name("Peter".to_string())
            .surname("Raskolnikov".to_string())
            .dob(Some(NaiveDate::from_ymd(1970, 1, 1)))
            .contacts(vec![])
            .build()
            .unwrap();
        let cv = CVBuilder::default().basic(basic_info).build();
    }

    #[should_panic]
    fn builder_cv_basic_empty_contacts_second_take() {
        let basic_info = BasicInfoBuilder::default()
            .name("Peter".to_string())
            .surname("Raskolnikov".to_string())
            .dob(Some(NaiveDate::from_ymd(1970, 1, 1)))
            .build()
            .unwrap();
        let cv = CVBuilder::default().basic(basic_info).build();
    }

    #[test]
    fn builder_cv_basic_ok() {
        let basic_info = BasicInfoBuilder::default()
            .name("Peter".to_string())
            .surname("Raskolnikov".to_string())
            .dob(Some(NaiveDate::from_ymd(1970, 1, 1)))
            .contacts(vec![Contact::Email(EmailAddress::from("peter@raskolnikov.ru").unwrap())])
            .build()
            .unwrap();
        let cv = CVBuilder::default().basic(basic_info).build();
    }
}

// Coming up with an address scheme is a pain in itself. Let's at least
// define some format
// https://en.wikipedia.org/wiki/Address_(geography)
#[derive(Clone, Debug)]
struct Address {
    street : String,
    street_subunit : u32, // this is usually number of the building the address refers to
    postal_code : u32,
    country : Country
}

#[derive(Clone, Debug)]
struct EmailAddress {
    address : String
}

impl EmailAddress {
    fn from(address : &str) -> Result<EmailAddress, String> {
        if !is_valid_email(address) {
            return Err("Not a valid address.".to_string());
        }
        Ok(EmailAddress { address : address.to_string() })
    }
}

#[derive(Clone, Debug)]
enum Contact {
    Email(EmailAddress),
    Website(Url),
    Address(Address),
    Phone(PhoneNumber)
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(build_fn(validate = "Self::validate"))]
struct BasicInfo {
    name : String,
    surname : String,
    // In order to generate the builder for BasicInfo (and, transitively, for CV), we cannot
    // have the "raw" NaiveDate, because Default trait implementation is required (and, obviously,
    // there is no such date which could be considered as the default one)
    dob : Option<NaiveDate>,
    // One caveat : we want at least one contact present in the contacts. Tests should catch this.
    contacts : Vec<Contact>,
}

impl BasicInfoBuilder {
    fn validate(&self) -> Result<(), String> {
        let err_str = "You must provide at least one contact in your CV.";
        match self.contacts {
            // This should not even happen because of the Default trait
            // but let's not be too clever here.
            None => Err(err_str.to_string()),
            Some(ref vec) => {
                if vec.is_empty() {
                    return Err(err_str.to_string());
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug)]
struct TimeSpan {
    from : NaiveDate,
    to : NaiveDate
}

impl TimeSpan {
    fn new(from : NaiveDate, to : NaiveDate) -> TimeSpan {
        if from > to {
            panic!("The lefthand boundary must be lesser or equal to the righthand one.");
        }
        TimeSpan { from, to }
    }
}

#[derive(Clone, Debug)]
struct Education {
    span : TimeSpan,
    uni_name : String,
    degree : String,
    field_of_study : String,
}

#[derive(Clone, Debug)]
struct Experience {
    span : TimeSpan,
    employer : String,
    job_name : String,
    description : String,
}

// Based on the CEFR model.
#[derive(Clone, Debug)]
enum LanguageProficiency {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2
}

impl LanguageProficiency {
    fn description(&self) -> &'static str {
        use self::LanguageProficiency::*;
        match self {
           A1 => "Beginner",
           A2 => "Elementary",
           B1 => "Intermediate",
           B2 => "Upper Intermediate",
           C1 => "Advanced",
           C2 => "Proficiency",
           _ => panic!("Unknown proficiency")
        }
    }
}

// Language would be ambiguous
#[derive(Clone, Debug)]
struct Lang {
    language : Language,
    proficiency : LanguageProficiency,
    notes : String
}

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct CV {
    basic : BasicInfo,
    education : Vec<Education>,
    experience : Vec<Experience>,
    languages : Vec<Lang>
}

impl CV {
}