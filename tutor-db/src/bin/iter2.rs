use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routes::*;
use sqlx::postgres::PgPool;
use std::{env, io, sync::Mutex};

// ______________________________________________________________________
#[path = "../iter2/handlers.rs"]
mod handlers;
#[path = "../iter2/models.rs"]
mod models;
#[path = "../iter2/routes.rs"]
mod routes;
#[path = "../iter2/state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    // INFO: Construct app state
    let shared_data = web::Data::new(state::AppState {
        health_check_response: "I'm good. You have already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    // INFO: Construct App & configure routes
    let app = move || {
        let shared_data = shared_data.clone();
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    // INFO: Satrt HTTP Server
    HttpServer::new(app).bind("127.0.0.1:3002")?.run().await
}
