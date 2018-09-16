use self::datepicker::{DatePicker, DateView};
use base::contact_types;
use base::LanguageProficiency;
use base::TimeSpan;
use base::{
    BasicInfo, CVBuilder, Contact, Education, EmailAddress, Experience, Lang, Language, CV,
};
use cursive::align::HAlign;
use cursive::event::Key;
use cursive::menu::MenuTree;
use cursive::traits::*;
use cursive::view::Selector;
use cursive::views::{
    BoxView, Button, Canvas, Dialog, EditView, IdView, LinearLayout, SelectView, TextContent,
    TextView,
};
use cursive::Cursive;
use dao::{CVDao, CVManager};
use phonenumber::PhoneNumber;
use renderer::render_pdf;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use url::Url;

mod datepicker;

fn select_view_from_range<S: Display + 'static, T: Iterator<Item = S>>(rng: T) -> SelectView<S> {
    let mut sel_view: SelectView<S> = SelectView::new().h_align(HAlign::Center);
    rng.for_each(|item| sel_view.add_item(format!("{}", item), item));
    sel_view.popup()
}

static CONTACTS_ID: &'static str = "contacts";
static CONTACT_CHILD_ID: &'static str = "contact_child";
static EXP_ID: &'static str = "experience";
static EXP_CHILD_ID: &'static str = "experience_child";
//TODO inspect whether this will be needed (the same for FORM_ROOT_ID)
#[allow(dead_code)]
static LANGS_ID: &'static str = "languages";
static LANG_CHILD_ID: &'static str = "language_child";
static EDU_ID: &'static str = "education";
static EDU_CHILD_ID: &'static str = "education_child";
#[allow(dead_code)]
static FORM_ROOT_ID: &'static str = "form_root";

pub struct Graphics {
    engine: Cursive,
}

impl Graphics {
    pub fn new(engine: Cursive) -> Graphics {
        Graphics { engine }
    }

    fn setup_looks(&mut self) {
        if let Err(msg) = self.engine.load_theme_file("src/resources/theme.toml") {
            // We could also panic but receiving the error only means
            // we fall back to the default theme which is ok for the time being.
            eprintln!("Could not load the themes file: {:?}", msg);
        }
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        self.init()?;
        self.engine.run();
        Ok(())
    }

    fn init(&mut self) -> Result<(), Box<Error>> {
        self.setup_looks();
        self.add_menu();
        self.engine.add_layer(Canvas::new(()));
        self.add_form();
        self.engine
            .add_global_callback(Key::Esc, |s| s.select_menubar());
        Ok(())
    }

    // Creates a form row containing description on the left and an editable field on the right.
    // The label must be nonempty.
    fn form_row(label_text: &str, col_size: usize) -> LinearLayout {
        if label_text.is_empty() {
            panic!("Got empty label text, expected nonempty.");
        }
        let data_child = EditView::new().fixed_width(col_size).with_id(label_text);
        let mut result = LinearLayout::horizontal()
            .child(TextView::new_with_content(TextContent::new(label_text)).fixed_width(col_size))
            .child(data_child);
        // TODO: Helper "assertion" which should be removed after the problem is fixed.
        result
            .get_child_mut(1)
            .unwrap()
            .as_any_mut()
            .downcast_ref::<IdView<BoxView<EditView>>>()
            .unwrap();
        result
    }

    fn form_row_default_col_size(label_text: &str) -> LinearLayout {
        Self::form_row(label_text, 20)
    }

    fn contact_select_view() -> SelectView<String> {
        let mut sel_view: SelectView<String> = SelectView::new().h_align(HAlign::Center);
        contact_types()
            .iter()
            .for_each(|&item| sel_view.add_item(item, item.to_string()));
        sel_view.popup()
    }

