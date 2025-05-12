use ratatui::{
    Frame,
    backend::CrosstermBackend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Clear, Paragraph},
};
use std::io;
use tui_input::{Input, backend::crossterm::to_input_request};

mod help;
mod page;
mod pages;
mod store;
mod task;
mod task_form;
mod task_list;
mod toml;

use help::{HelpScreen, HelpState};
use page::{PageAction, PageManager};
use pages::{DashboardPage, FocusPage, PlanningPage, StatsPage};
use store::Store;
use task::Task;
use task_form::{ActiveFormField, FormMode, TaskForm, TaskFormState};

use toml::TomlStore;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

pub struct App {
    running: bool,
    task_form: TaskFormState,
    help_state: HelpState,
    tasks: Vec<Task>,
    store: TomlStore,
    status_message: Option<(String, Color)>,
    page_manager: PageManager,
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
        let tasks = store.load_all_tasks().unwrap_or_else(|e| {
            eprintln!("Failed to load tasks: {}. Starting with an empty list.", e);
            Vec::new()
        });

        let task_form = TaskFormState::new();
        let help_state = HelpState::new();

        let page_manager = PageManager::new(
            Box::new(DashboardPage::new()),
            Box::new(PlanningPage::new()),
            Box::new(FocusPage::new()),
            Box::new(StatsPage::new()),
        );

        Self {
            running: true,
            task_form,
            help_state,
            tasks,
            store,
            status_message: None,
            page_manager,
        }
    }

    pub fn run(
        mut self,
        mut terminal: ratatui::Terminal<CrosstermBackend<io::Stdout>>,
    ) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let [main_area, status_bar] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(frame.area());

        frame.render_widget(Clear, main_area);
        self.page_manager.render(frame, main_area, &self.tasks);

        if self.task_form.is_modal_open {
            frame.render_stateful_widget(TaskForm, frame.area(), &mut self.task_form);
        }
        if self.help_state.visible {
            frame.render_stateful_widget(HelpScreen, frame.area(), &mut self.help_state);
        }

        if let Some((message, color)) = &self.status_message {
            frame.render_widget(
                Paragraph::new(message.clone()).style(Style::default().fg(*color)),
                status_bar,
            );
        } else {
            let active_page = self.page_manager.active_page();
            frame.render_widget(
                Paragraph::new(format!(
                    "{} | Shift + [1-4]: Switch Pages | ?: Help",
                    active_page
                ))
                .style(Style::default().fg(Color::DarkGray)),
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
                    if key.code == KeyCode::Char('q') {
                        self.quit();
                    }

                    let action = self.page_manager.handle_key_event(key);

                    if action.is_none() || matches!(action, Some(PageAction::None)) {
                        self.on_key_event(key);
                    }
                }

                self.status_message = None;
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_input_event(&mut self, key: KeyEvent, event: &ratatui::crossterm::event::Event) {
        match key.code {
            KeyCode::Esc => {
                self.task_form.is_modal_open = false;
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

            (_, KeyCode::Char('a')) => {
                self.task_form.open_for_create();
            }
            (_, KeyCode::Char('e')) => {
                if let Some(task) = self.get_selected_task() {
                    self.task_form.open_for_edit(&task);
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

    fn get_selected_task(&self) -> Option<Task> {
        self.tasks.get(0).cloned()
    }

    fn delete_selected_task(&mut self) {
        self.status_message = Some((
            "Delete task functionality needs to be reimplemented for pages".to_string(),
            Color::Yellow,
        ));
    }

    fn submit_form(&mut self) {
        match &self.task_form.mode {
            FormMode::Create => match TaskForm::create_task(&self.task_form) {
                Ok(task) => {
                    self.tasks.push(task.clone());
                    self.store.append_task(task).unwrap_or_else(|e| {
                        eprintln!("Failed to save tasks: {}", e);
                    });
                    self.task_form.is_modal_open = false;

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
                        *task = updated_task.clone();
                        self.store
                            .update_task(*task_id, updated_task)
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to save tasks: {}", e);
                            });
                    }

                    self.task_form.is_modal_open = false;

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
