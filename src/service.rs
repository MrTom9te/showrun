use actix_web::{get, web::Path, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/api/users")]
pub async fn fetch_users() -> impl Responder {
    "GET /users".to_string()
}

#[get("/api/{id}/articles")]
pub async fn get_articles(path: Path<u32>) -> impl Responder {
    let id: u32 = path.into_inner();
    format!("GET /users/{id}/articles")
}
