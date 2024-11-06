use std::{fs::{self, File}, io::Write};

use super::utils::{get_store_path, initialise_store};


/** Takes a `buffer` as param and stores it in the in the
blobstore. 
 
 * Also takes `file_name` which is the name of the file.
 * `bucket_name` which is the name of the bucket
 
 * the file is saved in the dir format
    `.blobstore/<bucket_name>::<filename>.<ext>`
 */

pub fn put(file_name: &str, bucket_name: &str, buffer: Vec<u8>)  -> Result<(), String>{

    initialise_store();
    
    let store_path = get_store_path();
    let file_path = format!("{}/{}::{}", store_path, bucket_name, file_name);

    if fs::exists(&file_path).unwrap() {

        return Err(
            format!("File with file name '{}', already exists", &file_name)
        );

    } else {
        let mut file = File::create(&file_path).unwrap();

        file.write_all(&buffer).unwrap();

        return Ok(());
    }
}