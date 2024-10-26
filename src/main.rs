#![allow(non_upper_case_globals)]
mod linear_regression;

use actix_files as afs;
use actix_web::*;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, HttpResponseBuilder, http::StatusCode, http::header::ContentType};
use linear_regression::{LinearRegressionModel, TrainingResult};
use std::{fs::read_to_string, num::ParseFloatError};


/* Linear Regression Interactive Server Code*/

fn do_train(req_body: String) -> Result<TrainingResult, ParseFloatError>{
    let mut model = LinearRegressionModel::new_uninit();

    let req_body = req_body
        .replace("data=", "")
        .replace(' ', "")
        .replace('(', "")
        .replace(')', "");
    
        for line in req_body.lines() {
            let line: Vec<&str> = line.trim().split(',').collect();
            model.add_points(&[(line[0].parse()?, line[1].parse()?)]);
        }

        model.set_epochs(((16_777_216 as f32) / (model.graph.len() as f32)) as usize);

        model.train();

        Ok(model.best_result)
}


#[post("/train")]
async fn train(req_body: String) -> impl Responder {
    let result = do_train(req_body);

    match result{
        Ok(value) => {
            HttpResponseBuilder::new(StatusCode::OK)
                .json(value)
        }
        Err(_) => {
            HttpResponseBuilder::new(StatusCode::OK)
                .content_type(ContentType::plaintext())
                .body("INVALIDINPUT")
        }
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
async fn linear() -> impl Responder{
    HttpResponse::Ok().body(read_to_string("linear.html").unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(afs::Files::new("/static", "./static"))
            .service(dumbjoke)
            .service(raytracer)
            .service(linear)
            .service(train)
            
            
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
