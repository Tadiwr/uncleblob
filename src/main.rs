use providers::storage::api::{get, put};


mod providers;
mod models;

fn main() {
    let str = "Hie\nMy name is Tadiwanashe Shangwa\nI'm 18";

    match put(
        "tadiwa.txt",
        "documents",
        str.as_bytes().to_vec()
    ) {
        Ok(_) => {
            println!("File saved successfuly");
        }

        Err(err) => {
            println!("{}", err);
        }
    }


    match get("tadiwa.txt", "documents") {
        Ok(blob) => {
            println!("URL for resource: {}", blob.get_blob_url());
        }

        Err(err) => {
            println!("{}", err);
        }
    }
}

