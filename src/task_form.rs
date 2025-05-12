use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget},
};
use tui_input::Input as TextInput;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ActiveFormField {
    #[default]
    Title,
    Description,
    Status,
}

#[derive(Clone, Debug)]
pub enum FormMode {
    Create,
    Edit(usize),
}

pub struct TaskFormState {
    pub is_modal_open: bool,
    pub title_input: TextInput,
    pub description_input: TextInput,
    pub status_input: TextInput,
    pub active_field: ActiveFormField,
    pub mode: FormMode,
    pub error_message: Option<String>,
    pub status_options: Vec<&'static str>,
    pub status_selection: usize,
}

impl TaskFormState {
    pub fn new() -> Self {
        let title_input = TextInput::default();
        let description_input = TextInput::default();
        let status_input = TextInput::default();
        let status_options = vec!["Backlog", "Todo", "InProgress", "Done"];

        Self {
            is_modal_open: false,
            title_input,
            description_input,
            status_input,
            active_field: ActiveFormField::Title,
            mode: FormMode::Create,
            error_message: None,
            status_options,
            status_selection: 0,
        }
    }

    pub fn reset(&mut self) {
        self.title_input.reset();
        self.description_input.reset();
        self.status_selection = 0;
        self.active_field = ActiveFormField::Title;
        self.error_message = None;
        self.mode = FormMode::Create;
    }

    pub fn open_for_create(&mut self) {
        self.reset();
        self.is_modal_open = true;
    }

    pub fn open_for_edit(&mut self, task: &crate::task::Task) {
        self.reset();
        self.title_input = TextInput::new(task.title.clone());

        if let Some(desc) = &task.description {
            self.description_input = TextInput::new(desc.clone());
        }

        self.status_selection = match task.status {
            crate::task::TaskStatus::Backlog => 0,
            crate::task::TaskStatus::Todo => 1,
            crate::task::TaskStatus::InProgress => 2,
            crate::task::TaskStatus::Done => 3,
        };

        self.mode = FormMode::Edit(task.id);
        self.is_modal_open = true;
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.title_input.value().trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        Ok(())
    }

    pub fn get_task_status(&self) -> crate::task::TaskStatus {
        match self.status_selection {
            0 => crate::task::TaskStatus::Backlog,
            1 => crate::task::TaskStatus::Todo,
            2 => crate::task::TaskStatus::InProgress,
            3 => crate::task::TaskStatus::Done,
            _ => crate::task::TaskStatus::Backlog,
        }
    }

    pub fn next_field(&mut self) {
        self.active_field = match self.active_field {
            ActiveFormField::Title => ActiveFormField::Description,
            ActiveFormField::Description => ActiveFormField::Status,
            ActiveFormField::Status => ActiveFormField::Title,
        }
    }

    pub fn prev_field(&mut self) {
        self.active_field = match self.active_field {
            ActiveFormField::Title => ActiveFormField::Status,
            ActiveFormField::Description => ActiveFormField::Title,
            ActiveFormField::Status => ActiveFormField::Description,
        }
    }

    pub fn cycle_status(&mut self) {
        self.status_selection = (self.status_selection + 1) % self.status_options.len();
    }
}

pub struct TaskForm;

impl StatefulWidget for TaskForm {
    type State = TaskFormState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if !state.is_modal_open {
            return;
        }

        self.render_background(area, buf);

        let modal_area = centered_rect(70, 50, area);
        Clear.render(modal_area, buf);

        let title = match state.mode {
            FormMode::Create => "Add New Task",
            FormMode::Edit(_) => "Edit Task",
        };

        let modal_block = Block::bordered()
            .border_type(BorderType::Thick)
            .title(title)
            .style(Style::default());

        let content_area = modal_block.inner(modal_area);
        modal_block.render(modal_area, buf);

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(content_area);

        let field_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(main_layout[0]);

        self.render_title_field(buf, field_layout[0], state);
        self.render_description_field(buf, field_layout[1], state);
        self.render_status_field(buf, field_layout[2], state);

        if let Some(error) = &state.error_message {
            Paragraph::new(error.as_str())
                .style(Style::default().fg(Color::Red))
                .render(field_layout[3], buf);
        }

        let footer_text = "Tab: Next Field | Shift+Tab: Prev Field | Enter: Save | Esc: Cancel | Space: Cycle Status";
        Paragraph::new(footer_text)
            .style(Style::default().fg(Color::DarkGray))
            .centered()
            .render(main_layout[1], buf);
    }
}

impl TaskForm {
    fn render_background(&self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .style(Style::default().add_modifier(Modifier::DIM))
            .render(area, buf);
    }

    fn render_title_field(&self, buf: &mut Buffer, area: Rect, state: &mut TaskFormState) {
        self.render_field(
            buf,
            area,
            "Title:",
            state.title_input.value(),
            state.active_field == ActiveFormField::Title,
            state.title_input.visual_cursor(),
        );
    }

    fn render_description_field(&self, buf: &mut Buffer, area: Rect, state: &mut TaskFormState) {
        self.render_field(
            buf,
            area,
            "Description:",
            state.description_input.value(),
            state.active_field == ActiveFormField::Description,
            state.description_input.visual_cursor(),
        );
    }

    fn render_status_field(&self, buf: &mut Buffer, area: Rect, state: &mut TaskFormState) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(15), Constraint::Min(0)])
            .split(area);

        Paragraph::new("Status:").render(chunks[0], buf);

        let status_style = if state.active_field == ActiveFormField::Status {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let status_text = state.status_options[state.status_selection];

        Paragraph::new(status_text)
            .block(Block::default().borders(Borders::BOTTOM))
            .style(status_style)
            .render(chunks[1], buf);
    }

    // fn centered_text(&self) -> Paragraph<'static> {
    //     Paragraph::new("").alignment(Alignment::Center)
    // }

    fn render_field(
        &self,
        buf: &mut Buffer,
        area: Rect,
        label: &str,
        value: &str,
        is_active: bool,
        cursor_position: usize,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(15), Constraint::Min(0)])
            .split(area);

        Paragraph::new(label).render(chunks[0], buf);

        let style = if is_active {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        Paragraph::new(value)
            .block(Block::default().borders(Borders::BOTTOM))
            .style(style)
            .render(chunks[1], buf);

        if is_active {
            let input_area = chunks[1];
            let cursor_offset = cursor_position as u16;
            let cursor_x = input_area
                .x
                .saturating_add(cursor_offset)
                .min(input_area.right().saturating_sub(1));
            let cursor_y = input_area.y;

            if cursor_x < input_area.right() {
                buf.set_style(
                    Rect::new(cursor_x, cursor_y, 1, 1),
                    Style::default().add_modifier(Modifier::REVERSED),
                );
            }
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

impl TaskForm {
    pub fn create_task(state: &TaskFormState) -> Result<crate::task::Task, String> {
        state.validate()?;

        let now = chrono::Utc::now();
        Ok(crate::task::Task {
            id: now.timestamp() as usize,
            title: state.title_input.value().trim().to_string(),
            description: if state.description_input.value().trim().is_empty() {
                None
            } else {
                Some(state.description_input.value().trim().to_string())
            },
            status: state.get_task_status(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn update_task(state: &TaskFormState, task_id: usize) -> Result<crate::task::Task, String> {
        state.validate()?;

        Ok(crate::task::Task {
            id: task_id,
            title: state.title_input.value().trim().to_string(),
            description: if state.description_input.value().trim().is_empty() {
                None
            } else {
                Some(state.description_input.value().trim().to_string())
            },
            status: state.get_task_status(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }
}
