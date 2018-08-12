// Responsibility: the program will create a PDF file based on the data given by the user.
use base::CV;
use pdf_canvas::{BuiltinFont, Pdf};
use pdf_canvas::graphicsstate::CapStyle;

//TODO : linewrapper, maybe add more structs which would improve the library composition a bit better.
pub fn render_pdf(cv : &CV) -> Result<(), String> {
    let mut document = Pdf::create("/tmp/text.pdf").unwrap();
    document.set_title("Foo bar");
    document.render_page(800.0, 800.0, |c| {
            c.center_text(150.0, 330.0, BuiltinFont::Times_Bold, 18.0, &format!("{:?}", cv))?;
            Ok(())
        }).unwrap();
    document.finish().unwrap();
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {

    }
}