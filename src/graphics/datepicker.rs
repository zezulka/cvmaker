use chrono::{DateTime, Local, NaiveDate, Datelike};
use cursive::views::{LinearLayout, TextContent, TextView};
use cursive::view::ViewWrapper;
use cursive::traits::{View, Boxable};
use graphics::select_view_from_range;

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

pub struct DateView {
    view : LinearLayout,
}

impl DateView {
    pub fn new_full<'a>(id : &'a str) -> DateView {
        DateView { view : date_picker(id, true)}
    }

    pub fn new_without_days<'a>(id : &'a str) -> DateView {
        DateView { view : date_picker(id, false)}
    }
}

impl ViewWrapper for DateView {
    wrap_impl!(self.view : LinearLayout);
}

impl DatePicker for DateView {
    fn retrieve_date(&mut self) -> Option<NaiveDate> {
        unimplemented!()
    }
}