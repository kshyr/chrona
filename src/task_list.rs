use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Padding, StatefulWidget,
    },
};

use crate::{
    store::Store,
    task::{Task, TaskId},
    toml::TomlStore,
};

pub struct TaskListState {
    pub is_active: bool,
    pub picked_tasks: Vec<usize>,
    // ratatui
    pub list_state: ListState,
    pub last_selected: Option<usize>,
}

impl TaskListState {
    pub fn new(is_active: bool) -> Self {
        Self {
            picked_tasks: Vec::new(),
            list_state: ListState::default().with_selected(Some(0)),
            is_active,
            last_selected: None,
        }
    }

    pub fn toggle_pick_task(&mut self, task_id: TaskId) {
        let is_picked = self.picked_tasks.contains(&task_id);
        if is_picked {
            self.picked_tasks.retain(|id| *id != task_id);
        } else {
            self.picked_tasks.push(task_id);
        }
    }
}

pub struct TaskList<'a> {
    tasks: &'a [Task], // Borrows the tasks to display
}

impl<'a> TaskList<'a> {
    pub fn new(tasks: &'a [Task]) -> Self {
        Self { tasks }
    }
}

impl<'a> StatefulWidget for TaskList<'a> {
    type State = TaskListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
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

        let items: Vec<ListItem> = self
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
