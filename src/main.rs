// use std::fs::File;
// use std::path::PathBuf;
// use actix_files as fs;
// use actix_web::http::header::{ ContentDisposition, DispositionType };
use actix_web::{ get, post, web, App, /* HttpRequest, */ HttpResponse, HttpServer, /* Responder, Error, */ http::StatusCode };
// use actix_web::middleware::{ Response, ErrorHandlers };
use jsonwebtoken::{ encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use chrono::prelude::*;

const JWT_SECRET: &[u8] = b"some_secret";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // aud: String,         // Optional. Audience
    // exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    // iat: usize,          // Optional. Issued at (as UTC timestamp)
    // iss: String,         // Optional. Issuer
    // nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
    role: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct ResObj {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct UuidObj {
    message: String,
    data: String
}

async fn not_found() -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND).json(ResObj {
        message: "NOT FOUND".to_string(),
    })
}

pub fn create_jwt(uid: &str, role: &str) -> std::string::String {
    let expiration = UTC::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    match encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(value) => {
            return value
        },
        Err(error) => {
            println!("{}", error);
            return "error".to_string()
        },
    }
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

#[get("/uuid")]
/* async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
} */
async fn get_uuid() -> HttpResponse {
    let my_uuid = Uuid::new_v4();
    // println!("{}", my_uuid);
    HttpResponse::build(StatusCode::OK).json(UuidObj {
        message: "SUCCESS".to_string(),
        data:  my_uuid.to_string(),
    })
}

#[get("/token")]
/* async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
} */
async fn get_token() -> HttpResponse {
    let this_uuid = Uuid::new_v4();
    let encoded_token = create_jwt(&this_uuid.to_string(), "admin");
    let token: &str = 	"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJkNzk2ODk2My1lNjIwLTQwYzEtYjkxYy1kY2UyNDFjOTgxMTIiLCJyb2xlIjoiYWRtaW4iLCJleHAiOjE2MjUyNDk4MjZ9.jkob20pkzhvsT8btPbcMM3pK9VYvZcCAswOWWFuQxg2FqRfLPKi-FUY0IzOSxJYRwwzVUgp4CWbAtiat_o1CVQ";
    let decoded_token = decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &Validation::new(Algorithm::HS512));
    println!("{:?}", decoded_token);
    HttpResponse::build(StatusCode::OK).json(UuidObj {
        message: "SUCCESS".to_string(),
        data:  encoded_token.to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(docs)
            .service(get_uuid)
            .service(get_token)
            // .service(file)
            .route("/hey", web::get().to(manual_hello))
            .default_service(web::route().to(not_found))
    })
    .bind("127.0.0.1:8080") ?
    .run()
    .await
}
