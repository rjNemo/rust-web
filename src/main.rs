use actix_web::{get, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    id: u8,
    title: String,
    is_completed: bool,
}

#[derive(Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<Task>,
}

#[get("/")]
async fn index(data: web::Data<TaskList>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(&data.tasks))
}

#[get("/{id}")]
async fn get_task(
    web::Path(id): web::Path<usize>,
    data: web::Data<TaskList>,
) -> Result<HttpResponse> {
    let task = &data.tasks[id];
    Ok(HttpResponse::Ok().json(task))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .data(TaskList {
                tasks: vec![
                    Task {
                        id: 0,
                        title: "Learn Rust".to_string(),
                        is_completed: false,
                    },
                    Task {
                        id: 1,
                        title: "Learn Actix".to_string(),
                        is_completed: false,
                    },
                ],
            })
            .service(index)
            .service(get_task)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
