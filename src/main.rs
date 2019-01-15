#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate quick_xml;

mod track;
mod formatter;
mod fileaccessor;
mod org_manager;
mod org_entry;
mod nml_manager;

use std::{fs,
          io,
          io::{BufReader, BufRead},
          fs::File,
          path::{Path}};

use track::Track;
use org_entry::OrgEntry;

const TRACKS_FOLDER: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/tracks";
const TRACKS_TARGET_FOLDER: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/tracks";
const ORG_LOCATION: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/org/tracks.org";
const NML_LOCATION: &str = "/Users/fkajzer/Projects/seminar/ell-sem-18/nml/collection.nml";

#[allow(dead_code)]
enum DebugMode {
    NONE,
    TRACK,
    ORG,
    NML,
    ALL,
    WRITETODEBUGFILES
}

const DEBUG_MODE: DebugMode = DebugMode::WRITETODEBUGFILES;

fn main() {
    let paths = fs::read_dir(&Path::new(TRACKS_FOLDER)).unwrap();

    let dir_names = paths.filter_map(|entry| {
        entry.ok().and_then(
            |e| e.path().file_name().and_then(
                |n| n.to_str().map(|s| String::from(TRACKS_FOLDER.to_owned() + "/" + s)))
      )}).collect::<Vec<String>>();

    let mut track_list: Vec<Track> = vec![];
    for dir in dir_names {
        let mut dirs = read_tracks_from_dir(dir).unwrap();
        track_list.append(&mut dirs);
    }

    org_manager::clean_up(&track_list);

    let org_file = fileaccessor::append_org_file();
    let org_entries = read_tracks_from_org(&org_file);

    match DEBUG_MODE {
        DebugMode::ORG | DebugMode::ALL => println!("{:#?}", org_entries),
        _ => ()
    }

    let mut org_add_counter: i32 = 0;

    let mut new_tracks_in_org: Vec<Track> = vec![];
    for mut track in track_list {
        formatter::format_info(&mut track);
        formatter::create_short_name(&mut track);

        match DEBUG_MODE {
            DebugMode::TRACK | DebugMode::ALL => println!("{:#?}", track),
            _ => ()
        }

        fileaccessor::rename(
            &track.original_name,
            format!("{}/{}", TRACKS_TARGET_FOLDER, track.release_year),
            &track.short_name);

        if !org_entries.contains(&track.short_name) {
            org_add_counter += 1;
            new_tracks_in_org.push(track.clone());

            let org = OrgEntry::new(&track);

            match DEBUG_MODE {
                DebugMode::ORG | DebugMode::ALL => println!("{:#?}", org),
                _ => ()
            }

            org_manager::create_entry(&org_file, org);
        }
    }

    for track in &new_tracks_in_org {
        if &new_tracks_in_org.iter()
            .filter(|&t| *t.short_name == track.short_name
                && *t.file_name == track.file_name).count() > &1 {
            println!("DIR: Found duplicate for {:#?}", track);
        }
    }

    match org_add_counter == 0 {
        true => println!("org_manager: ORG_ENTRIES up to date!"),
        false => println!("org_manager: {} Entries have been added!", org_add_counter),
    }

    if TRACKS_FOLDER != TRACKS_TARGET_FOLDER {
        fs::remove_dir_all(TRACKS_FOLDER).expect("Deleting SOURCE FOLDER failed");
    }

    nml_manager::run(&new_tracks_in_org);
}

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
