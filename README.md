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


Selanjutnya nyomot apa?
- Not Found Error
- Handling Error(s)
- Upload file
- Websocket
- Apapun yang asik
