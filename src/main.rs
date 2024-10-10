use actix_web::{http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(req: HttpRequest) -> impl Responder {
    // return a 200 OK response
    HttpResponse::Ok()
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))  
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
