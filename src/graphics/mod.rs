use cursive::align::HAlign;
use cursive::event::Key;
use cursive::menu::MenuTree;
use cursive::traits::*;
use cursive::views::{Button, BoxView, Dialog, Canvas, EditView, SelectView,
                     LinearLayout, TextView, TextContent};
use cursive::Cursive;
use std::sync::atomic::{AtomicUsize, Ordering};
use base::contact_types;
use base::LanguageProficiency;
use std::fmt::Display;
use base::{Contact, Language, CV, CVBuilder, BasicInfo};
use url::Url;
use std::str::FromStr;
use self::datepicker::{DateView, DatePicker};
use dao::{CVManager, CVDao};

mod datepicker;

fn select_view_from_range<S : Display + 'static, T : Iterator<Item = S>>(rng : T) -> SelectView<S> {
    let mut sel_view : SelectView<S> = SelectView::new().h_align(HAlign::Center);
    rng.for_each(|item| { sel_view.add_item(format!("{}", item), item)} );
    sel_view.popup()
}

pub struct Graphics {
    engine : Cursive,
}

impl Graphics {
    pub fn new(engine : Cursive) -> Graphics {
        Graphics { engine }
    }

    fn setup_looks(&mut self) {
        if let Err(msg) = self.engine.load_theme_file("src/resources/theme.toml") {
            // We could also panic but receiving the error only means
            // we fall back to the default theme which is ok for the time being.
            eprintln!("Could not load the themes file: {:?}", msg);
        }
    }

    pub fn run(&mut self) {
        self.init();
        self.engine.run();
    }

    fn init(&mut self) {
        self.setup_looks();
        self.add_menu();
        self.engine.add_layer(Canvas::new(()));
        self.add_form();
        self.engine.add_global_callback(Key::Esc, |s| s.select_menubar());
    }

