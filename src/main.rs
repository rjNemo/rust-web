use std::env;
use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;

mod task;

/// main starts the server and listen on address 127.0.0.1:8000
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv::dotenv().ok();

    let addr = env::var("BASE_URL").unwrap();

    let tasks = web::Data::new(task::TaskList {
        tasks: Mutex::new(vec![
            task::Task {
                id: 0,
                title: "Learn Rust".to_string(),
                is_completed: true,
            },
            task::Task {
                id: 1,
                title: "Learn Actix".to_string(),
                is_completed: false,
            },
        ]),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(tasks.clone())
            .configure(task::init)
    })
    .bind(addr)?
    .run()
    .await
}
