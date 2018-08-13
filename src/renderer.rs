// Responsibility: the program will create a PDF file based on the data given by the user.
use base::CV;
use pdf_canvas::{TextObject, BuiltinFont, Pdf};
use pdf_canvas::graphicsstate::CapStyle;
use std::io::Result as IoRes;

struct Point {
    x : f32,
    y : f32
}

struct Resolution {
    width : f32,
    height : f32
}


// The res parameter should be the maximum space the text will span across.
// In other words, linewrapper does not care about context! The position of origin must be already
// set in order to achieve the "right" result.
fn wrap_line(t : &mut TextObject, width : usize, text : &str) -> IoRes<()> {
    let mut curr = 0;
    let len = text.len();
    let mut cap = width;
    if(len < width) {
        t.show(text)?;
        return Ok(());
    }
    while(cap <= len) {
        match text[..cap].rfind(' ') {
            None => { t.show_line(&text[curr..]); break; },
            Some(nextcurr) => {
                t.show_line(&text[curr..nextcurr]);
                curr = nextcurr + 1;
                cap = curr + width;
            }
        }
    }
    t.show_line(&text[curr..]);
    Ok(())
}

//TODO : linewrapper, maybe add more structs which would improve the library composition a bit better.
pub fn render_pdf(cv : &CV) -> Result<(), String> {
    let Resolution {width, height} = Resolution { width : 800.0, height : 1200.0};
    let mut document = Pdf::create("/tmp/text.pdf").unwrap();
    let lorem_ipsum = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";
    document.set_title("Foo bar");
    document.render_page(width, height, |c| {
            let font = c.get_font(BuiltinFont::Times_Roman);
            c.text(|t| {
                t.set_font(&font, 14.0)?;
                t.set_leading(18.0)?;
                t.pos(10.0, 300.0)?;
                wrap_line(t, 50, lorem_ipsum)
            })
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