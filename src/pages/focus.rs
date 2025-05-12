use crate::page::{Page, PageAction};
use crate::task::Task;
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

pub struct FocusPage {
    active_widget: FocusWidget,
    selected_task_idx: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusWidget {
    CurrentTask,
    Timer,
    SessionLogs,
}

impl FocusPage {
    pub fn new() -> Self {
        Self {
            active_widget: FocusWidget::CurrentTask,
            selected_task_idx: None,
        }
    }

    fn render_current_task(&self, frame: &mut Frame, area: Rect, tasks: &[Task]) {
        let block = Block::default()
            .title("Current Task")
            .borders(Borders::ALL)
            .style(if self.active_widget == FocusWidget::CurrentTask {
                Style::default()
            } else {
                Style::default().add_modifier(Modifier::DIM)
            });

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if let Some(idx) = self.selected_task_idx {
            if let Some(task) = tasks.get(idx) {
                let status_style = match task.status {
                    crate::task::TaskStatus::Backlog => Style::default().fg(Color::Gray),
                    crate::task::TaskStatus::Todo => Style::default().fg(Color::Blue),
                    crate::task::TaskStatus::InProgress => Style::default().fg(Color::Yellow),
                    crate::task::TaskStatus::Done => Style::default().fg(Color::Green),
                };

                let mut text = Vec::new();
                text.push(Line::from(Span::styled(
                    &task.title,
                    Style::default().add_modifier(Modifier::BOLD),
                )));

                text.push(Line::from(vec![
                    Span::raw("Status: "),
                    Span::styled(format!("{:?}", task.status), status_style),
                ]));

                if let Some(desc) = &task.description {
                    text.push(Line::default());
                    text.push(Line::from(desc.clone()));
                }

                frame.render_widget(Paragraph::new(Text::from(text)).centered(), inner);
            } else {
                frame.render_widget(
                    Paragraph::new("No task selected")
                        .style(Style::default().fg(Color::DarkGray))
                        .centered(),
                    inner,
                );
            }
        } else {
            frame.render_widget(
                Paragraph::new("Select a task to focus on")
                    .style(Style::default().fg(Color::DarkGray))
                    .centered(),
                inner,
            );
        }
    }
}

impl Page for FocusPage {
    fn render(&mut self, frame: &mut Frame, area: Rect, tasks: &[Task]) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let main_area = chunks[0];

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(main_area);

        self.render_current_task(frame, main_chunks[0], tasks);

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[1]);

        self.render_not_implemented(
            frame,
            bottom_chunks[0],
            "Timer",
            self.active_widget == FocusWidget::Timer,
            1,
        );

        self.render_not_implemented(
            frame,
            bottom_chunks[1],
            "Session Logs",
            self.active_widget == FocusWidget::SessionLogs,
            2,
        );
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<PageAction> {
        match (key.modifiers, key.code) {
            (modifiers, KeyCode::Tab) => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    self.active_widget = match self.active_widget {
                        FocusWidget::CurrentTask => FocusWidget::SessionLogs,
                        FocusWidget::Timer => FocusWidget::CurrentTask,
                        FocusWidget::SessionLogs => FocusWidget::Timer,
                    };
                } else {
                    self.active_widget = match self.active_widget {
                        FocusWidget::CurrentTask => FocusWidget::Timer,
                        FocusWidget::Timer => FocusWidget::SessionLogs,
                        FocusWidget::SessionLogs => FocusWidget::CurrentTask,
                    };
                }
                Some(PageAction::None)
            }

            (_, KeyCode::Char('t')) => Some(PageAction::None),
            (_, KeyCode::Char('p')) => Some(PageAction::None),
            (_, KeyCode::Char('s')) => Some(PageAction::None),

            _ => None,
        }
    }

    fn on_activate(&mut self) {
        self.active_widget = FocusWidget::CurrentTask;
    }

    fn on_deactivate(&mut self) {}

    fn select_widget(&mut self, index: usize) {
        self.active_widget = match index {
            0 => FocusWidget::CurrentTask,
            1 => FocusWidget::Timer,
            2 => FocusWidget::SessionLogs,
            _ => self.active_widget,
        };
    }
}
