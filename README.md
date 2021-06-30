# nyomot2rust - Repositori Belajar Rust

Belajar nyomot2 kodingan rust.

## actix_web

Belajar membuat service(s) pakai [Actix Web](https://github.com/actix/actix-web).

Coba nyomot dari [Getting Started](https://actix.rs/docs/getting-started/):
- GET `/` (OK)
- POST `/echo` (OK)
- GET `/hey` (OK)

## actix-swagger

Belajar ngoding tanpa ngoding, dengan auto generate koding dari OpenAPI Spec.

Coba nyomot dari [Swagg](https://github.com/openapi/actix-swagger):
- Generate from OpenAPI Spec. (OK)
```
cargo-swagg ./openapi.yaml --out-file ./src/api.rs
```

- Generate from OpenAPI Spec. [Pet Store](https://github.com/OAI/OpenAPI-Specification/blob/main/examples/v3.0/petstore.yaml) (Err)
```
cargo-swagg ./petstore.yaml --out-file ./src/petstore.rs
```

- Percantik hasil luaran koding (OK)
```
rustfmt ./src/api.rs
```

## serde

Belajar ngeformat output service jadi JSON

Coba nyomot dari [Examples](https://github.com/actix/examples/blob/master/json/json/src/main.rs):
- Output JSON `{ message: 'SUCCESS'}` (OK)

## Not Found Error

Belajar membuat handling default route yang tidak ada

Coba nyomot dari [Error 404 unfound routes Actix-web](https://users.rust-lang.org/t/error-404-unfound-routes-actix-web/46484/3)
- `{ message: NOT FOUND }` (OK)

## HTML Serve

Belajar mengeluarkan file html

Coba nyomot dari [What's the easiest way to get the HTML output of an actix-web endpoint handler to be rendered properly?](https://stackoverflow.com/questions/53182250/whats-the-easiest-way-to-get-the-html-output-of-an-actix-web-endpoint-handler-t)
- GET `/docs` (OK)

Selanjutnya nyomot apa?
- Handling Error(s)
- Upload file
- Websocket
- Apapun yang asik
