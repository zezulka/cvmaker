use chrono::{DateTime, Datelike, Local, NaiveDate};
use cursive::traits::{Boxable, Finder, Identifiable};
use cursive::view::ViewWrapper;
use cursive::views::{IdView, LinearLayout, SelectView, TextContent, TextView};
use graphics::select_view_from_range;
use std::ops::Range;

fn date_picker(label_text: &str, show_days: bool) -> LinearLayout {
    let dt: DateTime<Local> = Local::now();
    let yr = dt.year() + 1;
    let mut res = LinearLayout::horizontal()
        .child(TextView::new_with_content(TextContent::new(label_text)).fixed_width(20))
        .child(select_view_from_range((1900..yr).rev()).with_id("yr"))
        .child(select_view_from_range::<u32, Range<u32>>(1..13).with_id("month"));
    if show_days {
        res.add_child(select_view_from_range::<u32, Range<u32>>(1..32).with_id("day")); // TODO: this is wrong, Feb 31...?
    }
    res
}

pub trait DatePicker {
    fn retrieve_date(&mut self) -> Option<NaiveDate>;
}

pub struct DateView {
    view: LinearLayout,
}

impl DateView {
    pub fn new_full(id: &str) -> IdView<DateView> {
        DateView {
            view: date_picker(id, true),
        }.with_id(id)
    }

    pub fn new_without_days(id: &str) -> IdView<DateView> {
        DateView {
            view: date_picker(id, false),
        }.with_id(id)
    }
}

impl ViewWrapper for DateView {
    wrap_impl!(self.view: LinearLayout);
}

impl DatePicker for DateView {
    fn retrieve_date(&mut self) -> Option<NaiveDate> {
        let year = self
            .view
            .find_id("yr", |s: &mut SelectView<i32>| s.selection());
        if let Some(Some(year)) = year {
            let month = self
                .view
                .find_id("month", |s: &mut SelectView<u32>| s.selection());
            if let Some(Some(month)) = month {
                let day = self
                    .view
                    .find_id("day", |s: &mut SelectView<u32>| s.selection());
                if let Some(Some(day)) = day {
                    return Some(NaiveDate::from_ymd(*year, *month, *day));
                }
                return Some(NaiveDate::from_ymd(*year, *month, 1));
            }
        }
        None
    }
}
