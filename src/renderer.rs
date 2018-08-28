// Responsibility: the program will create a PDF file based on the data given by the user.
use base::CV;
use base::{BasicInfo, Education, Experience, Lang};
use std::fmt::Debug;
use std::io::Result as IoRes;

struct Point {
    x: f32,
    y: f32,
}

struct Resolution {
    width: f32,
    height: f32,
}

impl Resolution {
    // Respect the ratio specified by ISO 216
    // https://www.cl.cam.ac.uk/~mgk25/iso-paper.html
    fn new_a4(width: f32) -> Resolution {
        Resolution {
            width,
            height: width * 1.4142,
        }
    }
}

fn render_basic_info(
    BasicInfo {
        name,
        surname,
        dob,
        contacts,
    }: &BasicInfo,
) -> IoRes<()> {
    Ok(())
}

fn render_experience(data: &Vec<Experience>) -> IoRes<()> {
    Ok(())
}

fn render_education(ata: &Vec<Education>) -> IoRes<()> {
    Ok(())
}

fn render_languages(data: &Vec<Lang>) -> IoRes<()> {
    Ok(())
}

//TODO : maybe add more structs which would improve the library usage.
pub fn render_pdf(cv: &CV) -> Result<(), String> {
    // Leave the parameters hardwired for now.
    let res = Resolution::new_a4(1000.0);
    let Resolution { width, height } = res;
    let lorem_ipsum = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";
    //render_basic_info(&cv.basic)?;
    //render_experience(&cv.experience)?;
    //render_education(&cv.education)?;
    //render_languages(&cv.languages)?;
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {}
}
