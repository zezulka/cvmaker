use chrono::{NaiveDate};
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
    fn year_invalid() {

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

enum MaritalStatus {
    Married,
    Single,
    Other(String)
}

struct BasicInfo {
    name : String,
    surname : String,
    dob : NaiveDate,
    email : Email,
    phone : PhoneNumber,
    website : Url,
    address : Address,
    marital_status : MaritalStatus,
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
           _ => panic!("Unknown level")
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