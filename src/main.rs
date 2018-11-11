#[macro_use] extern crate lazy_static;
extern crate regex;

mod track;
mod formatter;
mod fileaccessor;

use std::{fs,
          io,
          path::{Path}};

use track::Track;

fn main() {
    const SOURCE_FOLDER: &str = "../tracks";
    const TARGET_FOLDER: &str = "./ell-music";
    let paths = fs::read_dir(&Path::new(SOURCE_FOLDER)).unwrap();

    let dir_names = paths.filter_map(|entry| {
        entry.ok().and_then(
            |e| e.path().file_name().and_then(
                |n| n.to_str().map(|s| String::from(SOURCE_FOLDER.to_owned() + "/" + s)))
      )}).collect::<Vec<String>>();


    for dir in dir_names {
        let tracks = read_tracks_from_dir(dir);

        for mut track in tracks.unwrap() {
            formatter::format_info(&mut track);
            formatter::create_short_name(&mut track);
            println!("{:#?}", track);

            fileaccessor::rename(
                track.original_name,
                format!("{}/{}", TARGET_FOLDER, track.release_year),
                track.short_name);
        }
    }

    if SOURCE_FOLDER != TARGET_FOLDER {
        fs::remove_dir_all(SOURCE_FOLDER).expect("Deleting SOURCE FOLDER failed");
    }
}

// TODO add logic here iteravely
pub fn read_tracks_from_dir<P>(path: P) -> Result<Vec<Track>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| Track::new(&entry.path())))
        .collect()
}