    fn contact_row(s: &mut Cursive) {
        s.call_on_id(CONTACTS_ID, |view: &mut LinearLayout| {
            view.add_child(
                LinearLayout::horizontal()
                    .child(Self::contact_select_view())
                    .child(EditView::new().fixed_width(20))
                    .with_id(CONTACT_CHILD_ID),
            )
        });
    }

    fn experience_row(s: &mut Cursive) {
        s.call_on_id(EXP_ID, |view: &mut LinearLayout| {
            view.add_child(
                LinearLayout::vertical()
                    .child(DateView::new_without_days("From"))
                    .child(DateView::new_without_days("To"))
                    .child(Self::form_row_default_col_size("Employer"))
                    .child(Self::form_row_default_col_size("Job name"))
                    .child(Self::form_row_default_col_size("Description"))
                    .with_id(EXP_CHILD_ID),
            )
        });
    }

    //TODO : if user enters the "Other" option, let him fill in the "other" language
    fn language_row(s: &mut Cursive) {
        s.call_on_id(LANGS_ID, |view: &mut LinearLayout| {
            view.add_child(
                LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                            .child(
                                TextView::new_with_content(TextContent::new("Language name"))
                                    .fixed_width(20),
                            ).child(select_view_from_range(Language::iterator())),
                    ).child(
                        LinearLayout::horizontal()
                            .child(
                                TextView::new_with_content(TextContent::new("Proficiency"))
                                    .fixed_width(20),
                            ).child(select_view_from_range(LanguageProficiency::iterator())),
                    ).child(Self::form_row_default_col_size("Additional notes"))
                    .with_id(LANG_CHILD_ID),
            )
        });
    }

    fn education_row(s: &mut Cursive) {
        s.call_on_id(EDU_ID, |view: &mut LinearLayout| {
            view.add_child(
                LinearLayout::vertical()
                    .child(DateView::new_without_days("From"))
                    .child(DateView::new_without_days("To"))
                    .child(Self::form_row_default_col_size("University"))
                    .child(Self::form_row_default_col_size("Degree"))
                    .child(Self::form_row_default_col_size("Field of study"))
                    .with_id(EDU_CHILD_ID),
            )
        });
    }

    fn expandable_linear_layout_contacts(event_fun: &'static Fn(&mut Cursive)) -> LinearLayout {
        LinearLayout::vertical()
            .child(
                LinearLayout::horizontal().child(
                    TextView::new_with_content(TextContent::new("Contacts")).fixed_width(20),
                ),
            ).child(
                LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                            .child(Self::contact_select_view())
                            .child(EditView::new().fixed_width(20))
                            .with_id(CONTACT_CHILD_ID),
                    ).with_id(CONTACTS_ID),
            ).child(LinearLayout::horizontal().child(Button::new("Add another", event_fun)))
    }

    fn first_uppercase(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    fn expandable_linear_layout<'a>(
        label: &'a str,
        event_fun: &'static Fn(&mut Cursive),
    ) -> LinearLayout {
        LinearLayout::vertical()
            .child(
                LinearLayout::horizontal().child(
                    TextView::new_with_content(TextContent::new(Self::first_uppercase(label)))
                        .fixed_width(20),
                ),
            ).child(LinearLayout::vertical().with_id(label))
            .child(LinearLayout::horizontal().child(Button::new("Add another", event_fun)))
    }

    fn add_form(&mut self) {
        let form = LinearLayout::vertical()
            .child(Self::form_row_default_col_size("Name"))
            .child(Self::form_row_default_col_size("Surname"))
            .child(DateView::new_full("Date of birth"))
            .child(Self::expandable_linear_layout_contacts(&Self::contact_row))
            .child(Self::expandable_linear_layout(
                LANGS_ID,
                &Self::language_row,
            )).child(Self::expandable_linear_layout(EDU_ID, &Self::education_row))
            .child(Self::expandable_linear_layout(
                EXP_ID,
                &Self::experience_row,
            ))
            //TODO should dynamically fit to the content, this is just a hot fix
            .fixed_height(2000)
            .scrollable();
        self.engine.add_layer(
            Dialog::around(form)
                .title("New CV")
                .button("Create new CV", |s| {
                    if let Some(mut cv) = Self::collect_form_data(s) {
                        let manager = CVDao::new();
                        match manager.add_cv(&mut cv) {
                            Ok(_) => {
                                println!(
                                    "CV with id {} added successfully.",
                                    cv.path.as_ref().unwrap()
                                );
                                match render_pdf(&cv) {
                                    Err(e) => eprintln!("Could not render PDF:\n\t {}", e),
                                    Ok(_) => println!("CV rendered successfully."),
                                }
                            }
                            Err(e) => println!("{:?}", e),
                        }
                    }
                }),
        );
    }

    fn get_data_form_row(view: &mut View) -> Option<String> {
        let data_index = 1;
        let aux = view
            .as_any_mut()
            .downcast_mut::<LinearLayout>()
            .unwrap()
            .get_child_mut(data_index)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<IdView<BoxView<EditView>>>()
            .unwrap();
        let data = aux.get_mut().get_inner().get_content();
        Some(data.to_string())
    }

    fn collect_contacts(c: &mut Cursive) -> Vec<Contact> {
        let mut res = vec![];
        let mut contacts_root = c.find_id::<LinearLayout>(CONTACTS_ID).unwrap();
        contacts_root.call_on_any(
            &Selector::Id(CONTACT_CHILD_ID),
            Box::new(|s| {
                if let Some(id_view) = s.downcast_mut::<IdView<LinearLayout>>() {
                    let mut lin_lay = id_view.get_mut();
                    let data = lin_lay
                        .get_child_mut(1)
                        .unwrap()
                        .as_any_mut()
                        .downcast_mut::<BoxView<EditView>>()
                        .unwrap()
                        .get_inner_mut()
                        .get_content();
                    match &lin_lay
                        .get_child_mut(0)
                        .unwrap()
                        .as_any_mut()
                        .downcast_mut::<SelectView>()
                        .unwrap()
                        .selection()
                        .unwrap()[..]
                    {
                        //TODO: handle all error cases (dialog box to the user, maybe?)
                        "email" => {
                            if let Ok(address) = EmailAddress::from(&data.to_string()) {
                                res.push(Contact::Email(address));
                            }
                        }
                        "website" => {
                            if let Ok(url) = Url::from_str(&data) {
                                res.push(Contact::Website(url));
                            }
                        }
                        "phone" => {
                            if let Ok(number) = PhoneNumber::from_str(&data) {
                                res.push(Contact::Phone(number));
                            }
                        }
                        _ => panic!("Unexpected selection."),
                    }
                }
            }),
        );
        res
    }

    fn collect_experience(c: &mut Cursive) -> Vec<Experience> {
        let mut res = vec![];
        let mut experience_root = c
            .find_id::<LinearLayout>(EXP_ID)
            .expect("Could not find the root of the experience.");
        experience_root.call_on_any(
            &Selector::Id(EXP_CHILD_ID),
            Box::new(|s| {
                if let Some(id_view) = s.downcast_mut::<IdView<LinearLayout>>() {
                    let mut lin_lay = id_view.get_mut();
                    let (from, to, employer, job_name, description) = (0, 1, 2, 3, 4);
                    let from = lin_lay
                        .get_child_mut(from)
                        .unwrap()
                        .as_any_mut()
                        .downcast_mut::<IdView<DateView>>()
                        .unwrap()
                        .get_mut()
                        .retrieve_date();
                    let to = lin_lay
                        .get_child_mut(to)
                        .unwrap()
                        .as_any_mut()
                        .downcast_mut::<IdView<DateView>>()
                        .unwrap()
                        .get_mut()
                        .retrieve_date();
                    let employer = Self::get_data_form_row(
                        lin_lay
                            .get_child_mut(employer)
                            .expect("could not retrieve employer row"),
                    );
                    let job_name = Self::get_data_form_row(
                        lin_lay
                            .get_child_mut(job_name)
                            .expect("could not retrieve job_name row"),
                    );
                    let description = Self::get_data_form_row(
                        lin_lay
                            .get_child_mut(description)
                            .expect("could not retrieve employer row"),
                    );
                    if let (
                        Some(from),
                        Some(to),
                        Some(employer),
                        Some(job_name),
                        Some(description),
                    ) = (from, to, employer, job_name, description)
                    {
                        res.push(Experience {
                            span: TimeSpan::new(from, to),
                            employer,
                            job_name,
                            description,
                        });
                    }
                }
            }),
        );
        res
    }

    // Refactor with collect_experience
    fn collect_education(c: &mut Cursive) -> Vec<Education> {
        let mut res = vec![];
        let mut experience_root = c
            .find_id::<LinearLayout>(EXP_ID)
            .expect("Could not find the root of the education.");
        experience_root.call_on_any(
            &Selector::Id(EDU_CHILD_ID),
            Box::new(|s| {
                if let Some(id_view) = s.downcast_mut::<IdView<LinearLayout>>() {
                    let mut lin_lay = id_view.get_mut();
                    let from = 0;
                    let to = 1;
                    let uni_name = 2;
                    let degree = 3;
                    let field_of_study = 4;
                    let from = lin_lay
                        .get_child_mut(from)
                        .unwrap()
                        .as_any_mut()
                        .downcast_mut::<IdView<DateView>>()
                        .unwrap()
                        .get_mut()
                        .retrieve_date();
                    let to = lin_lay
                        .get_child_mut(to)
                        .unwrap()
                        .as_any_mut()
                        .downcast_mut::<IdView<DateView>>()
                        .unwrap()
                        .get_mut()
                        .retrieve_date();
                    let uni_name =
                        Self::get_data_form_row(lin_lay.get_child_mut(uni_name).unwrap());
                    let degree = Self::get_data_form_row(lin_lay.get_child_mut(degree).unwrap());
                    let field_of_study =
                        Self::get_data_form_row(lin_lay.get_child_mut(field_of_study).unwrap());
                    if let (
                        Some(from),
                        Some(to),
                        Some(uni_name),
                        Some(degree),
                        Some(field_of_study),
                    ) = (from, to, uni_name, degree, field_of_study)
                    {
                        res.push(Education {
                            span: TimeSpan::new(from, to),
                            uni_name,
                            degree,
                            field_of_study,
                        });
                    }
                }
            }),
        );
        res
    }

    fn collect_languages(c: &mut Cursive) -> Vec<Lang> {
        let mut res = vec![];
        let mut experience_root = c
            .find_id::<LinearLayout>(EXP_ID)
            .expect("Could not find the root of the education.");
        experience_root.call_on_any(
            &Selector::Id(EDU_CHILD_ID),
            Box::new(|s| {
                if let Some(id_view) = s.downcast_mut::<IdView<LinearLayout>>() {
                    let mut lin_lay = id_view.get_mut();
                    let language = 0;
                    let proficiency = 1;
                    let notes = 2;
                    let language = lin_lay
                        .get_child_mut(language)
                        .unwrap()
                        .as_any_mut()
                        .downcast_mut::<SelectView<Language>>()
                        .unwrap()
                        .selection();
                    let proficiency = lin_lay
                        .get_child_mut(proficiency)
                        .unwrap()
                        .as_any_mut()
                        .downcast_mut::<SelectView<LanguageProficiency>>()
                        .unwrap()
                        .selection();
                    let notes = Self::get_data_form_row(lin_lay.get_child_mut(notes).unwrap());
                    if let (Some(language), Some(proficiency), Some(notes)) =
                        (language, proficiency, notes)
                    {
                        if let (Ok(language), Ok(proficiency)) =
                            (Rc::try_unwrap(language), Rc::try_unwrap(proficiency))
                        {
                            res.push(Lang {
                                language,
                                proficiency,
                                notes,
                            });
                        }
                    }
                }
            }),
        );
        res
    }

    fn collect_basic_info(c: &mut Cursive) -> Result<BasicInfo, &str> {
        let name = c
            .call_on_id("Name", |s: &mut BoxView<EditView>| {
                s.get_inner().get_content()
            }).unwrap();
        let surname = c
            .call_on_id("Surname", |s: &mut BoxView<EditView>| {
                s.get_inner().get_content()
            }).unwrap();
        let dob = c
            .call_on_id("Date of birth", |s: &mut DateView| s.retrieve_date())
            .unwrap()
            .unwrap();
        let contacts = Self::collect_contacts(c);
        if contacts.is_empty() {
            return Err("There must be at least one valid contact filled in.");
        }
        Ok(BasicInfo::new(&name, &surname, dob, contacts))
    }

    // This handler is responsible for collecting the data from the "New CV" form.
    pub fn collect_form_data(c: &mut Cursive) -> Option<CV> {
        let mut error = String::new();
        let basic = match Self::collect_basic_info(c) {
            Ok(b) => Some(b),
            Err(e) => {
                error = e.to_string();
                None
            }
        };
        if let Some(basic) = basic {
            return match CVBuilder::default(basic)
                .experience(Self::collect_experience(c))
                .education(Self::collect_education(c))
                .languages(Self::collect_languages(c))
                .build()
            {
                Ok(cv) => Some(cv),
                Err(err) => {
                    error += "\n";
                    error += &err;
                    None
                }
            };
        }
        c.add_layer(Dialog::info(error));
        None
    }

    // Fearlessly stolen from the cursive example.
    fn add_menu(&mut self) {
        // We'll use a counter to name new files.
        let counter = AtomicUsize::new(1);

        // The menubar is a list of (label, menu tree) pairs.
        self.engine
            .menubar()
            // We add a new "File" tree
            .add_subtree(
                "File",
                MenuTree::new()
                    // Trees are made of leaves, with are directly actionable...
                    .leaf("New", move |s| {
                        // Here we use the counter to add an entry
                        // in the list of "Recent" items.
                        let i = counter.fetch_add(1, Ordering::Relaxed);
                        let filename = format!("New {}", i);
                        s.menubar()
                            .find_subtree("File")
                            .unwrap()
                            .find_subtree("Recent")
                            .unwrap()
                            .insert_leaf(0, filename, |_| ());

                        s.add_layer(Dialog::info("New file!"));
                    })
                    // ... and of sub-trees, which open up when selected.
                    .subtree(
                        "Recent",
                        // The `.with()` method can help when running loops
                        // within builder patterns.
                        MenuTree::new().with(|tree| {
                            for i in 1..100 {
                                // We don't actually do anything here,
                                // but you could!
                                tree.add_leaf(format!("Item {}", i), |_| ())
                            }
                        }),
                    )
                    // Delimiter are simple lines between items,
                    // and cannot be selected.
                    .delimiter()
                    .with(|tree| {
                        for i in 1..10 {
                            tree.add_leaf(format!("Option {}", i), |_| ());
                        }
                    }),
            ).add_subtree(
                "Help",
                MenuTree::new()
                    .subtree(
                        "Help",
                        MenuTree::new()
                            .leaf("General", |s| s.add_layer(Dialog::info("Help message!")))
                            .leaf("Online", |s| {
                                let text = "Google it yourself!\n\
                                            Kids, these days...";
                                s.add_layer(Dialog::info(text))
                            }),
                    ).leaf("About", |s| s.add_layer(Dialog::info("Cursive v0.0.0"))),
            ).add_delimiter()
            .add_leaf("Quit", |s| s.quit());
        self.engine.set_autohide_menu(false);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn form_basic_data() {
        // Might need https://github.com/gyscos/Cursive/issues/271 for UI tests.
        // Otherwise, things can get very clunky.
    }
}
