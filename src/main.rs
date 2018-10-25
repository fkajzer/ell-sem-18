mod track;

use std::{fs,
          io,
          path::{Path}};
use track::Track;

fn main() {
    let prefix = "../tracks";

    let paths = fs::read_dir(&Path::new(prefix)).unwrap();

    let dir_names = paths.filter_map(|entry| {
        entry.ok().and_then(
            |e| e.path().file_name().and_then(
                |n| n.to_str().map(|s| String::from(prefix.to_owned() + "/" + s)))
      )}).collect::<Vec<String>>();

    for dir in dir_names {
        let tracks = read_filenames_from_dir(dir);

        for track in tracks.unwrap() {
            println!("{:?}", track.file_name);
        }
    }
}

pub fn read_filenames_from_dir<P>(path: P) -> Result<Vec<Track>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| Track::new(&entry.path())))
        .collect()
}

