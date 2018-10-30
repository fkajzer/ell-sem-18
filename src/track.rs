use std::path::PathBuf;
use formatter;
use std::panic;
use regex::Captures;

// TODO derive allows for printing, maybe not needed at the end
#[derive(Debug)]
pub struct Track {
    pub original_name: String,
    pub file_name: String,
    pub short_name: String,
    pub author: String,
    pub author_plus: String,
    pub title: String,
    pub title_plus: String,
    pub version: String,
    pub version_plus: String,
    pub release_year: String,
    pub extension: String,
}

impl Track {
    // TODO Create a new Track from PathBuf.
    pub fn new(path_buf: &PathBuf) -> Track {
        let file_info = formatter::apply_regular_expression(path_buf.to_str().unwrap());

        Track {
            // TODO remove original_name
            original_name: String::from(path_buf.to_str().unwrap()),
            file_name: String::from(path_buf.file_stem().unwrap().to_str().unwrap()),
            short_name: String::new(),
            author: get_or_empty(&file_info, "author"),
            author_plus: String::new(),
            title: get_or_empty(&file_info, "title"),
            title_plus: String::new(),
            version: get_or_empty(&file_info, "version"),
            version_plus: String::new(),
            release_year: get_or_empty(&file_info, "year"),
            extension: String::from(path_buf.extension().unwrap().to_str().unwrap()),
        }
    }
}

fn get_or_empty(file_info: &Captures, index: &str) -> String {
    let result = panic::catch_unwind(|| {
        &file_info[index];
    });

    if result.is_err() {
        return String::new();
    }

    String::from(file_info[index].to_owned())
}
