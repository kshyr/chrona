use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize, palette::tailwind::ZINC},
    symbols,
    text::Line,
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget,
    },
};

use crate::task::{Task, TaskId};

#[derive(Debug)]
pub struct TaskListState {
    pub is_active: bool,
    pub tasks: Vec<Task>,
    pub picked_tasks: Vec<TaskId>,
    pub focused_task: Option<TaskId>,
    // ratatui
    pub list_state: ListState,
}

impl Default for TaskListState {
    fn default() -> Self {
        let focused_task = Some(0);
        Self {
            is_active: false,
            tasks: vec![
                Task::new("Task 1", ""),
                Task::new("Task 2", ""),
                Task::new("Task 3", ""),
            ],
            picked_tasks: Vec::new(),
            focused_task,
            list_state: ListState::default().with_selected(focused_task),
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

    pub fn toggle_pick_task(&mut self, task_id: TaskId) {
        let is_picked = self.picked_tasks.contains(&task_id);
        if is_picked {
            self.picked_tasks.retain(|id| *id != task_id);
        } else {
            self.picked_tasks.push(task_id);
        }
    }

    pub fn focus_next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.focused_task = Some(i);
    }

    pub fn focus_previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.focused_task = Some(i);
    }
}

#[derive(Debug)]
pub struct TaskList;

impl Default for TaskList {
    fn default() -> Self {
        Self {}
    }
}

impl StatefulWidget for TaskList {
    type State = TaskListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut border_style = Style::default().bold();
        if !state.is_active {
            border_style = border_style.dim();
        }

        let block = Block::bordered()
            .title(Line::raw("Tasks").centered())
            .border_type(if state.is_active {
                BorderType::Thick
            } else {
                BorderType::Plain
            })
            .border_style(border_style);

        let items: Vec<ListItem> = state
            .tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let mut style = Style::default().not_bold();

                if let Some(focused_task) = state.focused_task {
                    if i == focused_task {
                        style = style.fg(Color::Yellow).bold();
                    }
                }

                let prefix = if state.picked_tasks.contains(&i) {
                    " [*] "
                } else {
                    " [ ] "
                };

                let styled_task = Task {
                    description: task.description.to_owned(),
                    status: task.status,
                    title: format!("{}{}", prefix, task.title),
                    ..*task
                };
                ListItem::from(styled_task).style(style).bold()
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        list.render(area, buf, &mut state.list_state);
    }
}

impl From<Task> for ListItem<'_> {
    fn from(task: Task) -> Self {
        ListItem::new(task.title.clone())
    }
}
