use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Clear, Paragraph},
};
use std::{
    io::Stdout,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tui_input::{Input, backend::crossterm::to_input_request};

use crate::{
    help::{HelpScreen, HelpState},
    page::{PageAction, PageManager},
    store::Store,
    task::Task,
    toml::TomlStore,
    widgets::{
        navbar::Navbar,
        task_form::{TaskForm, TaskFormState},
    },
};

#[derive(Default)]
pub struct GlobalState {
    pub running: bool,
    pub task_form: TaskFormState,
    pub help_state: HelpState,
    pub tasks: Vec<Task>,
    pub store: TomlStore,
    pub status_message: Option<(String, Color)>,
}

pub struct App {
    global_state: Arc<Mutex<GlobalState>>,
    page_manager: PageManager,
}

impl App {
    pub fn new() -> Self {
        let mut store = TomlStore::default();
        let tasks = store.load_all_tasks().unwrap_or_else(|e| {
            eprintln!("Failed to load tasks: {}. Starting with an empty list.", e);
            Vec::new()
        });

        let global_state = Arc::new(Mutex::new(GlobalState {
            running: true,
            tasks,
            store,
            status_message: None,
            ..Default::default()
        }));
        let page_manager = PageManager::new(global_state.clone());

        Self {
            global_state,
            page_manager,
        }
    }

    pub fn run(
        mut self,
        mut terminal: Terminal<CrosstermBackend<Stdout>>,
    ) -> color_eyre::Result<()> {
        self.global_state.lock().unwrap().running = true;
        while self.global_state.lock().unwrap().running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let [navbar, main_area] =
            Layout::vertical([Constraint::Min(3), Constraint::Percentage(100)]).areas(frame.area());

        frame.render_widget(Clear, main_area);
        self.page_manager.render(frame, main_area);

        let gs = &mut self.global_state.lock().unwrap();

        if gs.task_form.is_modal_open {
            frame.render_stateful_widget(TaskForm, frame.area(), &mut gs.task_form);
        }
        if gs.help_state.visible {
            frame.render_stateful_widget(HelpScreen, frame.area(), &mut gs.help_state);
        }

        if let Some((message, color)) = &gs.status_message {
            frame.render_widget(
                Paragraph::new(message.clone()).style(Style::default().fg(*color)),
                navbar,
            );
        } else {
            let active_page = self.page_manager.active_page;
            frame.render_widget(Navbar::new(active_page), navbar);
        }
    }

    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        let event = event::read()?;

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if self.global_state.lock().unwrap().task_form.is_modal_open {
                } else if self.global_state.lock().unwrap().help_state.visible {
                    if key.code == KeyCode::Esc || key.code == KeyCode::Char('?') {
                        self.global_state.lock().unwrap().help_state.hide();
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

                self.global_state.lock().unwrap().status_message = None;
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),

            (_, KeyCode::Char('a')) => {
                self.global_state
                    .lock()
                    .unwrap()
                    .task_form
                    .open_for_create();
            }
            (_, KeyCode::Char('e')) => {
                if let Some(task) = self.get_selected_task() {
                    self.global_state
                        .lock()
                        .unwrap()
                        .task_form
                        .open_for_edit(&task);
                } else {
                    self.global_state.lock().unwrap().status_message =
                        Some(("No task selected to edit".to_string(), Color::Red));
                }
            }
            (_, KeyCode::Char('d')) => {
                self.delete_selected_task();
            }

            (_, KeyCode::Char('?')) => {
                self.global_state.lock().unwrap().help_state.toggle();
            }

            _ => {}
        }
    }

    fn quit(&mut self) {
        self.global_state.lock().unwrap().running = false;
    }

    fn get_selected_task(&self) -> Option<Task> {
        self.global_state.lock().unwrap().tasks.get(0).cloned()
    }

    fn delete_selected_task(&mut self) {
        self.global_state.lock().unwrap().status_message = Some((
            "Delete task functionality needs to be reimplemented for pages".to_string(),
            Color::Yellow,
        ));
    }
}
