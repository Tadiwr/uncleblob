use crate::providers::utils::serverutils::{get_server_address, get_server_port};


/** Representation of a blob */
pub struct Blob {
    pub file_name: String,
    pub bucket_name: String,

    /** The url to get access to the resource
     * in the format
     * 
     *  -> GET http://<server-address>/<bucket-name>/<file-name>;
     * 
     */

    /** The buffer read, to be written to or already written to  */
    pub buff: Vec<u8>
}


impl Blob {
    pub fn new(file_name: String, bucket_name:String, buff: Vec<u8>) -> Self {
        return Blob {
            file_name,
            bucket_name,
            buff
        }
    }

    pub fn get_blob_url(&self) -> String {

        return format!(
            "http://{}:{}/storage/{}/{}",
            get_server_address(),
            get_server_port(),
            &self.bucket_name,
            &self.file_name
        );
    }

    /* Serielizes blob object into json */
    
    pub fn to_json(&self) -> String {
        format!(
            "{{\n\"fileName\": \"{}\",\"bucketName\":\"{}\",\"url\": \"{}\"\n}}",
            self.file_name,
            self.bucket_name,
            self.get_blob_url()
        )
    }
}
