use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::StatefulWidget};

use crate::task::{Task, TaskId};

#[derive(Debug)]
pub struct TaskListState {
    pub tasks: Vec<Task>,
    pub selected_tasks: Vec<TaskId>,
    pub focused_task: Option<TaskId>,
}

impl Default for TaskListState {
    fn default() -> Self {
        Self {
            tasks: vec![
                Task::new("Task 1", "Description 1"),
                Task::new("Task 2", "Description 2"),
                Task::new("Task 3", "Description 3"),
            ],
            selected_tasks: Vec::new(),
            focused_task: None,
        }
    }
}

impl TaskListState {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, task_id: TaskId) {
        self.tasks.retain(|task| task.id != task_id);
    }

    pub fn select_task(&mut self, task_id: TaskId) {
        self.selected_tasks.push(task_id);
    }

    pub fn deselect_task(&mut self, task_id: TaskId) {
        self.selected_tasks.retain(|id| *id != task_id);
    }
}

#[derive(Debug, Default)]
pub struct TaskList;

impl StatefulWidget for TaskList {
    type State = TaskListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Render tasks
        for (i, task) in state.tasks.iter().enumerate() {
            let y = area.top() + i as u16;
            let x = area.left();
            let width = area.width;
            let height = area.height;

            // Render task
            buf.set_string(x, y, task.title.clone(), Style::default());
        }
    }
}
