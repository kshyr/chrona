use crate::{
    app::{App, StatusMessage},
    store::Store,
    task::{Task, TaskStatus},
    ui::modal::centered_rect,
    widgets::task_list::TaskListState,
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, StatefulWidget, Widget, Wrap},
};
use tracing::debug;
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

#[derive(Default, Clone)]
pub struct TaskFormState {
    pub is_modal_open: bool,
    pub title_input: TextInput,
    pub description_input: TextInput,
    pub active_field: ActiveFormField,
    pub mode: FormMode,
    pub error_message: Option<String>,
    pub status_selection: TaskStatus,
}

impl TaskFormState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn open_for_create(&mut self) {
        self.reset();
        self.is_modal_open = true;
    }

    pub fn open_for_edit(&mut self, task: &crate::task::Task) {
        debug!("entered open_for_edit for task :{:?}", task);
        self.reset();
        self.title_input = TextInput::new(task.title.clone());
        if let Some(desc) = &task.description {
            self.description_input = TextInput::new(desc.clone());
        }
        self.status_selection = task.status.clone();
        self.mode = FormMode::Edit(task.id);
        self.is_modal_open = true;
        debug!("TaskFormState prepared for editing task ID: {}", task.id);
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.title_input.value().trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        Ok(())
    }

    pub fn create_task(&self) -> Result<crate::task::Task, String> {
        self.validate()?;
        let now = chrono::Utc::now();
        Ok(crate::task::Task {
            id: now
                .timestamp_nanos_opt()
                .unwrap_or_else(|| now.timestamp_micros()) as usize,
            title: self.title_input.value().trim().to_string(),
            description: if self.description_input.value().trim().is_empty() {
                None
            } else {
                Some(self.description_input.value().trim().to_string())
            },
            status: self.status_selection.clone(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn update_task(
        &self,
        task_id: usize,
        original_created_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<crate::task::Task, String> {
        self.validate()?;
        Ok(crate::task::Task {
            id: task_id,
            title: self.title_input.value().trim().to_string(),
            description: if self.description_input.value().trim().is_empty() {
                None
            } else {
                Some(self.description_input.value().trim().to_string())
            },
            status: self.status_selection.clone(),
            created_at: original_created_at,
            updated_at: chrono::Utc::now(),
        })
    }

    pub fn handle_key_event(
        &mut self,
        key: KeyEvent,
        event: Event,
        tasks: &mut Vec<Task>,
        store: &mut dyn Store,
        status_message: &mut StatusMessage,
        task_list_state: &mut TaskListState,
    ) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                self.is_modal_open = false;
            }
            (_, KeyCode::Tab) => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.prev_field();
                } else {
                    self.next_field();
                }
            }
            (_, KeyCode::Char(' ')) if self.active_field == ActiveFormField::Status => {
                self.cycle_status();
            }
            (_, KeyCode::Enter) => self.submit_form(tasks, store, status_message, task_list_state),
            _ if self.active_field == ActiveFormField::Description => {
                self.description_input.handle_event(&event);
            }
            _ if self.active_field == ActiveFormField::Title => {
                self.title_input.handle_event(&event);
            }
            _ => {}
        }
    }

    fn submit_form(
        &mut self,
        tasks: &mut Vec<Task>,
        store: &mut dyn Store,
        status_message: &mut StatusMessage,
        task_list_state: &mut TaskListState,
    ) {
        match &self.mode {
            FormMode::Create => match self.create_task() {
                Ok(task) => {
                    tasks.push(task.clone());
                    if let Err(e) = store.append_task(task) {
                        tracing::error!("Failed to save new task: {}", e);
                        *status_message = StatusMessage::error(format!("Error saving task: {}", e));
                    } else {
                        *status_message =
                            StatusMessage::success("Task created successfully".into());
                        self.is_modal_open = false;
                    }
                }
                Err(err) => {
                    self.error_message = Some(err);
                }
            },
            FormMode::Edit(task_id) => {
                let task_id_val = *task_id;
                if let Some(pos) = tasks.iter().position(|t| t.id == task_id_val) {
                    let original_created_at = tasks[pos].created_at;
                    match self.update_task(task_id_val, original_created_at) {
                        Ok(updated_task) => {
                            debug!(task = ?updated_task, task_id = task_id_val, "submit in Edit mode");
                            tasks[pos] = updated_task.clone();
                            if let Err(e) = store.update_task(task_id_val, updated_task) {
                                tracing::error!("Failed to save updated task to store: {}", e);
                                *status_message =
                                    StatusMessage::error(format!("Error updating task: {}", e));
                            } else {
                                StatusMessage::success("Task updated successfully".into());
                                self.is_modal_open = false;
                                task_list_state.list_state.select(Some(pos));
                            }
                        }
                        Err(err) => {
                            debug!("Failed to update task: {}", err);
                            self.error_message = Some(err);
                        }
                    }
                } else {
                    self.error_message = Some(format!(
                        "Task with ID {} not found for update.",
                        task_id_val
                    ));
                    tracing::error!(
                        "Attempted to edit non-existent task with ID: {}",
                        task_id_val
                    );
                }
            }
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
        self.status_selection = match self.status_selection {
            TaskStatus::Backlog => TaskStatus::Todo,
            TaskStatus::Todo => TaskStatus::InProgress,
            TaskStatus::InProgress => TaskStatus::Done,
            TaskStatus::Done => TaskStatus::Backlog,
        }
    }
}

