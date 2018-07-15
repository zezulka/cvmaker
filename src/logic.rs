use chrono::{NaiveDate, Datelike};
use email_format::Email;
use phonenumber::PhoneNumber;
use url::Url;
use iso_country::Country;
use isolang::Language;

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
}

// Coming up with an address scheme is a pain in itself. Let's at least
// define some format
// https://en.wikipedia.org/wiki/Address_(geography)
struct Address {
    street : String,
    street_subunit : u32, // this is usually number of the building the address refers to
    postal_code : u32,
    country : Country
}

enum Contact {
    Email(Email),
    Website(Url),
    Address(Address),
    Phone(PhoneNumber)
}

struct BasicInfo {
    name : String,
    surname : String,
    dob : NaiveDate,
    contact : Vec<Contact>,
}

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

struct Education {
    span : TimeSpan,
    uni_name : String,
    degree : String,
    field_of_study : String,
}

struct Experience {
    span : TimeSpan,
    employer : String,
    job_name : String,
    description : String,
}

// Based on the CEFR model.
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
struct Lang {
    language : Language,
    proficiency : LanguageProficiency,
    notes : String
}

pub struct Cv {
    basic : BasicInfo,
    education : Vec<Education>,
    experience : Vec<Experience>,
    languages : Vec<Lang>
}

impl Cv {
    // we should define a way of creating a new Cv struct, maybe implement a builder pattern
}