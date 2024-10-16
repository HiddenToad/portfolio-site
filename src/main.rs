use actix_files as fs;
use actix_web::*;
use std::fs::read_to_string;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(read_to_string("index.html").unwrap())
}

#[get("/dumbjoke")]
async fn dumbjoke() -> impl Responder {
    HttpResponse::Ok().body(read_to_string("dumbjoke.html").unwrap())
}

#[get("/raytracer")]
async fn raytracer() -> impl Responder {
    HttpResponse::Ok().body(read_to_string("raytracer.html").unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(dumbjoke)
            .service(raytracer)
            .service(fs::Files::new("/static", "./static"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
