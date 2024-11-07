use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use models::blob;
use providers::{storage, utils::serverutils::{get_server_address, get_server_port}};

mod providers;
mod models;

#[get("/")] 
async fn index() -> impl Responder {
    return format!("Hie Im Uncle Blob, your self hosted object store!");
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

    println!("Server running at http://{}:{}", addr, port);

    HttpServer::new(|| {
        App::new()
        .service(index)
        .service(retrieve_file)
    })
        .bind((addr, port)).unwrap()
        .run()
        .await

}

