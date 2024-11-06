use std::fs;

use crate::providers::utils::gitignore::add_to_gitignore;

pub fn initialise_store() {

    let store_path = get_store_path();

    if !fs::exists(&store_path).unwrap() {
        fs::create_dir(&store_path).unwrap();
        add_to_gitignore(&store_path);
    }
}

pub fn get_store_path() -> String {
    return ".blobstore".to_string();
}