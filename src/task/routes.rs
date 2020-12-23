use crate::task::types::*;
use actix_web::{get, post, web, HttpResponse, Result};

pub fn init(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(get_task);
    config.service(add_task);
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

#[post("/")]
async fn add_task(form: web::Form<NewTask>, data: web::Data<TaskList>) -> Result<HttpResponse> {
    let id = data.tasks.len();
    let new_task = Task::new(id, form.title.clone());
    // data.tasks.push(new_task);
    Ok(HttpResponse::Ok().json(new_task))
}
