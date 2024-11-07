use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use providers::{storage, utils::serverutils::{get_server_address, get_server_port}};

mod providers;
mod models;

#[get("/")] 
async fn index() -> impl Responder {
    return format!("Hie Im Uncle Blob, your self hosted object store!");
}


#[post("/storage/upload/{bucket_name}/{file_name}")] 
async fn upload(path: web::Path<(String, String)>, payload: web::Bytes) -> impl Responder {

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
            .body(blob.to_json());
        }

        Err(err) => {
            return HttpResponse::NotFound().body(err);
        }
    }

}

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


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    let addr = get_server_address();
    let port = get_server_port();

    println!("\n-------------------------------------------------------------\n");
    println!("Server running at http://{}:{}", addr, port);
    println!("\n-------------------------------------------------------------\n");

    HttpServer::new(|| {
        App::new()
        .service(index)
        .service(retrieve_file)
        .service(upload)
    })
        .bind((addr, port)).unwrap()
        .run()
        .await

}

