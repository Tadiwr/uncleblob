use std::{fs::{self, File}, io::{Read, Write}};

use crate::models::blob::Blob;

use super::utils::{get_store_path, initialise_store};


/** Takes a `buffer` as param and stores it in the in the
blobstore. 
 
 * Also takes `file_name` which is the name of the file.
 * `bucket_name` which is the name of the bucket
 
 * the file is saved in the dir format
    `.blobstore/<bucket_name>::<filename>.<ext>`
 */

pub fn put(file_name: &str, bucket_name: &str, buffer: Vec<u8>)  -> Result<Blob, String>{

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

        return Ok(Blob::new(
            file_name.to_string(),
            bucket_name.to_string(),
            buffer
        ));
    }
}


/** Retrieves a file from the blobstore
 * 
 * takes `file_name` which is the name of the file.
 * `bucket_name` which is the name of the bucket
 
 * The function will attempt to get the file @ this path
    `.blobstore/<bucket_name>::<filename>.<ext>`
 */

 pub fn get(file_name: &str, bucket_name: &str)  -> Result<Blob, String>{

    initialise_store();
    
    let store_path = get_store_path();
    let file_path = format!("{}/{}::{}", store_path, bucket_name, file_name);

    if fs::exists(&file_path).unwrap() {

        let mut file = File::open(&file_path).unwrap();
        let mut buff: Vec<u8> = Vec::new();

        file.read_to_end(&mut buff).unwrap();

        return Ok(Blob::new(
            file_name.to_string(),
            bucket_name.to_string(),
            buff
        ));
       
    } else {
        

        return Err(
            format!("File with file name '{}', was not found", &file_name)
        );
    }
}



/** Deletes a file with the specified name and storage bucket */
pub fn delete(file_name: &str, bucket_name: &str) {

    initialise_store();
    
    let store_path = get_store_path();
    let file_path = format!("{}/{}::{}", store_path, bucket_name, file_name);

    if fs::exists(&file_path).unwrap() {
        fs::remove_file(file_path).unwrap();
    }
}


/** Overides the contents of an existing file */
pub fn overwrite(file_name: &str, bucket_name: &str, buffer: Vec<u8>)  -> Result<Blob, String>{

    initialise_store();
    
    let store_path = get_store_path();
    let file_path = format!("{}/{}::{}", &store_path, &bucket_name, &file_name);

    let file_result = File::options()
        .create(true)
        .write(true)
        .open(&file_path);

    match file_result {
        Ok(mut file) => {
            file.write_all(&buffer).unwrap();

            return Ok(Blob::new(file_name.to_string(), bucket_name.to_string(), buffer));
        }

        Err(_) => {
            return  Err(String::from("Error occured whiles overwriting file"));
        }
    }

}

