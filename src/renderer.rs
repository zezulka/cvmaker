// Responsibility: the program will create a PDF file based on the data given by the user.
use base::CV;
use pdf_canvas::{Canvas, FontRef, TextObject, BuiltinFont, Pdf};
use pdf_canvas::graphicsstate::Color;
use std::io::Result as IoRes;
use base::{Education, Lang, Experience, BasicInfo};

struct Point {
    x : f32,
    y : f32
}

struct Resolution {
    width : f32,
    height : f32
}

impl Resolution {
    // Respect the ratio specified by ISO 216
    // https://www.cl.cam.ac.uk/~mgk25/iso-paper.html
    fn new_a4(width : f32) -> Resolution {
        Resolution { width, height : width * 1.4142 }
    }
}

// The res parameter should be the maximum space the text will span across.
// In other words, linewrapper does not care about context! The position of origin must be already
// set in order to achieve the "right" result.
fn wrap_line(t : &mut TextObject, width : usize, text : &str) -> IoRes<()> {
    let mut curr = 0;
    let len = text.len();
    let mut cap = width;
    if len < width {
        t.show(text)?;
        return Ok(());
    }
    while cap <= len {
        match text[..cap].rfind(' ') {
            None => { t.show_line(&text[curr..])?; break; },
            Some(next_curr) => {
                t.show_line(&text[curr..next_curr])?;
                curr = next_curr + 1;
                cap = curr + width;
            }
        }
    }
    t.show_line(&text[curr..])?;
    Ok(())
}

fn black_rectangle_white_text(c : &mut Canvas, text : &str, fref : &FontRef, fsize : f32, Point{ x, y } : Point) -> IoRes<()> {
    let w = fref.get_width(fsize, text);
    c.set_stroke_color(Color::rgb(0, 0, 0))?;
    c.set_fill_color(Color::rgb(0,0,0))?;
    c.rectangle(x, y, w, 26.0)?;
    c.fill()?;
    c.stroke()?;
    c.text(|t| {
        t.set_font(fref, fsize)?;
        t.set_fill_color(Color::rgb(0xFF, 0xFF, 0xFF))?;
        t.pos(x, y + 23.0)?;
        t.show_line(text)
    })
}

fn render_basic_info(c : &mut Canvas, Resolution {width, height} : &Resolution, fref : BuiltinFont, fref_bold : BuiltinFont, fsize : f32,
                     BasicInfo {name, surname, dob, contacts} : &BasicInfo) -> IoRes<()> {
    c.set_stroke_color(Color::rgb(0,0,0))?;
    let name_surname = name.to_string() + " " + surname;
    c.center_text(*width * 0.25, *height - 60.0, fref_bold, fsize * 2.0, &name_surname)?;
    c.line(*width * 0.25, *height - 90.0, *width, *height - 90.0);
    c.stroke()?;
    let mut offset = *height - 150.0;
    contacts.iter().for_each(|contact| {
        c.right_text(*width * 0.75, offset, fref_bold, fsize, &format!("{:#}", contact));
        offset -= 30.0;
    });
    Ok(())
}

fn render_experience(cv : &mut Canvas, data : &Vec<Experience>) -> IoRes<()> {
    Ok(())
}

fn render_education(cv : &mut Canvas, data : &Vec<Education>) -> IoRes<()> {
    Ok(())
}

fn render_languages(cv : &mut Canvas, data : &Vec<Lang>) -> IoRes<()> {
    Ok(())
}

//TODO : linewrapper, maybe add more structs which would improve the library composition a bit better.
pub fn render_pdf(cv : &CV) -> Result<(), String> {
    let res = Resolution::new_a4(1000.0);
    let Resolution {width, height} = res;

    let mut document = Pdf::create("/tmp/text.pdf").unwrap();
    let lorem_ipsum = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";
    document.set_title("Foo bar");
    document.render_page(width, height, |c| {
            let font = BuiltinFont::Times_Roman;
            let font_ref = c.get_font(font);
            render_basic_info(c, &res, font, BuiltinFont::Times_Bold, 14.0, &cv.basic)?;
            render_experience(c, &cv.experience)?;
            render_education(c, &cv.education)?;
            render_languages(c, &cv.languages)?;
            c.text(|t| {
                t.set_font(&font_ref, 14.0)?;
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