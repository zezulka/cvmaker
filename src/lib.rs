#[macro_use]
extern crate cursive;
extern crate chrono;
#[macro_use]
extern crate derive_builder;
extern crate fast_chemail;
extern crate isocountry;
extern crate open;
extern crate phonenumber;
extern crate printpdf;
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
use base::CV;
use cursive::Cursive;
use graphics::Graphics;
use open::that as open_url;
use std::error::Error;

pub fn run() -> Result<(), Box<Error>> {
    //Graphics::new(Cursive::default()).run();
    run_mocked_renderer();
    Ok(())
}

fn run_mocked_renderer() {
    use base::BasicInfo;
    use base::CVBuilder;
    use base::Contact;
    use base::EmailAddress;
    use chrono::NaiveDate;
    use renderer::render_pdf;

    let email = Contact::Email(EmailAddress::from("peter@raskolnikov.ru").unwrap());
    let basic_info = BasicInfo::new(
        "Peter",
        "Raskolnikov",
        NaiveDate::from_ymd(2000, 1, 1),
        vec![email],
    );
    render_pdf(&CVBuilder::default(basic_info).build().unwrap());
    open_url("/tmp/test_cv.pdf");
}
