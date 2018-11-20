#[macro_use] extern crate lazy_static;
extern crate regex;

mod track;
mod formatter;
mod fileaccessor;
mod org_manager;
mod org_entry;

use std::{fs,
          io,
          io::{BufReader, BufRead},
          fs::File,
          path::{Path}};

use track::Track;
use org_entry::OrgEntry;

const SOURCE_FOLDER: &str = "./ell-music";
const TARGET_FOLDER: &str = "./ell-music";

//TODO remove helper constatn
const DEBUG_MODE: i32 = 0;

fn main() {
    let paths = fs::read_dir(&Path::new(SOURCE_FOLDER)).unwrap();

    let dir_names = paths.filter_map(|entry| {
        entry.ok().and_then(
            |e| e.path().file_name().and_then(
                |n| n.to_str().map(|s| String::from(SOURCE_FOLDER.to_owned() + "/" + s)))
      )}).collect::<Vec<String>>();

    let org_file = fileaccessor::append_org_file();
    let org_entries = read_tracks_from_org(&org_file);

    if DEBUG_MODE > 1 { println!("{:#?}", org_entries) };

    let mut org_add_counter: i32 = 0;
    let mut track_list: Vec<Track> = vec![];

    for dir in dir_names {
        track_list.append(&mut read_tracks_from_dir(dir).unwrap());
    }

    org_manager::clean_up(&org_file, &org_entries, &track_list);

    for mut track in track_list {
        formatter::format_info(&mut track);
        formatter::create_short_name(&mut track);

        if DEBUG_MODE > 2 { println!("{:#?}", track) };

        fileaccessor::rename(
            &track.original_name,
            format!("{}/{}", TARGET_FOLDER, track.release_year),
            &track.short_name);

        if !org_entries.contains(&track.short_name) {
            let org = OrgEntry::new(&track);

            if DEBUG_MODE > 2 { println!("{:#?}", org_entries) };

            org_add_counter += 1;
            org_manager::create_entry(&org_file, org);
        }
    }

    match org_add_counter == 0 {
        true => println!("org_manager: ORG_ENTRIES up to date!"),
        false => println!("org_manager: {} Entries have been added!", org_add_counter),
    }

    if SOURCE_FOLDER != TARGET_FOLDER {
        fs::remove_dir_all(SOURCE_FOLDER).expect("Deleting SOURCE FOLDER failed");
    }
}

// TODO add logic here iteravely
fn read_tracks_from_dir<P>(path: P) -> Result<Vec<Track>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| Track::new(&entry.path())))
        .collect()
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
