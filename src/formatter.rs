use regex::Regex;
use regex::Captures;
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
