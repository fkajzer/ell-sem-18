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
}

pub fn clean_up(read_access: &File, org_entries: &Vec<String>, track_list: &Vec<Track>) {
    let mut flagged_for_delete: Vec<String> = vec![];

    let flat_track_list: Vec<String> = track_list.into_iter()
        .map(|track| format!("{}.{}", track.file_name, track.extension))
        .collect();

    for entry in org_entries {
        let org_entry = format!("** {}", entry);

        if !flat_track_list.contains(entry) { flagged_for_delete.push(org_entry) }
    }

    println!("{:#?}", flat_track_list);
    println!("{:#?}", flagged_for_delete);

    let mut file_string: String = String::new();

    let mut deleting = false;
    let mut deletion_counter: i32 = 0;

    for line in BufReader::new(&fileaccessor::append_org_file()).lines() {
        let line = line.unwrap();

        if flagged_for_delete.contains(&line) {
            deleting = true;
            continue;
        }

        if !deleting { file_string += &(line.to_owned() + "\n"); }

        if deleting && line == ":END:" {
            deletion_counter += 1;
            deleting = false;
        }
    }

    println!("{:#?}", file_string);
    // let mut overwrite_access = fileaccessor::overwrite_org_file();
    // overwrite_access.write_all(file_string.as_bytes()).expect("Something went wrong when cleaning up ORG_FILE");

    match deletion_counter == 0 {
        true => println!("org_manager: No changes detected when scanning existing ORG_ENTRIES!"),
        false => println!("org_manager: {} Entries have been removed!", deletion_counter),
    }
}
