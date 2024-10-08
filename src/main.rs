use actix_cors::Cors;
use actix_web::http;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use proto::basic_client::BasicClient;
use serde::Deserialize;
use std::sync::Arc;
use tonic::transport::Channel;
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

#[get("/{key}")]
async fn getDataFromDb(path: web::Path<String>) -> impl Responder {
    let key = path.into_inner(); // Get the key from the path

    println!("Received GET request for key: {}", key);

    // Retrieve the corresponding URL for the key
    let response = methods::get::get(key).await;

    match response {
        Ok(url) => {
            println!("Redirecting to: {}", url);
            HttpResponse::Found() // 302 Found for redirection
                .header("Location", url) // Redirect to the URL
                .finish()
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[post("/set")]
async fn setDataToDb(data: web::Json<Set>) -> impl Responder {
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
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173") // Update with your frontend URL
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(setDataToDb)
            .service(getDataFromDb)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
