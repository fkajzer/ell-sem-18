use regex::Regex;
use regex::Captures;
use track::Track;
// use std::panic;

pub fn apply_regular_expression(file_info: &str) -> Captures {
    // Avoid compiling the same regex in a loop
    // https://docs.rs/regex/1.0.5/regex/#example-avoid-compiling-the-same-regex-in-a-loop
    lazy_static! {
        static ref YATV: Regex = Regex::new(
            r".*/(?P<year>\d{4})/(?P<author>.*) - (?P<title>.*) \((?P<version>.*)\)\.")
                       .unwrap();
        static ref ATVY: Regex = Regex::new(
            r".*/(?P<author>.*) - (?P<title>.*) \((?P<version>.*)\)\.(?P<year>\d{4})\.")
                .unwrap();
        static ref YAT: Regex = Regex::new(
            r".*/(?P<year>\d{4})/(?P<author>.*) - (?P<title>.*)\.")
                .unwrap();
        static ref ATY: Regex = Regex::new(
            r".*/(?P<author>.*) - (?P<title>.*)\.(?P<year>\d{4})")
                .unwrap();
        static ref ATV: Regex = Regex::new(
            r".*/(?P<author>.*) - (?P<title>.*) \((?P<version>.*)\)\.")
                .unwrap();
        static ref AT: Regex = Regex::new(
            r".*/(?P<author>.*) - (?P<title>.*)\.")
                .unwrap();
        static ref T: Regex = Regex::new(
            r".*/(?P<title>.*)\.")
                .unwrap();
    }

    // println!("{:?}", "***********FILE***********");
    // println!("{:?}", file_info);

    let caps = match YATV.is_match(file_info) {
        true => YATV.captures(file_info).unwrap(),
        false => match ATVY.is_match(file_info) {
            true => ATVY.captures(file_info).unwrap(),
            false => match YAT.is_match(file_info) {
                true => YAT.captures(file_info).unwrap(),
                false => match ATY.is_match(file_info) {
                    true => ATY.captures(file_info).unwrap(),
                    false => match ATV.is_match(file_info) {
                        true => ATV.captures(file_info).unwrap(),
                        false => match AT.is_match(file_info) {
                            true => AT.captures(file_info).unwrap(),
                            false => T.captures(file_info).unwrap()
                        }
                    }
                }
            }
        }
    };
    // print_caps(&caps);
    caps
}

pub fn format_info(track: &mut Track) {
    // Avoid compiling the same regex in a loop
    // https://docs.rs/regex/1.0.5/regex/#example-avoid-compiling-the-same-regex-in-a-loop
    lazy_static! {
        static ref AUTHOR_SHORT: Regex = Regex::new(
            r"(?P<author>.*) (feat|ft|presents|pres|with|introduce) (?P<author_plus>.*)")
                    .unwrap();
        static ref TITLE_SHORT: Regex = Regex::new(
            r"(?P<title>.*) (?P<title_plus>\(.*\))")
                    .unwrap();
        static ref VERSION_SHORT: Regex = Regex::new(
            r"(?P<version>.*) (?P<version_plus>\(.*\))")
                    .unwrap();
    }
    if !track.author.is_empty() {
        let original_val = String::from(track.author.as_str());
        track.author = match AUTHOR_SHORT.is_match(&track.author) {
            true => {
                let caps = AUTHOR_SHORT.captures(&track.author).unwrap();
                track.author_plus = String::from(&caps["author_plus"]);

                String::from(&caps["author"]) + "_"
            },
            false => original_val
        };
    }

    if !track.title.is_empty() {
        let original_val = String::from(track.title.as_str());
        track.title = match TITLE_SHORT.is_match(&track.title) {
            true => {
                let caps = TITLE_SHORT.captures(&track.title).unwrap();
                track.title_plus = String::from(&caps["title_plus"]);

                String::from(&caps["title"]) + "_"
            },
            false => original_val
        };
    }

    if !track.version.is_empty() {
        let original_val = String::from(track.version.as_str());
        track.version = match VERSION_SHORT.is_match(&track.version) {
            true => {
                let caps = TITLE_SHORT.captures(&track.version).unwrap();
                track.version_plus = String::from(&caps["version_plus"]);

                String::from(&caps["version"]) + "_"
            }
            false => original_val
        };
    }
}

/*
fn print_caps(caps: &Captures) {
    print_or_error(caps, "year");
    print_or_error(caps, "author");
    print_or_error(caps, "title");
    print_or_error(caps, "version");
}

fn print_or_error(caps: &Captures, index: &str) {
    let result = panic::catch_unwind(|| {
        println!("{:?} {:?}", index , &caps[index])
    });

    if result.is_err() {
        println!("No {:?} found", index)
    }
}*/
