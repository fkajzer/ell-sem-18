use regex::Regex;
use regex::Captures;
use regex::RegexBuilder;

use track::Track;

const TOO_LONG: usize = 70;

pub fn apply_regular_expression(file_info: &str) -> Captures {
    // Avoid compiling the same regex in a loop
    // https://docs.rs/regex/1.0.5/regex/#example-avoid-compiling-the-same-regex-in-a-loop
    lazy_static! {
        static ref YATVV2: Regex = RegexBuilder::new(
            r".*/(?P<year>\d{4})/(?P<author>.*) - (?P<title>.*) \((?P<version>.*) \((?P<versionplus>.*)\)\)\.")
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");
        static ref YATV: Regex = RegexBuilder::new(
            r".*/(?P<year>\d{4})/(?P<author>.*) - (?P<title>.*) \((?P<version>.*)\)\.")
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");
        static ref ATVY: Regex = RegexBuilder::new(
            r".*/(?P<author>.*) - (?P<title>.*) \((?P<version>.*)\)\.(?P<year>\d{4})\.")
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");
        static ref YAT: Regex = RegexBuilder::new(
            r".*/(?P<year>\d{4})/(?P<author>.*) - (?P<title>.*)\.")
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");
        static ref ATY: Regex = RegexBuilder::new(
            r".*/(?P<author>.*) - (?P<title>.*)\.(?P<year>\d{4})")
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");
        static ref ATV: Regex = RegexBuilder::new(
            r".*/(?P<author>.*) - (?P<title>.*) \((?P<version>.*)\)\.")
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");
        static ref AT: Regex = RegexBuilder::new(
            r".*/(?P<author>.*) - (?P<title>.*)\.")
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");
        static ref T: Regex = RegexBuilder::new(
            r".*/(?P<title>.*)\.")
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");
    }

    let caps = match YATVV2.is_match(file_info) {
        true => YATVV2.captures(file_info).unwrap(),
        false => match YATV.is_match(file_info) {
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
        }
    };

    caps
}

pub fn format_info(track: &mut Track) {
    // Avoid compiling the same regex in a loop
    // https://docs.rs/regex/1.0.5/regex/#example-avoid-compiling-the-same-regex-in-a-loop
    lazy_static! {
        static ref AUTHOR_SHORT: Regex = RegexBuilder::new(
            r"(?P<author>.*) (?P<feat>(feat|ft|presents|pres|with|introduce)) (?P<author_plus>.*)")
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");
        static ref TITLE_SHORT: Regex = RegexBuilder::new(
            r"(?P<title>.*) (?P<title_plus>\(.*\))")
                .case_insensitive(true)
                .build()
                .expect("Invalid Regex");
    }
    if !track.author.is_empty() {
        let original_val = String::from(track.author.as_str());
        track.author = match AUTHOR_SHORT.is_match(&track.author) {
            true => {
                let caps = AUTHOR_SHORT.captures(&track.author).unwrap();
                track.author_plus = String::from(format!("{} {}", &caps["feat"], &caps["author_plus"]));

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

    if !track.version_plus.is_empty() {
        track.version += "_";
    }

    if track.release_year.is_empty() {
        track.release_year = String::from("3001");
    }
}

// Creates the short name, title + extension is mandatory,
// hence the other will get concanated with extras.
pub fn create_short_name(track: &mut Track) {
    let mut short_name: String = String::from("");
    if !track.author.is_empty() {
        short_name += &(format!("{} - ", track.author.to_owned()));
    }

    if !track.title.is_empty() {
        short_name += &(track.title.to_owned());
    }

    if !track.version.is_empty() {
        short_name += &(format!(" ({})", &track.version));
    }

    short_name += &(format!(".{}", &track.extension));

    if short_name.len() >= TOO_LONG {
        return shortened(track);
    }
    track.short_name = String::from(short_name);
}

pub fn get_or_empty(file_info: &Captures, index: &str) -> String {
    match &file_info.name(index) {
        None => String::new(),
        _ => String::from(file_info[index].to_owned())
    }
}

fn shortened(track: &mut Track) {
    let mut short_name: String = String::from("");
    if !track.author.is_empty() {
        let split_author = track.author.split("&").collect::<Vec<&str>>();

        if split_author.len() > 2 {
            short_name += &(format!("{} & {} &_ - ", split_author[0], split_author[1]));
        } else {
            short_name += &(format!("{} - ", track.author.to_owned()));
        }
    }

    if !track.title.is_empty() {
        let split_title = track.title.split(" ").collect::<Vec<&str>>();

        if split_title.len() > 2 {
            short_name += &(format!("{} {}_", split_title[0], split_title[1]));
        } else {
            short_name += &(track.title.to_owned());
        }
    }

    if !track.version.is_empty() {
        let split_version = track.version.split(" ").collect::<Vec<&str>>();

        if split_version.len() > 2 {
            short_name += &(format!(" ({}_{})", split_version[0], split_version.last().unwrap()));
        } else {
            short_name += &(format!(" ({})", &track.version));
        }
    }

    short_name += &(format!(".{}", &track.extension));

    if short_name.len() >= TOO_LONG {
        println!("formatter: The filename '{}' is still to long, shortening must be improved!", short_name);
    }

    track.short_name = String::from(short_name);
}
