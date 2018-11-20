use std::fs;
use std::fs::OpenOptions;

const ORG_DIR: &str = "./org";
const ORG_FILE: &str = "/tracks.org";

pub fn rename(file_name: &String, target_folder: String, new_name: &String) {
    create(target_folder.to_owned());

    fs::rename(file_name.to_owned(), target_folder + "/" + &new_name)
        .expect(&format!("Something went wrong when renaming {} -> {}",
            file_name, new_name
        ));
}

pub fn overwrite_org_file() -> fs::File {
    fs::create_dir_all(ORG_DIR)
        .expect(&format!("Error when creating FOLDER {}", ORG_DIR));

    OpenOptions::new()
    .create(true)
    .write(true)
    .open(ORG_DIR.to_owned() + ORG_FILE)
    .unwrap()
}

pub fn append_org_file() -> fs::File {
    fs::create_dir_all(ORG_DIR)
        .expect(&format!("Error when creating FOLDER {}", ORG_DIR));

    OpenOptions::new()
    .create(true)
    .read(true)
    .append(true)
    .open(ORG_DIR.to_owned() + ORG_FILE)
    .unwrap()
}

fn create(folder: String) {
    fs::create_dir_all(folder.to_owned())
        .expect(&format!("Error when creating FOLDER {}", folder));
}
