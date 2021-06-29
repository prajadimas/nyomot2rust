use actix_web::{ get, post, web, App, /* HttpRequest, */ HttpResponse, HttpServer, /* Responder, Error, */ http::StatusCode };
// use actix_web::middleware::{ Response, ErrorHandlers };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
struct ResObj {
    message: String,
}

async fn not_found() -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND).json(ResObj {
        message: "NOT FOUND".to_string(),
    })
}

#[get("/")]
/* async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
} */
async fn hello() -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json(ResObj {
        message: "SUCCESS".to_string(),
    })
}

#[post("/echo")]
/* async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body + "\n")
} */
async fn echo(req_body: String) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json(ResObj {
        message: req_body,
    })
}

/*
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
} */
async fn manual_hello() -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json(ResObj {
        message: "Hey there".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .default_service(web::route().to(not_found))
    })
    .bind("127.0.0.1:8080") ?
    .run()
    .await
}