pub struct TaskForm;

impl TaskForm {
    fn render_background(&self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .style(Style::default().add_modifier(Modifier::DIM))
            .render(area, buf);
    }

    fn render_title_field(&self, buf: &mut Buffer, area: Rect, state: &TaskFormState) {
        let width = area.width.max(3) - 3;
        self.render_field(
            buf,
            area,
            "Title:",
            state.title_input.value(),
            state.active_field == ActiveFormField::Title,
            state.title_input.visual_cursor(),
            state.title_input.visual_scroll(width as usize),
        );
    }

    fn render_description_field(&self, buf: &mut Buffer, area: Rect, state: &TaskFormState) {
        let width = area.width.max(3) - 3;
        self.render_field(
            buf,
            area,
            "Description:",
            state.description_input.value(),
            state.active_field == ActiveFormField::Description,
            state.description_input.visual_cursor(),
            state.description_input.visual_scroll(width as usize),
        );
    }

    fn render_status_field(&self, buf: &mut Buffer, area: Rect, state: &TaskFormState) {
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

        let status_text = state.status_selection.as_ref();

        Paragraph::new(status_text)
            .block(Block::default().borders(Borders::BOTTOM))
            .style(status_style)
            .render(chunks[1], buf);
    }

    fn render_field(
        &self,
        buf: &mut Buffer,
        area: Rect,
        label: &str,
        value: &str,
        is_active: bool,
        cursor_position: usize,
        _scroll: usize,
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
            .wrap(Wrap { trim: false })
            .render(chunks[1], buf);

        if is_active {
            let input_area = chunks[1];
            let visible_width = input_area.width.saturating_sub(1) as usize;

            let scroll_offset = cursor_position / visible_width;

            let display_cursor_pos = cursor_position.saturating_sub(scroll_offset);

            let cursor_x = input_area
                .x
                .saturating_add(display_cursor_pos as u16)
                .min(input_area.right().saturating_sub(1));
            let cursor_y = input_area.y + scroll_offset as u16;

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
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app_state: &mut Self::State) {
        let form_data = &app_state.task_form;

        if !form_data.is_modal_open {
            return;
        }

        self.render_background(area, buf);

        let modal_area = centered_rect(70, 50, area);
        Clear.render(modal_area, buf);

        let title = match form_data.mode {
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
            .constraints([Constraint::Min(10), Constraint::Length(1)])
            .split(content_area);

        let field_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(3),
                Constraint::Length(2),
            ])
            .split(main_layout[0]);

        self.render_title_field(buf, field_layout[0], form_data);
        self.render_description_field(buf, field_layout[1], form_data);
        self.render_status_field(buf, field_layout[2], form_data);

        if let Some(error) = &form_data.error_message {
            Paragraph::new(error.as_str())
                .style(Style::default().fg(Color::Red))
                .render(field_layout[5], buf);
        }

        let footer_text =
            "Tab: Next | Shift+Tab: Prev | Enter: Save | Esc: Cancel | Space: Cycle Status";
        Paragraph::new(footer_text)
            .style(Style::default().fg(Color::DarkGray))
            .centered()
            .render(main_layout[1], buf);
    }
}
