use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello 123 {name}!")
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello,world!")
}
