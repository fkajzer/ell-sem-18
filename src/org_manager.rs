use std::fs::File;
use std::io::Write;
use std::io::{BufReader, BufRead};

use org_entry::OrgEntry;
use track::Track;
use fileaccessor;

pub fn create_entry(mut org_file: &File, org_entry: OrgEntry) {
    org_file.write_all(&org_entry.header.as_bytes()).unwrap();
    org_file.write_all(&org_entry.properties_header.as_bytes()).unwrap();

    if !org_entry.author.is_empty() {
        org_file.write_all(&org_entry.author.as_bytes()).unwrap();
    }
    if !org_entry.author_plus.is_empty() {
        org_file.write_all(&org_entry.author_plus.as_bytes()).unwrap();
    }
    if !org_entry.title.is_empty() {
        org_file.write_all(&org_entry.title.as_bytes()).unwrap();
    }
    if !org_entry.title_plus.is_empty() {
        org_file.write_all(&org_entry.title_plus.as_bytes()).unwrap();
    }
    if !org_entry.version.is_empty() {
        org_file.write_all(&org_entry.version.as_bytes()).unwrap();
    }
    if !org_entry.version_plus.is_empty() {
        org_file.write_all(&org_entry.version_plus.as_bytes()).unwrap();
    }

    org_file.write_all(&org_entry.release_year.as_bytes()).unwrap();
    org_file.write_all(&org_entry.end.as_bytes()).unwrap();

    println!("org_manager: new ORG DB entry: {}", &org_entry.header);
}

pub fn clean_up(track_list: &Vec<Track>) {
    let org_file = fileaccessor::append_org_file();
    let org_entries = read_tracks_from_org(&org_file);

    let mut flagged_for_report: Vec<String> = vec![];

    let flat_track_list: Vec<String> = track_list.into_iter()
        .map(|track| format!("{}.{}", track.file_name, track.extension))
        .collect();

    for entry in org_entries {
        let org_entry = format!("** {}", entry);

        if !flat_track_list.contains(&entry) { flagged_for_report.push(org_entry) }
    }

    match flagged_for_report.is_empty() {
        false => for entry in flagged_for_report {
                println!("org_manager: Mismatch in ORG DB with: {}", entry);
            }
        true => println!("org_manager: No mismatches detected when scanning existing ORG_ENTRIES!"),
    }
}

fn read_tracks_from_org(file: &File) -> Vec<String> {
    let mut org_list = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.starts_with("** ") {
            org_list.push(String::from(&line.as_str()[3..]));
        }
    }

    org_list
}
