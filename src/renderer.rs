// Responsibility: the program will create a PDF file based on the data given by the user.
use base::{Education, Experience, Lang, CV};
use printpdf::{
    types::pdf_layer::PdfLayerReference,
    types::plugins::graphics::two_dimensional::IndirectFontRef, Mm, PdfDocument,
    PdfDocumentReference,
};
use std::fs::File;
use std::io::BufWriter;

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

trait Renderable {
    fn render_object(&self, renderer: &mut Renderer);
}

impl Renderable for Experience {
    fn render_object(&self, renderer: &mut Renderer) {
        let Mm(width) = renderer.boundaries.width;
        renderer.render_text(
            &(self.employer.to_string() + "                             " + &self.span.to_string()),
            RenderParams::default()
                .with_font_type(FontType::Bold)
                .with_offset(width * 0.25),
        );
        renderer.render_text(
            &self.job_name,
            RenderParams::default()
                .with_font_type(FontType::Italic)
                .with_offset(width * 0.25),
        );
        renderer.render_text(
            &self.description,
            RenderParams::default().with_offset(width * 0.25),
        )
    }
}

impl Renderable for Education {
    fn render_object(&self, renderer: &mut Renderer) {
        let Mm(width) = renderer.boundaries.width;
        renderer.render_text(
            &(self.field_of_study.to_string()
                + "                             "
                + &self.span.to_string()),
            RenderParams::default()
                .with_font_type(FontType::Bold)
                .with_offset(width * 0.25),
        );
        renderer.render_text(
            &self.degree,
            RenderParams::default()
                .with_font_type(FontType::Italic)
                .with_offset(width * 0.25),
        );
        renderer.render_text(
            &self.uni_name,
            RenderParams::default().with_offset(width * 0.25),
        );
    }
}

impl Renderable for Lang {
    fn render_object(&self, renderer: &mut Renderer) {
        let Mm(width) = renderer.boundaries.width;
        renderer.render_text(
            &(self.language.to_string() + ": " + &self.proficiency.to_string()),
            RenderParams::default().with_offset(width * 0.25),
        );
        renderer.render_text(
            &self.notes,
            RenderParams::default().with_offset(width * 0.30),
        );
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
    bold_font: IndirectFontRef,
}

enum FontType {
    Normal,
    Italic,
    Bold,
}

struct RenderParams {
    offset: Option<f64>,
    f_type: FontType,
}

impl RenderParams {
    pub fn default() -> Self {
        RenderParams {
            offset: None,
            f_type: FontType::Normal,
        }
    }

    pub fn with_offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_font_type(mut self, f_type: FontType) -> Self {
        self.f_type = f_type;
        self
    }
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
                ).unwrap(),
            bold_font: doc
                .add_external_font(File::open("src/resources/fonts/OpenSans-Bold.ttf").unwrap())
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
    fn render_text_vector(
        &mut self,
        data: Vec<String>,
        RenderParams { offset, f_type }: RenderParams,
    ) {
        let font_size = 15;
        self.canvas.begin_text_section();
        let mut future_row_pos: f64 = 0.0;
        // Create an artifical scope because we borrow a font which would collide with the statement
        // right next after this scope.
        {
            use self::FontType::*;
            let font = match f_type {
                Normal => &self.font,
                Italic => &self.italic_font,
                Bold => &self.bold_font,
            };
            self.canvas.set_font(font, font_size);
            self.canvas.set_line_height(font_size);
            let mut y = self.current.col;

            if let Some(offset) = offset {
                y += Mm(offset);
            }
            self.canvas.set_text_cursor(y, self.current.row);
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

    fn render_text(&mut self, text: &str, render_params: RenderParams) {
        self.render_text_vector(vec![text.to_string()], render_params);
    }

    /// Implementation note: The y axis is inverted, therefore passing RendererCoordinates { col : Mm(0.0), row : Mm(5.5) }
    /// will have the intended effect of moving 5.5 mm BELOW, however, the rendering algorithm
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
        self.render_text_vector(basic_vec, RenderParams::default());
        Ok(())
    }

    fn render_data_vector<T>(&mut self, data: &Vec<T>, label: &str) -> RendererResult
    where
        T: Renderable,
    {
        if !data.is_empty() {
            self.render_text(
                label,
                RenderParams::default().with_font_type(FontType::Italic),
            );
            data.iter().for_each(|item| {
                item.render_object(self);
            });
        }
        Ok(())
    }

    fn render_experience(&mut self) -> RendererResult {
        self.render_data_vector(&self.cv.experience, "Experience")
    }

    fn render_education(&mut self) -> RendererResult {
        self.render_data_vector(&self.cv.education, "Education")
    }

    fn render_languages(&mut self) -> RendererResult {
        self.render_data_vector(&self.cv.languages, "Languages")
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
    #[test]
    fn test() {}
}
