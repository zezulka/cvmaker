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

// Coming up with a address scheme is a pain itself. Let's at least
// define some format
// https://en.wikipedia.org/wiki/Address_(geography)
struct Address {
    street : String,
    street_subunit : u32, // this is usually number of the building the address refers to
    postal_code : u32,
    country : Country
}

struct BasicInfo {
    name : String,
    surname : String,
    // maybe incorporate given name
    dob : NaiveDate,
    email : Email,
    phone : PhoneNumber,
    website : Url,
    address : Address
}

struct Education {
    // TODO
}

struct Experience {
    // TODO
}

pub struct Cv {
    basic : BasicInfo,
    education : Education,
    experience : Experience,
}

impl Cv {
    // we should define a way of creating a new Cv struct, maybe implement a builder pattern
}