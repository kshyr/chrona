use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Modifier, Style, Stylize as _},
    text::{Line, Span},
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget,
    },
};
use tracing::debug;

use crate::{
    app::App,
    page::PageWidget,
    task::{Task, TaskId, TaskStatus},
};

#[derive(Default)]
pub struct TaskListState {
    pub is_active: bool,
    pub picked_tasks: Vec<usize>,
    pub list_state: ListState,
}

impl TaskListState {
    pub fn new() -> Self {
        Self {
            is_active: false,
            picked_tasks: Vec::new(),
            list_state: ListState::default(),
        }
    }

    pub fn toggle_pick_task(&mut self, task_idx: usize) {
        if self.picked_tasks.contains(&task_idx) {
            self.picked_tasks.retain(|&id| id != task_idx);
        } else {
            self.picked_tasks.push(task_idx);
        }
    }

    pub fn select_next(&mut self, total_items: usize) {
        if total_items == 0 {
            self.list_state.select(None);
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= total_items - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_previous(&mut self, total_items: usize) {
        if total_items == 0 {
            self.list_state.select(None);
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    total_items - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn handle_key_event(
        key: KeyEvent,
        state: &mut TaskListState,
        tasks: &[Task],
    ) -> TaskListAction {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                state.select_next(tasks.len());
                TaskListAction::SelectNext
            }
            KeyCode::Char('k') | KeyCode::Up => {
                state.select_previous(tasks.len());
                TaskListAction::SelectPrevious
            }
            KeyCode::Char('a') => TaskListAction::AddTask,
            KeyCode::Char('e') => {
                if let Some(selected_idx) = state.list_state.selected() {
                    if let Some(task) = tasks.get(selected_idx) {
                        TaskListAction::EditSelectedTask(task.id)
                    } else {
                        debug!("Attempted to edit task at invalid index: {}", selected_idx);
                        TaskListAction::None
                    }
                } else {
                    TaskListAction::None
                }
            }
            KeyCode::Char('d') => {
                if let Some(selected_idx) = state.list_state.selected() {
                    if let Some(task) = tasks.get(selected_idx) {
                        TaskListAction::DeleteSelectedTask(task.id)
                    } else {
                        debug!(
                            "Attempted to delete task at invalid index: {}",
                            selected_idx
                        );
                        TaskListAction::None
                    }
                } else {
                    TaskListAction::None
                }
            }
            KeyCode::Char(' ') => {
                if let Some(selected_idx) = state.list_state.selected() {
                    state.toggle_pick_task(selected_idx);
                }
                TaskListAction::TogglePickSelected
            }
            _ => TaskListAction::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TaskListAction {
    SelectNext,
    SelectPrevious,
    AddTask,
    EditSelectedTask(TaskId),
    DeleteSelectedTask(TaskId),
    TogglePickSelected,
    None,
}

pub struct TaskList<'a> {
    tasks: &'a [Task],
    state: &'a mut TaskListState,
}

impl<'a> TaskList<'a> {
    pub fn new(tasks: &'a [Task], state: &'a mut TaskListState) -> Self {
        Self { tasks, state }
    }
}

impl<'a> Widget for TaskList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut border_style = Style::default().dim();
        if self.state.is_active {
            border_style = border_style.not_dim().yellow();
        }

        let block = Block::bordered()
            .title(Line::from(vec![
                Span::from("[1] "),
                Span::from("Today").bold(),
            ]))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 1, 1))
            .border_style(border_style);

        let items: Vec<ListItem> = self
            .tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let prefix = if self.state.picked_tasks.contains(&i) {
                    "[*] "
                } else {
                    "[ ] "
                };

                let created_at = task.created_at.format("%y-%m-%d %H:%M");

                let status_style = match task.status {
                    TaskStatus::Backlog => Style::default().fg(Color::DarkGray),
                    TaskStatus::Todo => Style::default().fg(Color::Cyan),
                    TaskStatus::InProgress => Style::default().fg(Color::Yellow),
                    TaskStatus::Done => Style::default().fg(Color::Green),
                };

                let status_text = format!("{:?}", task.status);

                let line = Line::from(vec![
                    Span::styled(
                        prefix,
                        if self.state.is_active && self.state.list_state.selected() == Some(i) {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default()
                        },
                    ),
                    Span::styled(
                        task.title.clone(),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(format!("[{}]", status_text), status_style),
                    Span::raw(" "),
                    Span::styled(
                        format!("({})", created_at),
                        Style::default().add_modifier(Modifier::DIM),
                    ),
                ]);

                ListItem::new(line)
            })
            .collect();

        let list_widget = List::new(items)
            .block(block)
            .highlight_symbol("→ ")
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow)
                    .bg(Color::DarkGray),
            );

        StatefulWidget::render(list_widget, area, buf, &mut self.state.list_state);
    }
}

impl<'a> PageWidget for TaskList<'a> {
    fn render_help(&self, area: Rect, buf: &mut Buffer) {
        let help_text = "[j/k: navigate] [a: add] [e: edit] [d: delete] [space: select]";
        Paragraph::new(help_text)
            .style(Style::default().fg(Color::DarkGray))
            .render(area, buf);
    }

    fn handle_key_event(&self, _key: KeyEvent, _app: &mut App) {}
}
