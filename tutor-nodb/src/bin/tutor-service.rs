use actix_web::{web, App, HttpServer};
use std::{io, sync::Mutex};

use routes::*;
use state::AppState;

// ______________________________________________________________________
#[path = "../handlers.rs"]
mod handlers;

#[path = "../models.rs"]
mod models;

#[path = "../routes.rs"]
mod routes;

#[path = "../state.rs"]
mod state;
// ______________________________________________________________________

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me 'I'm good!'".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3002")?.run().await
}
