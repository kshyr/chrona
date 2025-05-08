use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::task::TaskStatus;

pub struct TaskFormState {
    pub is_modal_open: bool,
    pub task_title: Option<String>,
    pub task_description: Option<Option<String>>,
    pub task_status: Option<TaskStatus>,
}

impl TaskFormState {
    pub fn new() -> Self {
        Self {
            is_modal_open: false,
            task_title: None,
            task_description: None,
            task_status: None,
        }
    }
}

pub struct TaskForm;

impl StatefulWidget for TaskForm {
    type State = TaskFormState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        if !state.is_modal_open {
            return;
        }

        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        let modal_area = centered_rect(60, 25, area);
        popup_block.render(modal_area, buf);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
