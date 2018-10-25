use std::ffi::OsString;
use std::path::PathBuf;

pub struct Track {
    pub file_name: OsString,
    pub author: String,
    pub author_plus: String,
    pub title: String,
    pub title_plus: String,
    pub version: String,
    pub version_plus: String,
    pub release_year: i32,
}

impl Track {
    // TODO Create a new Track from PathBuf.
    pub fn new(path_buf: &PathBuf) -> Track {
        Track {
            file_name: get_file_name(path_buf),
            author: String::new(),
            author_plus: String::new(),
            title: String::new(),
            title_plus: String::new(),
            version: String::new(),
            version_plus: String::new(),
            release_year: 0,
        }
    }
}

fn get_file_name(path_buf: &PathBuf) -> OsString {
    path_buf.file_name().unwrap().to_os_string()
}

