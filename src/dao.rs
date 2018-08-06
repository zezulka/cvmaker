use base::CV;
use uuid::Uuid;
use std::path::Path;

// Basically the backend for this application. The default is reading raw files from the filesystem
// (the "primary key" is the path to the file, in this case)
// Fs-backed backend in not scalable, of course, but that's not what we want anyway (for now).
pub trait CVManager {
    fn add_cv(cv : CV) -> Result<(), String>;
    fn remove_cv(cv : CV) -> Result<(), String>;
    fn update_cv(cv : CV) -> Result<(), String>;
    fn read_cv(file_path : &str) -> CV;
}

pub struct CVManagerFileBased {
}

fn save_cv(cv : & CV) -> Result<(), String> {
    println!("{:?}", cv);
    Ok(())
}

impl CVManager for CVManagerFileBased {
    fn add_cv(mut cv: CV) -> Result<(), String> {
        match cv.path {
            Some(_) => Err("Cannot add a CV which already has the same ID.".to_string()),
            None => {
                let mut id = "/tmp/".to_string();
                id.push_str(&Uuid::new_v4().hyphenated().to_string());
                id.push_str(".json");
                let id = Path::new(&id);
                cv.set_path(id.to_path_buf());
                save_cv(& cv);
                Ok(())
            }
        }
    }

    fn remove_cv(cv: CV) -> Result<(), String> {
        match cv.path {
            None => Err("Cannot remove a CV which has no ID.".to_string()),
            Some(cv) => Ok(())
        }
    }

    fn update_cv(cv: CV) -> Result<(), String> {
        unimplemented!()
    }

    fn read_cv(file_path: &str) -> CV {
        unimplemented!()
    }
}
