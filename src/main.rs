// use std::fs::File;
// use std::path::PathBuf;
// use actix_files as fs;
// use actix_web::http::header::{ ContentDisposition, DispositionType };
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

/* #[get("/file/{filename:.yaml}")]
async fn file(req_file: HttpRequest) -> Result<fs::NamedFile, Error> {
    println!("{:?}", req_file);
    let path: PathBuf = req_file.match_info().query("filename").parse().unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
    // response
    HttpResponse::build(StatusCode::OK)
    .content_type("text/yaml; charset=utf-8")
    .body(include_str!("../views/index.html"))
} */

#[get("/docs")]
async fn docs() -> HttpResponse {
    // println!("{:?}", req);
    // response
    HttpResponse::build(StatusCode::OK)
    .content_type("text/html; charset=utf-8")
    .body(include_str!("../views/index.html"))
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
            .service(docs)
            // .service(file)
            .route("/hey", web::get().to(manual_hello))
            .default_service(web::route().to(not_found))
    })
    .bind("127.0.0.1:8080") ?
    .run()
    .await
}
