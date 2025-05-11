use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Padding, StatefulWidget,
    },
};

use crate::task::{Task, TaskId};

pub struct TaskListState {
    pub is_active: bool,
    pub picked_tasks: Vec<usize>,
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
    tasks: &'a [Task],
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
        let title = if state.is_active {
            Line::from(vec![
                Span::raw("Tasks ("),
                Span::styled(
                    format!("{}", self.tasks.len()),
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(")"),
                Span::styled(
                    " [a: add] [e: edit] [d: delete] [space: select]",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                ),
            ])
        } else {
            Line::from(vec![
                Span::raw("Tasks ("),
                Span::raw(format!("{}", self.tasks.len())),
                Span::raw(")"),
            ])
        };

        let mut block = Block::bordered()
            .title(title.centered())
            .border_type(if state.is_active {
                BorderType::Thick
            } else {
                BorderType::Plain
            })
            .padding(Padding::vertical(1));

        if !state.is_active {
            block = block.border_style(Style::default().add_modifier(Modifier::DIM));
        }

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

                let created_at = task.created_at.format("%Y-%m-%d %H:%M");

                let status_style = match task.status {
                    crate::task::TaskStatus::Backlog => Style::default().fg(Color::Gray),
                    crate::task::TaskStatus::Todo => Style::default().fg(Color::Blue),
                    crate::task::TaskStatus::InProgress => Style::default().fg(Color::Yellow),
                    crate::task::TaskStatus::Done => Style::default().fg(Color::Green),
                };

                let status_text = format!("{:?}", task.status);

                let line = Line::from(vec![
                    Span::raw(prefix),
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

        let list = List::new(items)
            .block(block)
            .highlight_symbol("→ ")
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow)
                    .bg(Color::DarkGray),
            );

        list.render(area, buf, &mut state.list_state);
    }
}
