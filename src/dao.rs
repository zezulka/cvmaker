use base::CV;
use serde_json;
use uuid::Uuid;
use vfs::{MemoryFS, PhysicalFS, VPath, VFS};

pub type CVDao = CVManagerFileBased<PhysicalFS>;

// The backend for this application. The default is reading raw files from the filesystem
// (the "primary key" is the path to the file, in this case)
// Fs-backed backend in not performance-scalable, of course, but that's not what we want anyway.
pub trait CVManager {
    fn add_cv(&self, cv: &mut CV) -> Result<(), String>;
    fn remove_cv(&self, cv: &mut CV) -> Result<(), String>;
    fn update_cv(&self, cv: &CV) -> Result<(), String>;
    fn read_cv(&self, file_path: &str) -> Result<CV, String>;
}

pub struct CVManagerFileBased<T: VFS> {
    cvs_path: String,
    backend: T,
}

impl<T> CVManagerFileBased<T>
where
    T: VFS,
{
    //TODO tmpfs is not a very good storage for permanent data :)
    // for testing the program out, this should be enough though
    pub fn new() -> CVManagerFileBased<PhysicalFS> {
        CVManagerFileBased {
            cvs_path: "/tmp".to_string(),
            backend: PhysicalFS {},
        }
    }

    //TODO use memory FS in tests
    #[allow(dead_code)]
    fn new_testing() -> CVManagerFileBased<MemoryFS> {
        CVManagerFileBased {
            cvs_path: "".to_string(),
            backend: MemoryFS::new(),
        }
    }

    // Saves a cv as a JSON to a file.
    // Panics:
    //      when there is no id set in the cv attribute.
    fn save_cv(&self, cv: &CV) -> Result<(), String> {
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
            return match p.mkdir() {
                Ok(_) => {
                    if let Ok(mut vfile) = path.create() {
                        if let Err(err) = vfile.write(json_str.as_bytes()) {
                            eprintln!("{}", err);
                        }
                    }
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            };
        }
        Err("Wrong path given.".to_string())
    }
}

// Generates a unique id. The implementation is based on the UUID concept.
fn gen_id() -> String {
    Uuid::new_v4().simple().to_string()
}

// TODO: deal with duplicates (which are VERY rare to occur, but possible nevertheless!)
// TODO: only UNIX path separators
impl<T> CVManager for CVManagerFileBased<T>
where
    T: VFS,
    T::PATH: 'static,
{
    fn add_cv(&self, cv: &mut CV) -> Result<(), String> {
        match cv.path {
            Some(_) => Err("Cannot add a CV which already has an ID.".to_string()),
            None => {
                let mut id = self.cvs_path.to_string();
                id.push_str("/");
                id.push_str(&gen_id());
                id.push_str(".json");
                let id = self.backend.path(id);
                cv.set_path(Box::new(id));
                self.save_cv(&cv)
            }
        }
    }

    fn remove_cv(&self, cv: &mut CV) -> Result<(), String> {
        match &cv.path {
            None => Err("Cannot remove a CV which has no ID.".to_string()),
            Some(path) => {
                let path_str = path.as_str();
                let path = self.backend.path(path_str);
                if !path.exists() {
                    return Err(format!("The path '{}' does not exist.", path_str));
                }
                match path.rm() {
                    Err(_) => Err(format!("Could not remove the file: '{}'", path_str)),
                    Ok(_) => Ok(()),
                }
            }
        }
    }

    fn update_cv(&self, cv: &CV) -> Result<(), String> {
        match &cv.path {
            None => Err("Cannot update a CV which has no ID.".to_string()),
            Some(_) => self.save_cv(&cv),
        }
    }

    fn read_cv(&self, file_path: &str) -> Result<CV, String> {
        let path = self.backend.path(file_path);
        match path.open() {
            Err(_) => Err(format!("Couldn't open file {}", file_path)),
            Ok(mut vfile) => {
                let mut buff = String::new();
                match &vfile.read_to_string(&mut buff) {
                    Err(_) => Err(format!("Coudn't read file {}", file_path)),
                    Ok(_) => match serde_json::from_str(&buff) {
                        Ok(cv) => Ok(cv),
                        Err(err) => Err(err.to_string()),
                    },
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;
    use base::basic_cv_factory;
    use std::collections::HashSet;
    type CVDao = CVManagerFileBased<MemoryFS>;
    #[test]
    fn test_id_gen_uniqueness() {
        let iterations = 1000;
        let mut ids: HashSet<String> = HashSet::new();
        for _ in 0..iterations {
            ids.insert(gen_id());
        }
        assert!(ids.len() >= (iterations - 1));
    }

    #[test]
    fn fail_when_add_cv_with_id() {
        let fs = MemoryFS::new();
        let manager = CVDao::new_testing();
        let mut cv = basic_cv_factory();
        cv.set_path(Box::new(fs.path("")));
        assert_eq!(
            Err("Cannot add a CV which already has an ID.".to_string()),
            manager.add_cv(&mut cv)
        );
    }

    #[test]
    fn add_cv_then_id_nonempty() {
        let manager = CVDao::new_testing();
        let mut cv = basic_cv_factory();
        assert!(cv.path.is_none());
        manager.add_cv(&mut cv);
        assert!(cv.path.is_some());
    }

    fn count_num_files(mgr: &CVDao) -> usize {
        mgr.backend.path("/").read_dir().unwrap().count()
    }

    #[test]
    fn add_cv_then_assert_existence() {
        let manager = CVDao::new_testing();
        let mut cv = basic_cv_factory();
        assert_eq!(0, count_num_files(&manager));
        manager.add_cv(&mut cv);
        // Assert that SOMETHING happened to the fs.
        assert_ne!(0, count_num_files(&manager));
    }

    #[test]
    fn remove_cv_happy_scenario() {
        let manager = CVDao::new_testing();
        let mut cv = basic_cv_factory();
        assert_eq!(Ok(()), manager.add_cv(&mut cv));
        let before = count_num_files(&manager);
        assert_eq!(Ok(()), manager.remove_cv(&mut cv));
        let after = count_num_files(&manager);
        assert_ne!(before, after);
    }

    #[test]
    fn remove_cv_no_id() {
        let manager = CVDao::new_testing();
        let mut cv = basic_cv_factory();
        assert_eq!(
            Err("Cannot remove a CV which has no ID.".to_string()),
            manager.remove_cv(&mut cv)
        );
    }

    #[test]
    fn remove_cv_nonexistent_path() {
        let manager = CVDao::new_testing();
        let mut cv = basic_cv_factory();
        cv.set_path(Box::new(manager.backend.path("/foobar")));
        assert_eq!(
            Err("The path '/foobar' does not exist.".to_string()),
            manager.remove_cv(&mut cv)
        );
    }

    #[test]
    fn update_cv_no_id() {
        let manager = CVDao::new_testing();
        let mut cv = basic_cv_factory();
        assert_eq!(
            Err("Cannot update a CV which has no ID.".to_string()),
            manager.update_cv(&cv)
        );
    }

    #[test]
    fn update_cv_happy_scenario() {
        //TODO implement read_cv to test update
    }

    #[test]
    fn read_cv_happy_scenario() {
        let manager = CVDao::new_testing();
        let mut cv = basic_cv_factory();
        let mut cv_copy = basic_cv_factory();
        assert_eq!(Ok(()), manager.add_cv(&mut cv));
        cv_copy.path = cv.path.clone();
        assert_eq!(Ok(cv_copy), manager.read_cv(&cv.path.unwrap()));
    }

}
