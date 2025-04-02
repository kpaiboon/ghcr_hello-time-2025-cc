mod errors;
mod person;
mod routes;

use std::sync::RwLock;
use std::env;

use actix_web::{middleware::Logger, web, App, HttpServer};
use routes::{add_person, delete_person, health, persons, single_person, update_person, AppState, landing_page, not_found_handler};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Read GREATING_TEXT from the environment or use default
    let greeting_text = env::var("GREETING_TEXT").unwrap_or_else(|_| "Hi!".to_string());  

    let shared_state = web::Data::new(AppState {
        person_collection: RwLock::new(person::create_person_collection()),
        greeting_text, // Pass the `greeting_text` value here
    });

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(shared_state.clone())
            .service(landing_page) // Landing page
            .service(single_person)
            .service(persons)
            .service(add_person)
            .service(delete_person)
            .service(update_person)
            .service(health)
            .default_service(web::route().to(not_found_handler)) // Handle unknown paths
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
