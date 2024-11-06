#![allow(non_upper_case_globals)]
mod linear_regression;

use actix_files as afs;
use actix_web::*;
use actix_web::{
    get, http::header::ContentType, http::StatusCode, post, App, HttpResponse, HttpResponseBuilder,
    HttpServer, Responder,
};
use linear_regression::{LinearRegressionModel, TrainingResult};
use std::{fs::read_to_string, num::ParseFloatError};

/* Linear Regression Interactive Server Code*/
// that weird parse code placed throughout is my DISGUSTING way of returning error
// you can't just construct an empty error in base rust
// so if the formatting is broken i just attempt to parse a non numeric string
// and it errors out
// this is nasty and not best practice :sob:
fn do_train(req_body: String) -> Result<TrainingResult, ParseFloatError> {
    let mut model = LinearRegressionModel::new_uninit();

    let req_body = req_body
        .replace("data=", "")
        .replace(' ', "")
        .replace('(', "")
        .replace(')', "")
        .trim()
        .to_owned();
    for line in req_body.lines() {
        let line: Vec<&str> = line.trim().split(',').collect();
        if line.len() != 2 {
            let _: f64 = "this is so hacky i hate it".parse()?;
            unreachable!()
        }

        let Some(line0) = line.get(0) else {
            let _: f64 = "this is so hacky i hate it".parse()?;
            unreachable!()
        };
        let Some(line1) = line.get(1) else {
            let _: f64 = "this is so hacky i hate it".parse()?;
            unreachable!()
        };
        model.add_points(&[(line0.parse()?, line1.parse()?)]);
    }

    model.set_epochs(((40_388_608 as f32) / (model.graph.len() as f32)) as usize);

    model.train();

    Ok(model.best_result)
}

#[post("/train")]
async fn train(req_body: String) -> impl Responder {
    let result = do_train(req_body);

    match result {
        Ok(value) => HttpResponseBuilder::new(StatusCode::OK).json(value),
        Err(_) => HttpResponseBuilder::new(StatusCode::OK)
            .content_type(ContentType::plaintext())
            .body("INVALIDINPUT"),
    }
}

/********************************************/

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

#[get("/linear")]
async fn linear() -> impl Responder {
    HttpResponse::Ok().body(read_to_string("linear.html").unwrap())
}

#[get("/bayes")]
async fn bayes() -> impl Responder {
    HttpResponse::Ok().body(read_to_string("bayes.html").unwrap())
}

#[get("/connect4")]
async fn connect4() -> impl Responder {
    HttpResponse::Ok().body(read_to_string("connect4.html").unwrap())
}

#[get("/aboutsite")]
async fn aboutsite() -> impl Responder {
    HttpResponse::Ok().body(read_to_string("aboutsite.html").unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(afs::Files::new("/static", "./static"))
            .service(aboutsite)
            .service(dumbjoke)
            .service(raytracer)
            .service(linear)
            .service(train)
            .service(bayes)
            .service(connect4)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
