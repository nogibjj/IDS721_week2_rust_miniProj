//Calculator Microservice
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::{StatusCode};

#[get("/")]
async fn index() -> impl Responder {
    // Ok::<HttpResponse, E>(HttpResponse::build(StatusCode::OK)
    // .content_type("text/html; charset=utf-8")
    // .body(include_str!("index.html")))
    HttpResponse::Ok().body("This is a calculator microservice.")
}
// #[get("/")]
// fn index(req: &HttpRequest) -> Result<HttpResponse, E> {
//     println!("{:?}", req);

//     // // set counter to session
//     // req.session().set("counter", counter)?;

//     // response
//     Ok(HttpResponse::build(StatusCode::OK)
//         .content_type("text/html; charset=utf-8")
//         .body(include_str!("index.html")))
// }
//library add route using lib.rs
#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::add(info.0, info.1);
    HttpResponse::Ok().body(result.to_string())
}

//library subtract route using lib.rs
#[get("/subtract/{a}/{b}")]
async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::subtract(info.0, info.1);
    HttpResponse::Ok().body(result.to_string())
}

//library multiply route using lib.rs
#[get("/multiply/{a}/{b}")]
async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::multiply(info.0, info.1);
    HttpResponse::Ok().body(result.to_string())
}

//library divide route using lib.rs
#[get("/divide/{a}/{b}")]
async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::divide(info.0, info.1);
    HttpResponse::Ok().body(result.to_string())
}

#[get("/power/{a}/{b}")]
async fn power(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::power(info.0, info.1);
    HttpResponse::Ok().body(result.to_string())
}

#[get("/sqrt/{a}")]
async fn squareroot(info: web::Path<f64>) -> impl Responder {
    let result = calc::squareroot(*info);
    HttpResponse::Ok().body(result.to_string())
}

//run it
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
            .service(power)
            .service(squareroot)
    })
    .bind(("127.0.0.1", 8083))?
    .run()
    .await
}
