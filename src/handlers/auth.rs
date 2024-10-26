use actix_web::{get, post, HttpResponse, Responder};

#[post("/api/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("endpoint login")
}

#[post("/api/logout")]
pub async fn logout() -> impl Responder {
    HttpResponse::Ok().body("logout")
}
