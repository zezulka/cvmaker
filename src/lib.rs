#[macro_use]
extern crate cursive;
extern crate chrono;
extern crate fast_chemail;
extern crate phonenumber;
extern crate url;
extern crate iso_country;
extern crate linked_hash_set;
#[macro_use]
extern crate derive_builder;

mod base;
mod graphics;
use std::error::Error;
use graphics::Graphics;
use cursive::Cursive;

pub fn run() -> Result<(), Box<Error>> {
    Graphics::new(Cursive::default()).run();
    Ok(())
}