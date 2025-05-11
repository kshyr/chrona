use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Clear, Paragraph},
};
use tui_input::{Input, backend::crossterm::to_input_request};

mod help;
mod store;
mod task;
mod task_form;
mod task_list;
mod toml;

use help::{HelpScreen, HelpState};
use store::{Store, get_config_dir};
use task::Task;
use task_form::{ActiveFormField, FormMode, TaskForm, TaskFormState};
use task_list::{TaskList, TaskListState};
use toml::TomlStore;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Default, Copy, Clone, PartialEq)]
enum ActiveWidget {
    #[default]
    TaskList1,
    TaskList2,
}

pub struct App {
    running: bool,
    active_widget: ActiveWidget,
    task_list1: TaskListState,
    task_list2: TaskListState,
    task_form: TaskFormState,
    help_state: HelpState,
    tasks: Vec<Task>,
    store: TomlStore,
    status_message: Option<(String, Color)>,
}

impl App {
    pub fn new() -> Self {
        let mut store = TomlStore::new().unwrap_or_else(|e| {
            eprintln!(
                "Failed to initialize TomlStore: {}. Using an empty store.",
                e
            );
            TomlStore::new().expect("Fallback TomlStore creation failed")
        });
        let mut tasks = store.load_all_tasks().unwrap_or_else(|e| {
            eprintln!("Failed to load tasks: {}. Starting with an empty list.", e);
            Vec::new()
        });

        if let Ok(data_dir_result) = get_config_dir() {
            if tasks
                .iter()
                .find(|t| t.title.contains("CHRONA_CONFIG_DIR"))
                .is_none()
            {
                tasks.push(Task {
                    title: format!(
                        "Config Dir: {:?}",
                        data_dir_result
                            .into_os_string()
                            .into_string()
                            .unwrap_or_default()
                    ),
                    ..Default::default()
                });
            }
        }

        store.save_all_tasks(tasks.clone()).unwrap_or_else(|e| {
            eprintln!("Failed to save initial tasks: {}", e);
        });

        let task_list1 = TaskListState::new(true);
        let task_list2 = TaskListState::new(false);
        let task_form = TaskFormState::new();
        let help_state = HelpState::new();
        Self {
            running: true,
            active_widget: ActiveWidget::TaskList1,
            task_list1,
            task_list2,
            task_form,
            help_state,
            tasks,
            store,
            status_message: None,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn set_active_widget(&mut self, active_widget: ActiveWidget) {
        self.active_widget = active_widget;
        match active_widget {
            ActiveWidget::TaskList1 => {
                self.task_list1.is_active = true;
                self.task_list2.is_active = false;
            }
            ActiveWidget::TaskList2 => {
                self.task_list1.is_active = false;
                self.task_list2.is_active = true;
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let [main_area, status_bar] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(frame.area());

        let [left, right] =
            Layout::horizontal([Constraint::Percentage(49), Constraint::Percentage(51)])
                .areas(main_area);

        frame.render_widget(Clear, left);
        frame.render_stateful_widget(TaskList::new(&self.tasks), left, &mut self.task_list1);
        frame.render_widget(Clear, right);
        frame.render_stateful_widget(TaskList::new(&self.tasks), right, &mut self.task_list2);

        frame.render_stateful_widget(TaskForm, frame.area(), &mut self.task_form);
        frame.render_stateful_widget(HelpScreen, frame.area(), &mut self.help_state);

        if let Some((message, color)) = &self.status_message {
            frame.render_widget(
                Paragraph::new(message.clone()).style(Style::default().fg(*color)),
                status_bar,
            );
        } else {
            frame.render_widget(
                Paragraph::new("Press ? for help").style(Style::default().fg(Color::DarkGray)),
                status_bar,
            );
        }
    }

    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        let event = event::read()?;

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if self.task_form.is_modal_open {
                    self.on_input_event(key, &event);
                } else if self.help_state.visible {
                    if key.code == KeyCode::Esc || key.code == KeyCode::Char('?') {
                        self.help_state.hide();
                    }
                } else {
                    self.on_key_event(key);
                }

                self.status_message = None;
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_input_event(&mut self, key: KeyEvent, event: &Event) {
        match key.code {
            KeyCode::Esc => {
                self.task_form.is_modal_open = false;

                match self.active_widget {
                    ActiveWidget::TaskList1 => {
                        self.task_list1
                            .list_state
                            .select(self.task_list1.last_selected);
                    }
                    ActiveWidget::TaskList2 => {
                        self.task_list2
                            .list_state
                            .select(self.task_list2.last_selected);
                    }
                }
            }
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.task_form.prev_field();
                } else {
                    self.task_form.next_field();
                }
            }
            KeyCode::Enter => {
                self.submit_form();
            }
            KeyCode::Char(' ') if self.task_form.active_field == ActiveFormField::Status => {
                self.task_form.cycle_status();
            }
            _ => {
                if self.task_form.active_field != ActiveFormField::Status {
                    let active_form_input: &mut Input = match self.task_form.active_field {
                        ActiveFormField::Title => &mut self.task_form.title_input,
                        ActiveFormField::Description => &mut self.task_form.description_input,
                        _ => unreachable!(),
                    };

                    if let Some(req) = to_input_request(event) {
                        let _ = active_form_input.handle(req);
                    }
                }
            }
        }
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Char('j')) | (_, KeyCode::Down) => {
                match self.active_widget {
                    ActiveWidget::TaskList1 => self.task_list1.list_state.select_next(),
                    ActiveWidget::TaskList2 => self.task_list2.list_state.select_next(),
                };
            }
            (_, KeyCode::Char('k')) | (_, KeyCode::Up) => {
                match self.active_widget {
                    ActiveWidget::TaskList1 => self.task_list1.list_state.select_previous(),
                    ActiveWidget::TaskList2 => self.task_list2.list_state.select_previous(),
                };
            }
            (KeyModifiers::SHIFT, KeyCode::Tab) => {
                match self.active_widget {
                    ActiveWidget::TaskList1 => self.set_active_widget(ActiveWidget::TaskList2),
                    ActiveWidget::TaskList2 => self.set_active_widget(ActiveWidget::TaskList1),
                };
            }
            (_, KeyCode::Tab) => {
                match self.active_widget {
                    ActiveWidget::TaskList1 => self.set_active_widget(ActiveWidget::TaskList2),
                    ActiveWidget::TaskList2 => self.set_active_widget(ActiveWidget::TaskList1),
                };
            }
            (_, KeyCode::Char(' ')) => {
                match self.active_widget {
                    ActiveWidget::TaskList1 => {
                        if let Some(selected_idx) = self.task_list1.list_state.selected() {
                            self.task_list1.toggle_pick_task(selected_idx);
                        }
                    }
                    ActiveWidget::TaskList2 => {
                        if let Some(selected_idx) = self.task_list2.list_state.selected() {
                            self.task_list2.toggle_pick_task(selected_idx);
                        }
                    }
                };
            }
            (_, KeyCode::Char('a')) => {
                self.task_form.open_for_create();
                self.save_selection_state();
            }
            (_, KeyCode::Char('e')) => {
                if let Some(task) = self.get_selected_task() {
                    self.task_form.open_for_edit(&task);
                    self.save_selection_state();
                } else {
                    self.status_message =
                        Some(("No task selected to edit".to_string(), Color::Red));
                }
            }
            (_, KeyCode::Char('d')) => {
                self.delete_selected_task();
            }
            (_, KeyCode::Char('?')) => {
                self.help_state.toggle();
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn save_selection_state(&mut self) {
        match self.active_widget {
            ActiveWidget::TaskList1 => {
                if let Some(i) = self.task_list1.list_state.selected() {
                    self.task_list1.last_selected = Some(i);
                }
                self.task_list1.list_state.select(None);
            }
            ActiveWidget::TaskList2 => {
                if let Some(i) = self.task_list2.list_state.selected() {
                    self.task_list2.last_selected = Some(i);
                }
                self.task_list2.list_state.select(None);
            }
        }
    }

    fn get_selected_task(&self) -> Option<Task> {
        let selected_idx = match self.active_widget {
            ActiveWidget::TaskList1 => self.task_list1.list_state.selected()?,
            ActiveWidget::TaskList2 => self.task_list2.list_state.selected()?,
        };

        self.tasks.get(selected_idx).cloned()
    }

    fn delete_selected_task(&mut self) {
        let selected_idx = match self.active_widget {
            ActiveWidget::TaskList1 => self.task_list1.list_state.selected(),
            ActiveWidget::TaskList2 => self.task_list2.list_state.selected(),
        };

        if let Some(idx) = selected_idx {
            if idx < self.tasks.len() {
                self.tasks.remove(idx);

                match self.store.save_all_tasks(self.tasks.clone()) {
                    Ok(_) => {
                        self.status_message =
                            Some(("Task deleted successfully".to_string(), Color::Green));

                        if idx > 0 {
                            match self.active_widget {
                                ActiveWidget::TaskList1 => {
                                    self.task_list1.list_state.select(Some(idx - 1))
                                }
                                ActiveWidget::TaskList2 => {
                                    self.task_list2.list_state.select(Some(idx - 1))
                                }
                            }
                        } else if !self.tasks.is_empty() {
                            match self.active_widget {
                                ActiveWidget::TaskList1 => {
                                    self.task_list1.list_state.select(Some(0))
                                }
                                ActiveWidget::TaskList2 => {
                                    self.task_list2.list_state.select(Some(0))
                                }
                            }
                        } else {
                            match self.active_widget {
                                ActiveWidget::TaskList1 => self.task_list1.list_state.select(None),
                                ActiveWidget::TaskList2 => self.task_list2.list_state.select(None),
                            }
                        }
                    }
                    Err(e) => {
                        self.status_message =
                            Some((format!("Failed to delete task: {}", e), Color::Red));
                    }
                }
            }
        } else {
            self.status_message = Some(("No task selected to delete".to_string(), Color::Red));
        }
    }

    fn submit_form(&mut self) {
        match &self.task_form.mode {
            FormMode::Create => match TaskForm::create_task(&self.task_form) {
                Ok(task) => {
                    self.tasks.push(task);
                    self.store
                        .save_all_tasks(self.tasks.clone())
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to save tasks: {}", e);
                        });
                    self.task_form.is_modal_open = false;

                    match self.active_widget {
                        ActiveWidget::TaskList1 => {
                            self.task_list1
                                .list_state
                                .select(self.task_list1.last_selected);
                        }
                        ActiveWidget::TaskList2 => {
                            self.task_list2
                                .list_state
                                .select(self.task_list2.last_selected);
                        }
                    }

                    self.status_message =
                        Some(("Task created successfully".to_string(), Color::Green));
                }
                Err(err) => {
                    self.task_form.error_message = Some(err);
                }
            },
            FormMode::Edit(task_id) => match TaskForm::update_task(&self.task_form, *task_id) {
                Ok(updated_task) => {
                    if let Some(task) = self.tasks.iter_mut().find(|t| t.id == *task_id) {
                        *task = updated_task;
                    }

                    self.store
                        .save_all_tasks(self.tasks.clone())
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to save tasks: {}", e);
                        });
                    self.task_form.is_modal_open = false;

                    match self.active_widget {
                        ActiveWidget::TaskList1 => {
                            self.task_list1
                                .list_state
                                .select(self.task_list1.last_selected);
                        }
                        ActiveWidget::TaskList2 => {
                            self.task_list2
                                .list_state
                                .select(self.task_list2.last_selected);
                        }
                    }

                    self.status_message =
                        Some(("Task updated successfully".to_string(), Color::Green));
                }
                Err(err) => {
                    self.task_form.error_message = Some(err);
                }
            },
        }
    }
}
