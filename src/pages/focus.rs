use std::sync::{Arc, Mutex};

use crate::app::GlobalState;
use crate::page::{Page, PageAction};
use crate::task::Task;
use ratatui::buffer::Buffer;
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

#[derive(Default)]
pub struct FocusPageState {
    global_state: Arc<Mutex<GlobalState>>,
    tasks: Vec<Task>,
    active_widget: FocusWidget,
    selected_task_idx: Option<usize>,
}

impl FocusPageState {
    pub fn new(global_state: Arc<Mutex<GlobalState>>) -> Self {
        Self {
            global_state,
            ..Default::default()
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct FocusPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum FocusWidget {
    #[default]
    CurrentTask,
    Timer,
    SessionLogs,
}

impl StatefulWidget for FocusPage {
    type State = FocusPageState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let main_area = chunks[0];

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(main_area);

        self.render_current_task(main_chunks[0], buf, state);

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[1]);

        self.render_not_implemented(
            buf,
            bottom_chunks[0],
            "Timer",
            state.active_widget == FocusWidget::Timer,
            1,
        );

        self.render_not_implemented(
            buf,
            bottom_chunks[1],
            "Session Logs",
            state.active_widget == FocusWidget::SessionLogs,
            2,
        );
    }
}

impl FocusPage {
    fn render_current_task(&self, area: Rect, buf: &mut Buffer, state: &mut FocusPageState) {
        let block = Block::default()
            .title("Current Task")
            .borders(Borders::ALL)
            .style(if state.active_widget == FocusWidget::CurrentTask {
                Style::default()
            } else {
                Style::default().add_modifier(Modifier::DIM)
            });

        let inner = block.inner(area);
        block.render(area, buf);

        if let Some(idx) = state.selected_task_idx {
            if let Some(task) = state.tasks.get(idx) {
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

                Paragraph::new(Text::from(text))
                    .centered()
                    .render(inner, buf);
            } else {
                Paragraph::new("No task selected")
                    .style(Style::default().fg(Color::DarkGray))
                    .centered()
                    .render(inner, buf);
            }
        } else {
            Paragraph::new("Select a task to focus on")
                .style(Style::default().fg(Color::DarkGray))
                .centered()
                .render(inner, buf);
        }
    }
}

impl Page for FocusPage {
    fn handle_key_event(self, key: KeyEvent, state: &mut Self::State) -> Option<PageAction> {
        match (key.modifiers, key.code) {
            (modifiers, KeyCode::Tab) => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    state.active_widget = match state.active_widget {
                        FocusWidget::CurrentTask => FocusWidget::SessionLogs,
                        FocusWidget::Timer => FocusWidget::CurrentTask,
                        FocusWidget::SessionLogs => FocusWidget::Timer,
                    };
                } else {
                    state.active_widget = match state.active_widget {
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

    fn on_activate(self, state: &mut Self::State) {
        state.active_widget = FocusWidget::CurrentTask;
    }

    fn on_deactivate(self, state: &mut Self::State) {
        state.active_widget = FocusWidget::CurrentTask;
    }

    fn select_widget(self, index: usize, state: &mut Self::State) {
        state.active_widget = match index {
            0 => FocusWidget::CurrentTask,
            1 => FocusWidget::Timer,
            2 => FocusWidget::SessionLogs,
            _ => state.active_widget,
        };
    }
}
