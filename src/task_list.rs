use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style, Styled, Stylize},
    text::Line,
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Padding, StatefulWidget,
    },
};

use crate::{
    task::{Task, TaskId},
    toml::TomlStore,
};

#[derive(Debug)]
pub struct TaskListState {
    pub is_active: bool,
    pub tasks: Vec<Task>,
    pub picked_tasks: Vec<usize>,
    // ratatui
    pub list_state: ListState,
    pub store: TomlStore,
}

impl TaskListState {
    pub fn new(store: TomlStore, is_active: bool) -> Self {
        Self {
            tasks: store.tasks.clone(),
            picked_tasks: Vec::new(),
            list_state: ListState::default().with_selected(Some(0)),
            store,
            is_active,
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
    }

    pub fn focus_current(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => i,
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

#[derive(Debug, Default)]
pub struct TaskList;

impl StatefulWidget for TaskList {
    type State = TaskListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut border_style = Style::default();
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
            .border_style(border_style)
            .padding(Padding::proportional(1));

        let items: Vec<ListItem> = state
            .tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let prefix = if state.picked_tasks.contains(&i) {
                    " [*] "
                } else {
                    " [ ] "
                };

                let item_text = format!("{}{}", prefix, task.title);
                ListItem::new(item_text)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            );

        list.render(area, buf, &mut state.list_state);
    }
}
