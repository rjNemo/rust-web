use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub is_completed: bool,
}

impl Task {
    pub fn new(id: usize, title: String) -> Task {
        Task {
            id,
            title,
            is_completed: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}
