use crate::page::{Page, PageAction};
use crate::task::Task;
use crate::task_list::{TaskList, TaskListState};
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

pub struct PlanningPage {
    task_list_state: TaskListState,
    active_widget: PlanningWidget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlanningWidget {
    TaskList,
    TaskDetail,
}

impl PlanningPage {
    pub fn new() -> Self {
        Self {
            task_list_state: TaskListState::new(true),
            active_widget: PlanningWidget::TaskList,
        }
    }

    fn render_task_detail(&self, frame: &mut Frame, area: Rect, tasks: &[Task]) {
        let block = Block::default().title("Task Details").borders(Borders::ALL);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if let Some(selected_idx) = self.task_list_state.list_state.selected() {
            if let Some(task) = tasks.get(selected_idx) {
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

                frame.render_widget(Paragraph::new(Text::from(text)), inner);
            }
        } else {
            frame.render_widget(
                Paragraph::new("No task selected")
                    .style(Style::default().fg(Color::DarkGray))
                    .centered(),
                inner,
            );
        }
    }
}

impl Page for PlanningPage {
    fn render(&mut self, frame: &mut Frame, area: Rect, tasks: &[Task]) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let main_area = chunks[0];

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_area);

        self.task_list_state.is_active = self.active_widget == PlanningWidget::TaskList;
        frame.render_stateful_widget(
            TaskList::new(tasks),
            main_chunks[0],
            &mut self.task_list_state,
        );

        self.render_task_detail(frame, main_chunks[1], tasks);
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<PageAction> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('j')) | (_, KeyCode::Down)
                if self.active_widget == PlanningWidget::TaskList =>
            {
                self.task_list_state.list_state.select_next();
                Some(PageAction::None)
            }
            (_, KeyCode::Char('k')) | (_, KeyCode::Up)
                if self.active_widget == PlanningWidget::TaskList =>
            {
                self.task_list_state.list_state.select_previous();
                Some(PageAction::None)
            }

            (_, KeyCode::Tab) => {
                self.active_widget = match self.active_widget {
                    PlanningWidget::TaskList => PlanningWidget::TaskDetail,
                    PlanningWidget::TaskDetail => PlanningWidget::TaskList,
                };
                Some(PageAction::None)
            }

            (_, KeyCode::Char('a')) => None,

            (_, KeyCode::Char('e')) => None,

            (_, KeyCode::Char('d')) => None,

            _ => None,
        }
    }

    fn on_activate(&mut self) {
        self.task_list_state.is_active = true;
        self.active_widget = PlanningWidget::TaskList;
    }

    fn on_deactivate(&mut self) {
        self.task_list_state.is_active = false;
    }

    fn select_widget(&mut self, index: usize) {
        self.active_widget = match index {
            0 => PlanningWidget::TaskList,
            1 => PlanningWidget::TaskDetail,
            _ => self.active_widget,
        };
    }
}
