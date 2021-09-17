use crate::routes::{courses_routes, general_routes};
use crate::state::AppState;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPool;
use std::sync::Mutex;
use std::{env, io};

#[path = "../iter3/db_access.rs"]
mod db_access;

#[path = "../iter3/handlers.rs"]
mod handlers;

#[path = "../iter3/routes.rs"]
mod routes;

#[path = "../iter3/models.rs"]
mod models;

#[path = "../iter3/state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL not found");
    let db = PgPool::new(&database_url).await.unwrap();
    let app_state = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db,
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(general_routes)
            .configure(courses_routes)
    });
    server.bind("127.0.0.1:3000")?.run().await
}
