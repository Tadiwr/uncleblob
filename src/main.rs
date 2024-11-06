use providers::storage::api::{get, put};

mod providers;

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


    match get("tadiwax.txt", "documents") {
        Ok(buff) => {
            println!("{}", String::from_utf8(buff).unwrap());
        }

        Err(err) => {
            println!("{}", err);
        }
    }
}

