use actix_web::{delete, get, post, put, web::{self, PayloadConfig}, App, HttpResponse, HttpServer, Responder};
use providers::{storage, utils::serverutils::{get_server_address, get_server_port}};

mod providers;
mod models;

#[get("/")] 
async fn index() -> impl Responder {
    return format!("Hie Im Uncle Blob, your self hosted object store!");
}


/** Route to upload a file */
#[post("/storage/upload/{bucket_name}/{file_name}")] 
async fn upload_file(path: web::Path<(String, String)>, payload: web::Bytes) -> impl Responder {

    let (bucket_name, file_name) = path.into_inner();
    let buffer = payload.to_vec();

    let res = storage::api::put(
        file_name.as_str(),
        bucket_name.as_str(),
        buffer
    );

    match res {
        Ok(blob) => {
            return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(blob.to_json());
        }

        Err(err) => {

            // FIXME: Server is responding with wrong error code
            return HttpResponse::NotFound().body(err);
        }
    }

}

/** Route to update or overwrite an existing file */
#[put("/storage/upload/{bucket_name}/{file_name}")] 
async fn overwrite_file(path: web::Path<(String, String)>, payload: web::Bytes) -> impl Responder {

    let (bucket_name, file_name) = path.into_inner();
    let buffer = payload.to_vec();

    let res = storage::api::overwrite(
        file_name.as_str(),
        bucket_name.as_str(),
        buffer
    );

    match res {
        Ok(blob) => {
            return HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(blob.to_json());
        }

        Err(err) => {
            return HttpResponse::NotFound().body(err);
        }
    }

}

/** Route to get an existing file */
#[get("/storage/{bucket_name}/{file_name}")] 
async fn retrieve_file(path: web::Path<(String, String)>) -> impl Responder {

    let (bucket_name, file_name) = path.into_inner();
    let res = storage::api::get(file_name.as_str(), bucket_name.as_str());

    match res {
        Ok(blob) => {
            return HttpResponse::Ok()
            .body(blob.buff);
        }

        Err(err) => {
            return HttpResponse::Accepted().body(err);
        }
    }
}

#[delete("/storage/delete/{bucket_name}/{file_name}")] 
async fn delete_file(path: web::Path<(String, String)>) -> impl Responder {

    let (bucket_name, file_name) = path.into_inner();
    
    storage::api::delete(file_name.as_str(), bucket_name.as_str());

    return  HttpResponse::Ok();
}


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    let addr = get_server_address();
    let port = get_server_port();

    // Sets upload limit to 10 mebi bytes
    const UPLOAD_LIMIT: usize = 10 * 1024 * 1024;

    println!("\n-------------------------------------------------------------\n");
    println!("Server running at http://{}:{}", addr, port);
    println!("\n-------------------------------------------------------------\n");

    HttpServer::new(|| {
        App::new()
        .service(index)
        .service(retrieve_file)
        .service(upload_file)
        .service(delete_file)
        .service(overwrite_file)
        .app_data(
            // Upload limit here
            PayloadConfig::new(UPLOAD_LIMIT)
        )
    })
        .bind((addr, port)).unwrap()
        .run()
        .await

}

