use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("Deserialized FormData: {:?}", form);
    HttpResponse::Ok().finish()
}