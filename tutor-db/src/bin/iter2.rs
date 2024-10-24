use std::io;

use dotenv::dotenv;

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

    todo!();
}
