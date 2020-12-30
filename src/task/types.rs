use serde::{Deserialize, Serialize};
use std::sync::Mutex;

/// Task model
#[derive(Serialize, Deserialize)]
pub struct Task {
    /// unique task identifier
    pub id: usize,
    /// short task description
    pub title: String,
    /// task completion status
    pub is_completed: bool,
}

impl Task {
    /// Create a new task with given title and id.
    ///
    /// A new task is always not completed.
    ///
    /// # Examples
    /// ```
    /// use task::types::Task;
    /// let task = Task::new(42, "Learn Rust");
    /// ```
    pub fn new(id: usize, title: String) -> Task {
        Task {
            id,
            title,
            is_completed: false,
        }
    }
}

/// Task DTO used to create a new task.
#[derive(Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
}

/// The actual task container.
#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Mutex<Vec<Task>>,
}
