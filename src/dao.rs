use base::CV;
use uuid::Uuid;
use std::path::Path;
use serde_json;
use std::collections::HashSet;
use vfs::{VFile, VMetadata, VPath, VFS, PhysicalFS, MemoryFS};
use base::basic_cv_factory;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_id_gen_uniqueness() {
        let iterations = 1000;
        let mut ids : HashSet<String> = HashSet::new();
        for _ in 0..iterations {
            ids.insert(gen_id());
        }
        assert!(ids.len() >= (iterations - 1));
    }

    #[test]
    fn fail_when_add_cv_with_id() {
        let fs = MemoryFS::new();
        let manager = CVManagerFileBased::<MemoryFS>::new_testing();
        let mut cv = basic_cv_factory();
        cv.set_path(Box::new(fs.path("")));
        assert_eq!(Err("Cannot add a CV which already an ID.".to_string()), manager.add_cv(&mut cv));
    }

    #[test]
    fn add_cv_then_id_nonempty() {
        let manager = CVManagerFileBased::<MemoryFS>::new_testing();
        let mut cv = basic_cv_factory();
        assert!(cv.path.is_none());
        manager.add_cv(&mut cv);
        assert!(cv.path.is_some());
    }

    #[test]
    fn add_cv_then_assert_existence() {
        let manager = CVManagerFileBased::<MemoryFS>::new_testing();
        let mut cv = basic_cv_factory();
        assert_eq!(0, manager.backend.path("/").read_dir().unwrap().count());
        manager.add_cv(&mut cv);
        // Assert that SOMETHING happened to the fs.
        assert_ne!(0, manager.backend.path("/").read_dir().unwrap().count());
    }
}

// Basically the backend for this application. The default is reading raw files from the filesystem
// (the "primary key" is the path to the file, in this case)
// Fs-backed backend in not scalable, of course, but that's not what we want anyway (for now).
pub trait CVManager {
    fn add_cv(&self, cv : &mut CV) -> Result<(), String>;
    fn remove_cv(&self, cv : &mut CV) -> Result<(), String>;
    fn update_cv(&self, cv : &CV) -> Result<(), String>;
    fn read_cv(&self, file_path : &str) -> CV;
}

pub struct CVManagerFileBased<T : VFS> {
    //TODO fs-related attrs go here...
    cvs_path: String,
    backend: T,
}

impl<T> CVManagerFileBased<T> where T : VFS {
    //TODO tmpfs is not a very good storage for permanent data :)
    // for testing the program out, this should be enough though
    pub fn new() -> CVManagerFileBased<PhysicalFS> {
        CVManagerFileBased { cvs_path : "/tmp".to_string(), backend : PhysicalFS{} }
    }

    fn new_testing() -> CVManagerFileBased<MemoryFS> {
        CVManagerFileBased { cvs_path : "".to_string(), backend : MemoryFS::new() }
    }

    // Saves a cv as a JSON to a file.
    // Panics when there is no id set in the cv attribute.
    fn save_cv(&self, cv : & CV) -> Result<(), String> {
        if cv.path.is_none() {
            panic!("CV must have a valid ID to be saved.");
        }
        let json_str = serde_json::to_string(&cv);
        if let Err(err) = json_str {
            return Err(err.to_string());
        }
        let json_str = json_str.unwrap();
        let path = self.backend.path(cv.path.as_ref().unwrap().to_string());
        if let Some(p) = path.parent() {
            p.mkdir();
        }
        if let Ok(mut vfile) = path.create() {
            vfile.write(json_str.as_bytes());
        }
        Ok(())
    }
}

// Generates a unique id. The implementation is based on the UUID concept.
fn gen_id() -> String {
    Uuid::new_v4().simple().to_string()
}

// TODO: deal with duplicates (which are VERY rare to occur, but possible nevertheless!)
// TODO: only UNIX path separators
impl<T> CVManager for CVManagerFileBased<T> where T : VFS, T::PATH: 'static {
    fn add_cv(&self, cv: &mut CV) -> Result<(), String> {
        match cv.path {
            Some(_) => Err("Cannot add a CV which already an ID.".to_string()),
            None => {
                let mut id = self.cvs_path.to_string();
                id.push_str("/");
                id.push_str(&gen_id());
                id.push_str(".json");
                let id = self.backend.path(id);
                cv.set_path(Box::new(id));
                self.save_cv(& cv);
                Ok(())
            }
        }
    }

    fn remove_cv(&self, cv: &mut CV) -> Result<(), String> {
        match &cv.path {
            None => Err("Cannot remove a CV which has no ID.".to_string()),
            Some(cv) => Ok(())
        }
    }

    fn update_cv(&self, cv: &CV) -> Result<(), String> {
        unimplemented!()
    }

    fn read_cv(&self, file_path: &str) -> CV {
        unimplemented!()
    }
}
