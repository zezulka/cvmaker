// Responsibility: the program will create a PDF file based on the data given by the user.
use base::CV;
use base::{BasicInfo, Education, Experience, Lang};
use printpdf::{
    types::pdf_layer::PdfLayerReference,
    types::plugins::graphics::two_dimensional::IndirectFontRef, Cmyk, Color, Line, Mm,
    PdfConformance, PdfDocument, PdfDocumentReference, Point, Rgb,
};
use std::fmt::Debug;
use std::fs::File;
use std::io::BufWriter;
use std::iter::FromIterator;

type RendererResult = Result<(), String>;
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

struct RendererCoordinates {
    col: Mm,
    row: Mm,
}

impl RendererCoordinates {
    pub fn start(dim: &SheetDim) -> RendererCoordinates {
        let start_height = dim.height - Mm(15.0);
        RendererCoordinates {
            col: Mm(0.0),
            row: start_height,
        }
    }
}

struct Renderer<'a> {
    cv: &'a CV,
    canvas: &'a PdfLayerReference,
    doc: PdfDocumentReference,
    current: RendererCoordinates,
    boundaries: &'a SheetDim,
    font: IndirectFontRef,
    italic_font: IndirectFontRef,
}

impl<'a> Renderer<'a> {
    pub fn new(
        cv: &'a CV,
        canvas: &'a PdfLayerReference,
        dim: &'a SheetDim,
        doc: PdfDocumentReference,
    ) -> Self {
        Renderer {
            cv,
            canvas,
            current: RendererCoordinates::start(dim),
            boundaries: dim,
            font: doc
                .add_external_font(File::open("src/resources/fonts/OpenSans-Regular.ttf").unwrap())
                .unwrap(),
            italic_font: doc
                .add_external_font(
                    File::open("src/resources/fonts/OpenSans-LightItalic.ttf").unwrap(),
                )
                .unwrap(),
            doc,
        }
    }

    /// This method consumes the object itself.
    pub fn render(mut self) -> RendererResult {
        self.render_italic_text("CV, bla bla");
        self.render_basic_info()?;
        self.render_experience()?;
        self.render_education()?;
        self.render_languages()?;
        // ISO standard optimized for print production.https://en.wikipedia.org/wiki/PDF/X
        //self.doc.repair_errors(PdfConformance::X5G_2010_PDF_1_6);
        match self.doc.save(&mut BufWriter::new(
            File::create("/tmp/test_cv.pdf").unwrap(),
        )) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    //TODO wrap lines if they are too long
    fn render_text_vector(&mut self, data: Vec<String>, italic_font : bool) {
        let font_size = 15;
        self.canvas.begin_text_section();
        let mut future_row_pos: f64 = 0.0;
        // Create an artifical scope because we borrow a font
        {
            let font = if italic_font { &self.italic_font } else { &self.font };
            self.canvas.set_font(font, font_size);
            self.canvas.set_text_cursor(Mm(10.0), Mm(10.0));
            self.canvas.set_line_height(font_size);
            self.canvas.set_word_spacing(3000);
            self.canvas
                .set_text_cursor(self.current.col, self.current.row);
            data.iter().for_each(|line| {
                self.canvas.write_text(line.as_str(), font);
                self.canvas.add_line_break();
                future_row_pos += font_size as f64;
            });
        }
        self.move_cursor_with_offset(RendererCoordinates {
            row: Mm(future_row_pos),
            col: Mm(0.0),
        });
        self.canvas.end_text_section();
    }

    fn render_text(&mut self, text: &str) {
        self.render_text_vector(vec![text.to_string()], false);
    }

    fn render_italic_text(&mut self, text: &str) {
        self.render_text_vector(vec![text.to_string()], true);
    }

    /// Note: The y axis is inverted, therefore passing RendererCoordinates { col : Mm(0.0), row : Mm(5.5) }
    /// will have the intended effect of moving 5.5 mm BELOW, the rendering algorithm
    /// condiders the origin of the document as the left bottom corner, so y the value passed must
    /// be subtracted from the current cursor.
    fn move_cursor_with_offset(&mut self, diff: RendererCoordinates) {
        self.current.col += diff.col;
        self.current.row -= diff.row;
    }

    fn render_basic_info(&mut self) -> RendererResult {
        let basic = &self.cv.basic;
        let mut basic_vec = vec![
            basic.name.to_string() + " " + &basic.surname,
            "Date of birth: ".to_string() + &basic.dob.unwrap().to_string(),
        ];
        basic
            .contacts
            .iter()
            .for_each(|contact| basic_vec.push(contact.to_string()));
        self.render_text_vector(basic_vec, false);
        Ok(())
    }

    fn render_experience(&mut self) -> RendererResult {
        self.cv
            .experience
            .iter()
            .for_each(|experience| self.render_text(&format!("{:?}", experience)));
        Ok(())
    }

    fn render_education(&mut self) -> RendererResult {
        self.cv
            .education
            .iter()
            .for_each(|edu| self.render_text(&format!("{:?}", edu)));
        Ok(())
    }

    fn render_languages(&mut self) -> RendererResult {
        self.cv
            .languages
            .iter()
            .for_each(|lang| self.render_text(&format!("{:?}", lang)));
        Ok(())
    }
}

pub fn render_pdf(cv: &CV) -> RendererResult {
    let dim = SheetDim::a4();
    let SheetDim { width, height } = dim;
    let (doc, page_idx, layer_idx) = PdfDocument::new(
        format!("CV - {} {}", cv.basic.name, cv.basic.surname),
        width,
        height,
        "main layer".to_string(),
    );
    Renderer::new(&cv, &doc.get_page(page_idx).get_layer(layer_idx), &dim, doc).render()
}

//TODO write a bit more tests.
#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {}
}
