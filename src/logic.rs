extern crate chrono;
extern crate email_format;
extern crate phonenumber;
extern crate url;
extern crate iso_country;

use chrono::prelude::*;
use email_format::Email;
use phonenumber::PhoneNumber;
use url::Url;
use iso_country::Country;

#[cfg(test)]
mod tests {
    #[test]
    fn ok() {

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

struct Year {
    year : u32
}

impl Year {
    fn new(year : u32) -> Year {
        if year < 1900 {
            panic!("Please enter a sane year (1900-).");
        }
        Year { year }
    }
}

impl Ord for Year {
    fn cmp(&self, other : &Year) -> Ordering {
        self.year.cmp(&other.year)
    }
}

impl PartialOrd for Year {
    fn partial_cmp(&self, other: &Year) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct TimeSpan {
    from : Year,
    to : Year
}

impl TimeSpan {
    fn new(from : Year, to : Year) -> TimeSpan {
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
    field_of_study : String
}

struct Experience {
    employer : String,
    job_name : String,
    description : String,
    span : TimeSpan
}

struct Language {

}

pub struct Cv {
    basic : BasicInfo,
    education : Vec<Education>,
    experience : Vec<Experience>,
    languages : Vec<Language>
}

impl Cv {
    // we should define a way of creating a new Cv struct, maybe implement a builder pattern
}