use std::fs;

pub fn rename(file_name: String, target_folder: String, new_name: String) {
    fs::create_dir_all(target_folder.to_owned())
        .expect(&format!("Error when creating TARGET_FOLDER {}", target_folder));

    fs::rename(file_name.to_owned(), target_folder + "/" + &new_name)
        .expect(&format!("Something went wrong when renaming {} -> {}",
            file_name, new_name
        ));
}
