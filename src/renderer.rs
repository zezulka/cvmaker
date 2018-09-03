// Responsibility: the program will create a PDF file based on the data given by the user.
use base::CV;
use base::{BasicInfo, Education, Experience, Lang};
use printpdf::{
    types::pdf_layer::PdfLayerReference,
    types::plugins::graphics::two_dimensional::IndirectFontRef, Mm, PdfDocument,
    PdfDocumentReference,
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
        RendererCoordinates {
            col: Mm(0.0),
            row: dim.height,
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
}

// To be used with Renderer only.
// Tells the renderer which way to go after a line of text has been prepared in the buffer.
enum CursorMovement {
    Nothing,
    Newline,
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
            doc,
        }
    }

    /// This method consumes the object itself.
    pub fn render(mut self) -> RendererResult {
        self.render_basic_info()?;
        self.render_experience()?;
        self.render_education()?;
        self.render_languages()?;
        match self.doc.save(&mut BufWriter::new(
            File::create("/tmp/test_cv.pdf").unwrap(),
        )) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn render_text(&mut self, text: &str, post_movement: CursorMovement) {
        self.canvas
            .use_text(text, 17, self.current.col, self.current.row, &self.font);
        use self::CursorMovement::*;
        match post_movement {
            Nothing => (),
            //TODO the newline offset might need to be tempered with.
            Newline => self.move_cursor(RendererCoordinates {
                col: Mm(0.0),
                row: Mm(20.0),
            }),
        }
    }

    fn move_cursor(&mut self, diff: RendererCoordinates) {
        self.current.col += diff.col;
        self.current.row += diff.row;
    }

    fn render_basic_info(&mut self) -> RendererResult {
        let basic = &self.cv.basic;
        self.render_text(&basic.name, CursorMovement::Nothing);
        self.render_text(&basic.surname, CursorMovement::Newline);
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

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {}
}
