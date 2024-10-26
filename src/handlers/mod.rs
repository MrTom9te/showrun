use actix_web::web;
use auth::{login, logout};
use tasks::{create_task, get_tasks};

mod auth;
mod tasks;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(logout)
        .service(get_tasks)
        .service(create_task);
}
