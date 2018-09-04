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
    use base::*;
    use chrono::NaiveDate;
    use renderer::render_pdf;

    let email = Contact::Email(EmailAddress::from("peter@raskolnikov.ru").unwrap());
    let basic_info = BasicInfo::new(
        "Peter",
        "Raskolnikov",
        NaiveDate::from_ymd(2000, 1, 1),
        vec![email],
    );
    render_pdf(
        &CVBuilder::default(basic_info)
            .languages(vec![
                Lang {
                    language: Language::Russian,
                    proficiency: LanguageProficiency::C2,
                    notes: "native speaker".to_string(),
                },
                Lang {
                    language: Language::Arabic,
                    proficiency: LanguageProficiency::C1,
                    notes: String::new(),
                },
                Lang {
                    language: Language::English,
                    proficiency: LanguageProficiency::B1,
                    notes: "capable of basic communication".to_string(),
                }
            ])
            .experience(vec![Experience {
                span: TimeSpan::new(
                    NaiveDate::from_ymd(2015, 5, 1),
                    NaiveDate::from_ymd(2016, 12, 15),
                ),
                employer: "ABC, inc.".to_string(),
                job_name: "Translator".to_string(),
                description: String::new(),
            }])
            .education(vec![Education {
                span: TimeSpan::new(
                    NaiveDate::from_ymd(2011, 9, 1),
                    NaiveDate::from_ymd(2014, 6, 1),
                ),
                uni_name: "Cambridge".to_string(),
                degree: "Master of Arts".to_string(),
                field_of_study: "Applied linguistics".to_string(),
            }])
            .build()
            .unwrap(),
    );
    open_url("/tmp/test_cv.pdf");
}
