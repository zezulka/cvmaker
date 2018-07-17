extern crate base;
use base::CV;

// Basically the backend for this application. The default is reading raw files from the filesystem
// (the "primary key" is the path to the file, in this case)
// Fs-backed backend in not scalable, of course, but that's not what we want anyway (for now).
trait CVManager {
    fn add_cv(cv : CV) -> Result<(), String>;
    fn remove_cv(cv : CV) -> Result<(), String>;
    fn update_cv(cv : CV) -> Result<(), String>;
    fn read_cv(file_path : &str) -> CV;
}

struct CVManagerFileBased {
}


impl CVManager for CVManagerFileBased {
    fn add_cv(cv: CV) -> Result<(), String> {
        unimplemented!()
    }

    fn remove_cv(cv: CV) -> Result<(), String> {
        unimplemented!()
    }

    fn update_cv(cv: CV) -> Result<(), String> {
        unimplemented!()
    }

    fn read_cv(file_path: &str) -> CV {
        unimplemented!()
    }
}
