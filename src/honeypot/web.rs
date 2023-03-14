use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::Message;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn web_main(address: &str, port: u16) -> std::io::Result<()> {
    let info_str = format!("Running honeypot web at {}:{}..", address, port);
    info_str.info_message();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((address, port))?
    .run()
    .await
}

pub fn run(address: &str, port: u16) {
    match web_main(address, port) {
        Ok(_) => (),
        Err(e) => {
            let e_str = format!("Running honeypot web error: {}", e);
            e_str.error_message();
        }
    }
}