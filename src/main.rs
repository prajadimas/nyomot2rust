use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
// use actix_web::middleware::{ Response, ErrorHandlers };
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    message: String,
}

#[get("/")]
/* async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
} */
async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(MyObj {
        message: "SUCCESS".to_string(),
    })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body + "\n")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080") ?
    .run()
    .await
}
