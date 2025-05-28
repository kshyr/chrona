use crate::{app::GlobalState, page::PageWidget, store::Store, ui::modal::centered_rect};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget},
};
use std::sync::{Arc, Mutex};
use tui_input::{Input as TextInput, backend::crossterm::EventHandler};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ActiveFormField {
    #[default]
    Title,
    Description,
    Status,
}

#[derive(Clone, Debug, Default)]
pub enum FormMode {
    #[default]
    Create,
    Edit(usize),
}

#[derive(Default)]
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
    pub fn reset(&mut self) {
        self.title_input.reset();
        self.description_input.reset();
        self.status_selection = 0;
        self.active_field = ActiveFormField::Title;
        self.status_options = vec!["Backlog", "Todo", "In Progress", "Done"];
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

    fn submit_form(&mut self, global_state: Arc<Mutex<GlobalState>>) {
        match &global_state.lock().unwrap().task_form.mode {
            FormMode::Create => {
                match TaskForm::create_task(&global_state.lock().unwrap().task_form) {
                    Ok(task) => {
                        global_state.lock().unwrap().tasks.push(task.clone());
                        global_state
                            .lock()
                            .unwrap()
                            .store
                            .append_task(task)
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to save tasks: {}", e);
                            });
                        global_state.lock().unwrap().task_form.is_modal_open = false;

                        global_state.lock().unwrap().status_message =
                            Some(("Task created successfully".to_string(), Color::Green));
                    }
                    Err(err) => {
                        global_state.lock().unwrap().task_form.error_message = Some(err);
                    }
                }
            }
            FormMode::Edit(task_id) => {
                match TaskForm::update_task(&global_state.lock().unwrap().task_form, *task_id) {
                    Ok(updated_task) => {
                        if let Some(task) = global_state
                            .lock()
                            .unwrap()
                            .tasks
                            .iter_mut()
                            .find(|t| t.id == *task_id)
                        {
                            *task = updated_task.clone();
                            global_state
                                .lock()
                                .unwrap()
                                .store
                                .update_task(*task_id, updated_task)
                                .unwrap_or_else(|e| {
                                    eprintln!("Failed to save tasks: {}", e);
                                });
                        }

                        global_state.lock().unwrap().task_form.is_modal_open = false;

                        global_state.lock().unwrap().status_message =
                            Some(("Task updated successfully".to_string(), Color::Green));
                    }
                    Err(err) => {
                        global_state.lock().unwrap().task_form.error_message = Some(err);
                    }
                }
            }
        }
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

impl PageWidget for TaskForm {
    fn handle_key_event(key: KeyEvent, state: &mut Self::State) {
        match key.code {
            KeyCode::Esc => {
                state.is_modal_open = false;
            }
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    state.prev_field();
                } else {
                    state.next_field();
                }
            }
            KeyCode::Char(' ') if state.active_field == ActiveFormField::Status => {
                state.cycle_status();
            }
            _ => {
                if state.active_field != ActiveFormField::Status {
                    let active_form_input: &mut TextInput = match state.active_field {
                        ActiveFormField::Title => &mut state.title_input,
                        ActiveFormField::Description => &mut state.description_input,
                        _ => unreachable!(),
                    };

                    active_form_input.handle_event(&Event::Key(key));
                }
            }
        }
    }

    fn render_help(&self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}
