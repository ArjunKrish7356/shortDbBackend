use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tonic::transport::Channel;
use std::sync::Arc;
use proto::basic_client::BasicClient;
use serde::Deserialize;
pub mod methods;

pub mod proto {
    tonic::include_proto!("commands"); // The string here must match the package name in your .proto file
}


#[derive(Deserialize)]
struct Get {
    key: String,
}

#[derive(Deserialize)]
struct Set {
    key: String,
    value: String,
}


#[get("/get")]
async fn getDataFromDb(
    web::Query(data): web::Query<Get>, 
) -> impl Responder {
    println!("Received GET request for key: {}", data.key);
    let response = methods::get::get(data.key.to_string()).await;

    match response {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[post("/set")]
async fn setDataToDb(
    data: web::Json<Set>, 
) -> impl Responder {
    let key = &data.key;
    let value = &data.value;

    let response = methods::set::set(value.to_string(), key.to_string()).await;

    match response {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "http://[::1]:50051";
    let client = BasicClient::connect(url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .service(setDataToDb)
            .service(getDataFromDb)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
