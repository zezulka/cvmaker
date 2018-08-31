// Responsibility: the program will create a PDF file based on the data given by the user.
use base::CV;
use base::{BasicInfo, Education, Experience, Lang};
use printpdf::{
    types::pdf_layer::PdfLayerReference,
    types::plugins::graphics::two_dimensional::IndirectFontRef, Mm, PdfDocument,
};
use std::fmt::Debug;
use std::fs::File;
use std::io::BufWriter;
use std::iter::FromIterator;

type RendererResult = Result<(), String>;

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
            width: Mm(210.0),
            height: Mm(297.0),
        }
    }
}

struct Renderer<'a> {
    cv: &'a CV,
    canvas: &'a PdfLayerReference,
}

impl<'a> Renderer<'a> {
    pub fn new(cv: &'a CV, canvas: &'a PdfLayerReference) -> Self {
        Renderer { cv, canvas }
    }

    pub fn render(&self) -> RendererResult {
        self.render_basic_info()?;
        self.render_experience()?;
        self.render_education()?;
        self.render_languages()
    }

    fn render_basic_info(&self) -> RendererResult {
        Ok(())
    }

    fn render_experience(&self) -> RendererResult {
        Ok(())
    }

    fn render_education(&self) -> RendererResult {
        Ok(())
    }

    fn render_languages(&self) -> RendererResult {
        Ok(())
    }
}

//TODO : maybe add more structs which would improve the library usage.
pub fn render_pdf(cv: &CV) -> RendererResult {
    let SheetDim { width, height } = SheetDim::a4();
    let (doc, page1, layer1) = PdfDocument::new(
        format!("CV - {} {}", cv.basic.name, cv.basic.surname),
        width,
        height,
        "Layer 1".to_string(),
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc
        .add_external_font(File::open("src/resources/fonts/OpenSans-Regular.ttf").unwrap())
        .unwrap();
    current_layer.use_text(
        "Why does it hurt when I pee?",
        17,
        Mm(50.0),
        Mm(50.0),
        &font,
    );

    doc.save(&mut BufWriter::new(
        File::create("/tmp/test_working.pdf").unwrap(),
    )).unwrap();
    Renderer::new(&cv, &current_layer).render()
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {}
}
