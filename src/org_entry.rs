use track::Track;

#[derive(Debug)]
pub struct OrgEntry {
    pub header: String,
    pub properties_header: String,
    pub author: String,
    pub author_plus: String,
    pub title: String,
    pub title_plus: String,
    pub version: String,
    pub version_plus: String,
    pub release_year: String,
    pub end: String
}

impl OrgEntry {
    pub fn new(track: &Track) -> OrgEntry {
        OrgEntry {
            header: format!("** {}\n", track.short_name),
            properties_header: String::from(":PROPERTIES:\n"),
            author: get_or_empty(String::from("Author"), "\t\t", &track.author),
            author_plus: get_or_empty(String::from("Author+"), "\t\t", &track.author_plus),
            title: get_or_empty(String::from("Title"), "\t\t", &track.title),
            title_plus: get_or_empty(String::from("Title+"), "\t\t", &track.title_plus),
            version: get_or_empty(String::from("Version"), "\t", &track.version),
            version_plus: get_or_empty(String::from("Version+"), "\t", &track.version_plus),
            release_year: format!(":year: \t\t\t{}\n", track.release_year),
            end: String::from(":END:\n")
        }
    }
}

fn get_or_empty(prefix: String, tabs: &str, string: &String) -> String {
    match string.is_empty(){
        true => String::new(),
        false => format!(":{}: {}{}\n", prefix, tabs, string)
    }
}
