use crate::{
    app::StatusMessage,
    page::{Page, PageAction, cycle_widget_index},
    task::{Task, TaskId},
    toml::TomlStore,
    widgets::task_form::TaskFormState,
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget as _ /* StatefulWidget, Widget */},
};

#[derive(Default)]
pub struct FocusPageState {
    pub active_widget: FocusWidget,
    pub selected_task_idx: Option<usize>,
}

impl FocusPageState {
    pub fn new() -> Self {
        Self {
            selected_task_idx: None,
            ..Default::default()
        }
    }
    pub fn set_selected_task_by_id(&mut self, tasks: &[Task], task_id_to_find: Option<TaskId>) {
        if let Some(id) = task_id_to_find {
            self.selected_task_idx = tasks.iter().position(|t| t.id == id);
        } else {
            self.selected_task_idx = None;
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct FocusPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum_macros::EnumIter)]
pub enum FocusWidget {
    #[default]
    CurrentTask,
    Timer,
    SessionLogs,
}

impl FocusPage {
    pub fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        page_state: &mut FocusPageState,
        tasks: &[Task],
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let main_area = chunks[0];

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(main_area);

        self.render_current_task(main_chunks[0], buf, page_state, tasks);

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[1]);

        Self::render_not_implemented(
            buf,
            bottom_chunks[0],
            "Timer",
            page_state.active_widget == FocusWidget::Timer,
            1,
        );

        Self::render_not_implemented(
            buf,
            bottom_chunks[1],
            "Session Logs",
            page_state.active_widget == FocusWidget::SessionLogs,
            2,
        );
    }
}

impl FocusPage {
    fn render_current_task(
        &self,
        area: Rect,
        buf: &mut Buffer,
        page_state: &mut FocusPageState,
        tasks: &[Task],
    ) {
        let block = Block::default()
            .title("Current Task")
            .borders(Borders::ALL)
            .style(if page_state.active_widget == FocusWidget::CurrentTask {
                Style::default()
            } else {
                Style::default().add_modifier(Modifier::DIM)
            });

        let inner = block.inner(area);
        block.render(area, buf);

        if let Some(idx) = page_state.selected_task_idx {
            if let Some(task) = tasks.get(idx) {
                let status_style = match task.status {
                    crate::task::TaskStatus::Backlog => Style::default().fg(Color::Gray),
                    crate::task::TaskStatus::Todo => Style::default().fg(Color::Blue),
                    crate::task::TaskStatus::InProgress => Style::default().fg(Color::Yellow),
                    crate::task::TaskStatus::Done => Style::default().fg(Color::Green),
                };

                let mut text_lines = Vec::new();
                text_lines.push(Line::from(Span::styled(
                    &task.title,
                    Style::default().add_modifier(Modifier::BOLD),
                )));

                text_lines.push(Line::from(vec![
                    Span::raw("Status: "),
                    Span::styled(format!("{:?}", task.status), status_style),
                ]));

                if let Some(desc) = &task.description {
                    text_lines.push(Line::default());
                    text_lines.push(Line::from(desc.clone()));
                }

                Paragraph::new(Text::from(text_lines))
                    .centered()
                    .render(inner, buf);
            } else {
                Paragraph::new("Selected task not found. Maybe it was deleted?")
                    .style(Style::default().fg(Color::DarkGray))
                    .centered()
                    .render(inner, buf);
                page_state.selected_task_idx = None;
            }
        } else {
            Paragraph::new(
                "Select a task to focus on (Press 't' from this page to pick first task)",
            )
            .style(Style::default().fg(Color::DarkGray))
            .centered()
            .render(inner, buf);
        }
    }
}

impl Page for FocusPage {
    type State = FocusPageState;

    fn handle_key_event(
        &self,
        key: KeyEvent,
        page_state: &mut Self::State,
        tasks: &mut Vec<Task>,
        _task_form: &mut TaskFormState,
        _store: &mut TomlStore,
        status_message: &mut StatusMessage,
    ) -> Option<PageAction> {
        match (key.modifiers, key.code) {
            (modifiers, KeyCode::Tab) => {
                page_state.active_widget = cycle_widget_index(
                    page_state.active_widget,
                    modifiers.contains(KeyModifiers::SHIFT),
                );
                Some(PageAction::None)
            }
            (_, KeyCode::Char('t')) => {
                if !tasks.is_empty() {
                    page_state.set_selected_task_by_id(tasks, Some(tasks[0].id.clone()));
                } else {
                    *status_message = StatusMessage::error("No tasks to focus on.".into());
                }
                Some(PageAction::None)
            }
            (_, KeyCode::Char('p')) => {
                /* TODO: Pause timer */
                Some(PageAction::None)
            }
            (_, KeyCode::Char('s')) => {
                /* TODO: Stop timer/session */
                Some(PageAction::None)
            }
            _ => None,
        }
    }

    fn on_activate(&self, state: &mut Self::State) {
        state.active_widget = FocusWidget::CurrentTask;
    }

    fn on_deactivate(&self, state: &mut Self::State) {
        state.active_widget = FocusWidget::CurrentTask;
        state.selected_task_idx = None;
    }

    fn select_widget(&self, index: usize, state: &mut Self::State) {
        state.active_widget = match index {
            0 => FocusWidget::CurrentTask,
            1 => FocusWidget::Timer,
            2 => FocusWidget::SessionLogs,
            _ => state.active_widget,
        };
    }
}
