use std::{fs::{exists, File}, io::{Read, Write}};

/** Adds path pattern to git ignore file safetly
 * 
 * If git file doesn't exist it creates it
 * If pattern is already in gitignore file it skips
 */

pub fn add_to_gitignore(pattern: &str) {

    let gitignore_path = ".gitignore";
        
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(gitignore_path).unwrap();

    if !pattern_exits(pattern, &gitignore_path) {
        file.write(format!("{}\n", pattern).as_bytes()).unwrap();
    }


}

fn pattern_exits(pattern: &str, file_path: &str) -> bool {

    if exists(file_path).unwrap() {
        let mut file = File::open(file_path).unwrap();
        let mut str_buff = String::new();

        file.read_to_string(&mut str_buff).unwrap();

        if str_buff.contains(pattern) {
            return true;
        }

    }

    return false;

}