use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    // return a 200 OK response
    HttpResponse::Ok().finish()
}
