use std::fs;
use std::fs::OpenOptions;
use ORG_LOCATION;
use NML_LOCATION;
use File;
use io::LineWriter;
use io::Write;

use DEBUG_MODE;
use DebugMode;

pub fn rename(file_name: &String, target_folder: String, new_name: &String) {
    create(target_folder.to_owned());

    fs::rename(file_name.to_owned(), target_folder + "/" + &new_name)
        .expect(&format!("Something went wrong when renaming {} -> {}",
            file_name, new_name
        ));
}

pub fn append_org_file() -> fs::File {
    let org_file_name = match DEBUG_MODE {
        DebugMode::NONE => ORG_LOCATION.to_owned(),
        _ =>  ORG_LOCATION.to_owned() + "_debug"
    };
    OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(org_file_name)
        .unwrap()
}

fn create(folder: String) {
    fs::create_dir_all(folder.to_owned())
        .expect(&format!("Error when creating FOLDER {}", folder));
}

pub fn write_nml_file(nml_as_bytes: Vec<u8>) {
    let file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .open(get_nml_filename())
        .unwrap();

    //set len to 0 to remove all bytes from the file (when new_file < old_file, last bytes won't be overwritten)
    file.set_len(0).expect("Something went wrong when cleaning up NML_LOCATION");

    let mut file = LineWriter::new(file);
    file.write_all(&nml_as_bytes).expect("Something went wrong when cleaning up NML_LOCATION");
    file.flush().expect("Something went wrong when cleaning up NML_LOCATION");
}

pub fn get_nml_filename() -> String {
    let file_name = match DEBUG_MODE {
        DebugMode::NONE => NML_LOCATION.to_owned(),
        _ => NML_LOCATION.to_owned() + "_debug"
    };

    file_name
}

/* pub fn overwrite_org_file(file_string: String) {
    create(ORG_DIR.to_owned());

    let file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .open(ORG_DIR.to_owned() + ORG_FILE)
        .unwrap();

    //set len to 0 to remove all bytes from the file (when new_file < old_file, last bytes won't be overwritten)
    file.set_len(0).expect("Something went wrong when cleaning up ORG_FILE");

    let mut file = LineWriter::new(file);
    file.write_all(file_string.as_bytes()).expect("Something went wrong when cleaning up ORG_FILE");
    file.flush().expect("Something went wrong when cleaning up ORG_FILE");
} */
