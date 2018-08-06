#[macro_use]
extern crate cursive;
extern crate chrono;
#[macro_use]
extern crate derive_builder;
extern crate fast_chemail;
extern crate phonenumber;
extern crate iso_country;
extern crate linked_hash_set;
extern crate url;
extern crate uuid;

mod base;
mod graphics;
mod dao;
use std::error::Error;
use graphics::Graphics;
use cursive::Cursive;

pub fn run() -> Result<(), Box<Error>> {
    Graphics::new(Cursive::default()).run();
    Ok(())
}