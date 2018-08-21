#[macro_use]
extern crate cursive;
extern crate chrono;
#[macro_use]
extern crate derive_builder;
extern crate fast_chemail;
extern crate isocountry;
extern crate pdf_canvas;
extern crate phonenumber;
extern crate url;
extern crate url_serde;
extern crate uuid;
extern crate vfs;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_test;

mod base;
mod dao;
mod graphics;
mod renderer;
use cursive::Cursive;
use graphics::Graphics;
use std::error::Error;

pub fn run() -> Result<(), Box<Error>> {
    Graphics::new(Cursive::default()).run();
    Ok(())
}
