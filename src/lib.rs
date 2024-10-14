use actix_web::{dev::Server, web::{self, Form}, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    // return a 200 OK response
    HttpResponse::Ok().finish()
}


#[derive(serde::Deserialize, Debug)]
struct FormData {
    email: String,
    name: String
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("Deserialized FormData: {:?}", form);
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
