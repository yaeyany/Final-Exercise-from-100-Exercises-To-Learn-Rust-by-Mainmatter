
pub struct Task {
    id: TaskId,
    title: TaskTitle,
    description: TaskDescription,
    priority: TaskPriority,
    status: TaskStatus,
}

pub struct TaskId(u32);
pub struct TaskTitle(String);
pub struct TaskDescription(String);
pub enum TaskPriority {
    Low,
    Medium,
    High,
}
pub enum TaskStatus {
    New,
    InProgress,
    Completed,
}

impl Task {
    pub fn new(id: TaskId, title: TaskTitle, description: Option<TaskDescription>) -> Self {
        let description = match description {
            Some(desc) => desc,
            None => TaskDescription("No description".to_string()),
        };

        Self {
            id,
            title,
            description,
            priority: TaskPriority::Medium,
            status: TaskStatus::New,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new_with_description() {
        let id = TaskId(1);
        let title = TaskTitle("Test".to_string());
        let description = Some(TaskDescription("Some desc".to_string()));
        let task = Task::new(id, title, description);
        assert_eq!(task.id.0, 1);
        assert_eq!(task.title.0, "Test");
        assert_eq!(task.description.0, "Some desc");
        assert_eq!(task.priority, TaskPriority::Medium);
        assert_eq!(task.status, TaskStatus::New);
    }

    #[test]
    fn test_task_new_without_description() {
        let id = TaskId(2);
        let title = TaskTitle("Test2".to_string());
        let description = None;
        let task = Task::new(id, title, description);
        assert_eq!(task.id.0, 2);
        assert_eq!(task.title.0, "Test2");
        assert_eq!(task.description.0, "No description");
        assert_eq!(task.priority, TaskPriority::Medium);
        assert_eq!(task.status, TaskStatus::New);
    }
}