    // Creates a form row containing description on the left and an editable field on the right.
    // The label must be nonempty.
    fn form_row<'a>(label_text : &'a str, col_size : usize) -> LinearLayout {
        if label_text.is_empty() {
            panic!("Got empty label text, expected nonempty.");
        }
        LinearLayout::horizontal()
            .child(TextView::new_with_content(TextContent::new(label_text))
                .fixed_width(col_size))
            .child(EditView::new().fixed_width(col_size).with_id(label_text))
    }

    fn form_row_default_col_size<'a>(label_text : &'a str) -> LinearLayout {
        Self::form_row(label_text, 20)
    }

    fn contact_select_view() -> SelectView<String> {
        let mut sel_view : SelectView<String> = SelectView::new().h_align(HAlign::Center);
        contact_types().iter().for_each(|&item| { sel_view.add_item(item, item.to_string()) });
        sel_view.popup()
    }

    fn contact_row(s: &mut Cursive) {
        s.call_on_id("Contacts", |view: &mut LinearLayout| {
            view.add_child(LinearLayout::horizontal()
                .child(Self::contact_select_view())
                .child(EditView::new().fixed_width(20))
            )
        });
    }

    fn experience_row(s: &mut Cursive) {
        s.call_on_id("Experience", |view: &mut LinearLayout| {
            view.add_child(LinearLayout::vertical()
                .child(DateView::new_without_days("From"))
                .child(DateView::new_without_days("To"))
                .child(Self::form_row_default_col_size("Employer"))
                .child(Self::form_row_default_col_size("Job name"))
                .child(Self::form_row_default_col_size("Description"))
            )
        });
    }

    //TODO : if user enters the "Other" option, let him fill in the "other" language
    fn language_row(s: &mut Cursive) {
        s.call_on_id("Languages", |view: &mut LinearLayout| {
            view.add_child(LinearLayout::vertical()
                .child(LinearLayout::horizontal()
                    .child(TextView::new_with_content(TextContent::new("Language name")).fixed_width(20))
                    .child(select_view_from_range(Language::iterator())))
                .child(LinearLayout::horizontal()
                    .child(TextView::new_with_content(TextContent::new("Proficiency")).fixed_width(20))
                    .child(select_view_from_range(LanguageProficiency::iterator())))
                .child(Self::form_row_default_col_size("Additional notes"))
            )
        });
    }

    fn education_row(s : &mut Cursive) {
        s.call_on_id("Education", |view: &mut LinearLayout| {
            view.add_child(LinearLayout::vertical()
                .child(DateView::new_without_days("From"))
                .child(DateView::new_without_days("To"))
                .child(Self::form_row_default_col_size("University"))
                .child(Self::form_row_default_col_size("Degree"))
                .child(Self::form_row_default_col_size("Field of study"))
            )
        });
    }

    fn expandable_linear_layout_contacts(event_fun : &'static Fn(&mut Cursive))
                                         -> LinearLayout {
        LinearLayout::vertical()
            .child(LinearLayout::horizontal()
                .child(TextView::new_with_content(TextContent::new("Contacts")).fixed_width(20)))
            .child(LinearLayout::vertical()
                .child(LinearLayout::horizontal().child(Self::contact_select_view())
                    .child(EditView::new().fixed_width(20)))
                .with_id("Contacts"))
            .child(LinearLayout::horizontal()
                .child(Button::new("Add another", event_fun)))
    }

    fn expandable_linear_layout<'a>(label : &'a str, event_fun : &'static Fn(&mut Cursive))
                                    -> LinearLayout {
        LinearLayout::vertical()
            .child(LinearLayout::horizontal()
                .child(TextView::new_with_content(TextContent::new(label)).fixed_width(20)))
            .child(LinearLayout::vertical().with_id(label))
            .child(LinearLayout::horizontal()
                .child(Button::new("Add another", event_fun)))
    }

    fn add_form(&mut self) {
        let form = LinearLayout::vertical()
            .child(Self::form_row_default_col_size("Name"))
            .child(Self::form_row_default_col_size("Surname"))
            .child(DateView::new_full("Date of birth"))
            .child(Self::expandable_linear_layout_contacts(&Self::contact_row))
            .child(Self::expandable_linear_layout("Languages", &Self::language_row))
            .child(Self::expandable_linear_layout("Education", &Self::education_row))
            .child(Self::expandable_linear_layout("Experience", &Self::experience_row))
            .scrollable();
        self.engine.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(form))
            .title("New CV")
            .button("Create new CV", |s| {
                //TODO this is ugly as hell.
                let mut cv = Self::collect_form_data(s).unwrap();
                let manager = CVDao::new();
                match manager.add_cv(&mut cv) {
                    Ok(()) => println!("CV with id {} added successfully.", cv.path.unwrap()),
                    Err(e) => println!("{:?}", e)
                }
            })

        );
    }

    fn collect_contacts() -> Vec<Contact> {
        let lhs = vec![Contact::Website(Url::from_str("http://www.foo.bar").unwrap())];
        lhs
    }

    fn collect_basic_info(c : &mut Cursive) -> Option<BasicInfo> {
        let name = c.call_on_id("Name", |s : &mut BoxView<EditView>| s.get_inner().get_content())
            .unwrap();
        let surname = c.call_on_id("Surname", |s : &mut BoxView<EditView>| s.get_inner().get_content())
            .unwrap();
        let dob = c.call_on_id("Date of birth", |s : &mut DateView| s.retrieve_date()).unwrap().unwrap();
        Some(BasicInfo::new(&name, &surname, dob, Self::collect_contacts()))
    }

    // This handler is responsible for collecting the data from the "New CV" form.
    pub fn collect_form_data(c : &mut Cursive) -> Option<CV> {
        if let Some(basic) = Self::collect_basic_info(c) {
            return match CVBuilder::default(basic).build() {
                Ok(cv) => Some(cv),
                Err(err) => { eprintln!("{}", err); None },
            }
        }
        None
    }

    // Fearlessly stolen from the cursive example.
    fn add_menu(&mut self) {
        // We'll use a counter to name new files.
        let counter = AtomicUsize::new(1);

        // The menubar is a list of (label, menu tree) pairs.
        self.engine.menubar()
            // We add a new "File" tree
            .add_subtree("File",
                         MenuTree::new()
                             // Trees are made of leaves, with are directly actionable...
                             .leaf("New", move |s| {
                                 // Here we use the counter to add an entry
                                 // in the list of "Recent" items.
                                 let i = counter.fetch_add(1, Ordering::Relaxed);
                                 let filename = format!("New {}", i);
                                 s.menubar().find_subtree("File").unwrap()
                                     .find_subtree("Recent").unwrap()
                                     .insert_leaf(0, filename, |_| ());

                                 s.add_layer(Dialog::info("New file!"));
                             })
                             // ... and of sub-trees, which open up when selected.
                             .subtree("Recent",
                                      // The `.with()` method can help when running loops
                                      // within builder patterns.
                                      MenuTree::new().with(|tree| {
                                          for i in 1..100 {
                                              // We don't actually do anything here,
                                              // but you could!
                                              tree.add_leaf(format!("Item {}", i), |_| ())
                                          }
                                      }))
                             // Delimiter are simple lines between items,
                             // and cannot be selected.
                             .delimiter()
                             .with(|tree| {
                                 for i in 1..10 {
                                     tree.add_leaf(format!("Option {}", i), |_| ());
                                 }
                             }))
            .add_subtree("Help",
                         MenuTree::new()
                             .subtree("Help",
                                      MenuTree::new()
                                          .leaf("General", |s| {
                                              s.add_layer(Dialog::info("Help message!"))
                                          })
                                          .leaf("Online", |s| {
                                              let text = "Google it yourself!\n\
                                              Kids, these days...";
                                              s.add_layer(Dialog::info(text))
                                          }))
                             .leaf("About",
                                   |s| s.add_layer(Dialog::info("Cursive v0.0.0"))))
            .add_delimiter()
            .add_leaf("Quit", |s| s.quit());
        self.engine.set_autohide_menu(false);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn form_basic_data() {
        // Might need https://github.com/gyscos/Cursive/issues/271 for UI tests.
        // Otherwise, things can get very clunky.
    }
}
