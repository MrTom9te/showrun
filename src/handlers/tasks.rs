// src/handlers/tasks.rs

use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/tasks")]
pub async fn get_tasks() -> impl Responder {
    HttpResponse::Ok().body("Lista de Tarefas")
}

#[post("/tasks")]
pub async fn create_task() -> impl Responder {
    HttpResponse::Ok().body("Tarefa Criada")
}
