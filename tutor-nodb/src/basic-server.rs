use std::io;

use actix_web::{web, App, HttpResponse, HttpServer};

// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// Configure handler
pub async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok().json("Eazytutors Server is up and running!")
}

// Instantiate & run the HTTP Server
#[actix_web::main]
async fn main() -> io::Result<()> {
    // Construct App & configure routes
    let app = move || App::new().configure(general_routes);

    // Start HTTP Server
    HttpServer::new(app).bind("127.0.0.1:3002")?.run().await
}
