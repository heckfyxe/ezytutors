use crate::routes::{courses_routes, general_routes};
use crate::state::AppState;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::sync::Mutex;
use std::{env, io};

#[path = "../iter2/routes.rs"]
mod routes;

#[path = "../iter2/models.rs"]
mod models;

#[path = "../iter2/state.rs"]
mod state;

#[path = "../iter2/handlers.rs"]
mod handlers;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL isn't set in .env file");
    let db_pool = PgPool::new(&database_url).await.unwrap();
    // Construct App State
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    // Construct app and configure routes
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(courses_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
