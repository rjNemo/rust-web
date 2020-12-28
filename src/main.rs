// #![deny(missing_docs)]

use actix_web::{middleware, App, HttpServer};
use env_logger::Env;

mod task;

/// main starts the server and listen on address 127.0.0.1:8000
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(task::TaskList {
                tasks: vec![
                    task::Task {
                        id: 0,
                        title: "Learn Rust".to_string(),
                        is_completed: false,
                    },
                    task::Task {
                        id: 1,
                        title: "Learn Actix".to_string(),
                        is_completed: false,
                    },
                ],
            })
            .configure(task::init)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
