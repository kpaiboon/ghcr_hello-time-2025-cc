use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use serde::Deserialize;
//use std::env;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    port: u16,
    greeting_text: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 6000,
            greeting_text: "hi all ".to_string(),
        }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    let now = Utc::now();
    let config = envy::from_env::<Config>().unwrap_or_default();
    let greeting = config.greeting_text;

    let html = format!(
        "<!DOCTYPE html>
        <html>
        <head>
            <title>Rusty Web Server</title>
        </head>
        <body>
            <h1>Welcome!</h1>
            <p>Current GMT Time: {}</p>
            <p>Greeting: {}</p>
        </body>
        </html>",
        now.format("%Y-%m-%d %H:%M:%S UTC"),
        greeting
    );
    HttpResponse::Ok().body(html)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = envy::from_env::<Config>().unwrap_or_default();
    let port = config.port;

    println!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
