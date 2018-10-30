#[macro_use] extern crate lazy_static;
extern crate regex;

mod track;
mod formatter;

use std::{fs,
          io,
          path::{Path}};
use track::Track;

fn main() {
    const PREFIX: &str = "../tracks";

    let paths = fs::read_dir(&Path::new(PREFIX)).unwrap();

    let dir_names = paths.filter_map(|entry| {
        entry.ok().and_then(
            |e| e.path().file_name().and_then(
                |n| n.to_str().map(|s| String::from(PREFIX.to_owned() + "/" + s)))
      )}).collect::<Vec<String>>();


    // TODO this is for debugging and testing
    for dir in dir_names {
        let tracks = read_tracks_from_dir(dir);

        for mut track in tracks.unwrap() {
            // TODO find something thats not shitty for this
            formatter::format_info(&mut track);

            // :#? => pretty print, :? print inLine
            if !track.author_plus.is_empty() {
                println!("{:#?}", track);
            }
        }
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
