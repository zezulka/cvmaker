// Responsibility: the program will create a PDF file based on the data given by the user.
use base::CV;
use base::{BasicInfo, Education, Experience, Lang};
use std::fmt::Debug;
use std::io::Result as IoRes;
use printpdf::{ Mm, PdfDocument, types::plugins::graphics::two_dimensional::IndirectFontRef };
use std::fs::File;
use std::io::BufWriter;
use std::iter::FromIterator;

struct Point {
    x: f32,
    y: f32,
}

// Simple wrapper to be used with the printpdf library.
struct SheetDim {
    width: Mm,
    height: Mm,
}

impl SheetDim {
    // Source : https://www.papersizes.org/a-paper-sizes.htm
    fn a4() -> SheetDim {
        SheetDim {
            width : Mm(210.0),
            height: Mm(297.0),
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
    let SheetDim { width, height } = SheetDim::a4();
    let (doc, page1, layer1) = PdfDocument::new(format!("CV - {} {}", cv.basic.name, cv.basic.surname),
                                                width, height, "Layer 1".to_string());
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(File::open("src/resources/fonts/OpenSans-Regular.ttf")
        .unwrap()).unwrap();
    current_layer.use_text("Why does it hurt when I pee?", 17, Mm(50.0), Mm(50.0), &font);
    doc.save(&mut BufWriter::new(File::create("/tmp/test_working.pdf").unwrap())).unwrap();
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
