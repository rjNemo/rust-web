use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse, Result};

use crate::task::types::*;

/// Registers application routes.
///
/// # Example
///
/// ```
/// let app = App::new().configure(task::init);
/// ```
pub fn init(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(get_task);
    config.service(add_task);
    config.service(close_task);
}

/// List all existing tasks.
#[get("/")]
async fn index(data: web::Data<TaskList>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(&data.tasks))
}

/// Fetches the `Task` specified by `id`
#[get("/{id}")]
async fn get_task(
    web::Path(id): web::Path<usize>,
    data: web::Data<TaskList>,
) -> Result<HttpResponse> {
    let task = &data.tasks.lock().unwrap()[id];
    Ok(HttpResponse::Ok().json(task))
}

/// Creates and stores a new `Task`
#[post("/")]
async fn add_task(form: web::Form<NewTask>, data: web::Data<TaskList>) -> Result<HttpResponse> {
    let id = data.tasks.lock().unwrap().len();
    let new_task = Task::new(id, form.title.clone());
    data.tasks.lock().unwrap().push(new_task);
    Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(id))
}

/// Closes the given task if open, otherwise return an bad request
#[post("/{id}")]
async fn close_task(
    web::Path(id): web::Path<usize>,
    data: web::Data<TaskList>,
) -> Result<HttpResponse> {
    let task = &data.tasks.lock().unwrap()[id];
    if task.is_completed {
        Ok(HttpResponse::new(StatusCode::BAD_REQUEST))
    } else {
        let new_task = Task {
            is_completed: true,
            id: task.id,
            title: task.title.clone(),
        };
        Ok(HttpResponse::Ok().json(new_task))
    }
}
