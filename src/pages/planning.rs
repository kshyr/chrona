use crate::app::GlobalState;
use crate::page::{Page, PageAction, PageWidget};
use crate::widgets::task_list::{TaskList, TaskListState};
use ratatui::buffer::Buffer;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct PlanningPageState {
    global_state: Arc<Mutex<GlobalState>>,
    task_list_state: TaskListState,
    active_widget: PlanningWidget,
}

impl PlanningPageState {
    pub fn new(global_state: Arc<Mutex<GlobalState>>) -> Self {
        Self {
            global_state: global_state.clone(),
            task_list_state: TaskListState::new(global_state.clone()),
            ..Default::default()
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct PlanningPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum PlanningWidget {
    #[default]
    TaskList,
    TaskDetail,
}

impl PlanningPage {
    fn render_task_detail(&self, area: Rect, buf: &mut Buffer, state: &mut PlanningPageState) {
        let block = Block::default().title("Task Details").borders(Borders::ALL);
        if let Some(selected_idx) = state.task_list_state.list_state.selected() {
            if let Some(task) = state.global_state.lock().unwrap().tasks.get(selected_idx) {
                let status_style = match task.status {
                    crate::task::TaskStatus::Backlog => Style::default().fg(Color::Gray),
                    crate::task::TaskStatus::Todo => Style::default().fg(Color::Blue),
                    crate::task::TaskStatus::InProgress => Style::default().fg(Color::Yellow),
                    crate::task::TaskStatus::Done => Style::default().fg(Color::Green),
                };

                let created_at = task.created_at.format("%Y-%m-%d %H:%M").to_string();
                let updated_at = task.updated_at.format("%Y-%m-%d %H:%M").to_string();

                let mut text = Vec::new();
                text.push(Line::from(vec![
                    Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&task.title),
                ]));

                text.push(Line::from(vec![
                    Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(format!("{:?}", task.status), status_style),
                ]));

                text.push(Line::from(vec![
                    Span::styled("Created: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(created_at),
                ]));

                text.push(Line::from(vec![
                    Span::styled("Updated: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(updated_at),
                ]));

                text.push(Line::default());
                text.push(Line::from(Span::styled(
                    "Description:",
                    Style::default().add_modifier(Modifier::BOLD),
                )));

                if let Some(desc) = &task.description {
                    for line in desc.lines() {
                        text.push(Line::from(line));
                    }
                } else {
                    text.push(Line::from(Span::styled(
                        "(No description)",
                        Style::default().fg(Color::DarkGray),
                    )));
                }

                Paragraph::new(Text::from(text))
                    .block(block)
                    .render(area, buf);
            }
        } else {
            Paragraph::new("No task selected")
                .block(block)
                .style(Style::default().fg(Color::DarkGray))
                .centered()
                .render(area, buf);
        }
    }
}

impl StatefulWidget for PlanningPage {
    type State = PlanningPageState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let main_area = chunks[0];

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_area);

        state.task_list_state.is_active = state.active_widget == PlanningWidget::TaskList;
        TaskList::default().render(main_chunks[0], buf, &mut state.task_list_state);
        self.render_task_detail(main_chunks[1], buf, state);
    }
}

impl Page for PlanningPage {
    fn handle_key_event(self, key: KeyEvent, state: &mut Self::State) -> Option<PageAction> {
        TaskList::handle_key_event(key, &mut state.task_list_state);
        match (key.modifiers, key.code) {
            (_, KeyCode::Tab) => {
                state.active_widget = match state.active_widget {
                    PlanningWidget::TaskList => PlanningWidget::TaskDetail,
                    PlanningWidget::TaskDetail => PlanningWidget::TaskList,
                };
                Some(PageAction::None)
            }
            _ => None,
        }
    }

    fn on_activate(self, state: &mut Self::State) {
        state.task_list_state.is_active = true;
        state.active_widget = PlanningWidget::TaskList;
    }

    fn on_deactivate(self, state: &mut Self::State) {
        state.task_list_state.is_active = false;
    }

    fn select_widget(self, index: usize, state: &mut Self::State) {
        state.active_widget = match index {
            0 => PlanningWidget::TaskList,
            1 => PlanningWidget::TaskDetail,
            _ => state.active_widget,
        };
    }
}
