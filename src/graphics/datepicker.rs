use chrono::{DateTime, Local, NaiveDate, Datelike};
use cursive::views::{IdView, LinearLayout, TextContent, TextView};
use cursive::traits::{View, Boxable};
use graphics::select_view_from_range;
use cursive::Printer;

fn date_picker<'a>(label_text : &'a str, show_days : bool) -> LinearLayout {
    let dt : DateTime<Local> = Local::now();
    let yr = dt.year()+1;
    let mut res = LinearLayout::horizontal()
        .child(TextView::new_with_content(TextContent::new(label_text)).fixed_width(20))
        .child(select_view_from_range((1900..yr).rev()))
        .child(select_view_from_range(1..13));
    if show_days {
        res.add_child(select_view_from_range(1..32)); // TODO: this is wrong, Feb 31...?
    }
    res
}

trait DatePicker {
    fn retrieve_date(&mut self) -> Option<NaiveDate>;

}

struct DaylessDateView {
    contents : LinearLayout,
}

impl DaylessDateView {
    fn new<'a>(id : &'a str) -> DaylessDateView {
        DaylessDateView { contents : date_picker(id, false) }
    }
}

impl View for DaylessDateView {
    fn draw(&self, printer: &Printer) {
        self.contents.draw(printer);
    }
}

impl DatePicker for DaylessDateView {
    fn retrieve_date(&mut self) -> Option<NaiveDate> {
        unimplemented!()
    }
}

struct FullDateView {
    contents : LinearLayout,
}

impl FullDateView {
    fn new<'a>(id : &'a str) -> FullDateView {
        FullDateView { contents : date_picker(id, true)}
    }
}

impl View for FullDateView {
    fn draw(&self, printer: &Printer) {
        self.contents.draw(printer);
    }
}

impl DatePicker for FullDateView {
    fn retrieve_date(&mut self) -> Option<NaiveDate> {
        unimplemented!()
    }
}