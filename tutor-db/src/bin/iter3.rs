use actix_web::{web, App};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use state::AppState;
use std::{env, sync::Mutex};

use routes::*;

// ______________________________________________________________________
#[path = "../iter3/db_access.rs"]
mod db_access;
#[path = "../iter3/handlers.rs"]
mod handlers;
#[path = "../iter3/models.rs"]
mod models;
#[path = "../iter3/routes.rs"]
mod routes;
#[path = "../iter3/state.rs"]
mod state;

// ╾────────────────────────────────────────────────────────────────────╼
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You have already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        let shared_data = shared_data.clone();
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    // * INFO: Start HTTP Server
    println!("Starting HTTP server: 127.0.0.1:3002");

    actix_web::HttpServer::new(app)
        .bind("127.0.0.1:3002")?
        .run()
        .await
}
