use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::Paragraph,
};
use std::{env, fmt::Debug, io::Stdout};
use tracing::{debug, error, info};

use crate::{
    help::{HelpScreen, HelpState},
    page::PageManager,
    task::Task,
    ticktick::TickTickClient,
    toml::TomlStore,
    widgets::{
        navbar::Navbar,
        task_form::{TaskForm, TaskFormState},
    },
};

#[derive(Default, Debug, Clone)]
pub struct StatusMessage {
    message: String,
    color: Color,
    exp: DateTime<Utc>,
}

impl StatusMessage {
    pub fn new(message: String, color: Color) -> Self {
        Self {
            message,
            color,
            exp: Utc::now(),
        }
        .duration(Duration::seconds(5))
    }

    pub fn error(message: String) -> Self {
        Self::new(message, Color::Red)
    }

    pub fn success(message: String) -> Self {
        Self::new(message, Color::Green)
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.exp = Utc::now() + duration;
        self
    }
}

#[derive(Default)]
pub struct App {
    pub running: bool,
    pub page_manager: PageManager,
    pub task_form: TaskFormState,
    pub help_state: HelpState,
    pub tasks: Vec<Task>,
    pub store: TomlStore,
    pub status_message: StatusMessage,
}

impl App {
    pub fn new() -> Result<Self> {
        let username = env::var("TICKTICK_USERNAME")?;
        let password = env::var("TICKTICK_PASSWORD")?;
        let ticktick_client = TickTickClient::new(username, password)?;
        info!("TickTick client created successfully.");

        match ticktick_client.get_projects() {
            Some(projects) => debug!("Projects: {:?}", projects),
            None => error!("Error fetching projects"),
        };

        let store = TomlStore::default();
        // let tasks = store.load_all_tasks().unwrap_or_else(|e| {
        //     error!("Failed to load tasks: {}. Starting with an empty list.", e);
        //     Vec::new()
        // });

        let tasks = ticktick_client
            .get_tasks()
            .unwrap()
            .iter()
            .map(|t| {
                let task: Task = t.clone().into();
                task
            })
            .collect();

        Ok(Self {
            tasks,
            store,
            running: true,
            page_manager: PageManager::new(),
            ..Default::default()
        })
    }

    pub fn run(mut self, mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let [header_area, main_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Percentage(100)])
                .areas(frame.area());

        let [navbar_area, status_area] =
            Layout::horizontal([Constraint::Fill(3), Constraint::Fill(1)]).areas(header_area);

        let tasks_ref = &self.tasks;
        self.page_manager.render(frame, main_area, tasks_ref);

        if self.task_form.is_modal_open {
            frame.render_stateful_widget(TaskForm, frame.area(), self);
        }
        if self.help_state.visible {
            frame.render_stateful_widget(HelpScreen, frame.area(), &mut self.help_state);
        }

        if chrono::Utc::now() < self.status_message.exp {
            frame.render_widget(
                Paragraph::new(self.status_message.message.clone())
                    .style(Style::default().fg(self.status_message.color)),
                status_area,
            );
        }

        let active_page = self.page_manager.active_page;
        frame.render_widget(Navbar::new(active_page), navbar_area);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        let event = event::read()?;

        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if self.task_form.is_modal_open {
                    self.task_form.handle_key_event(
                        key,
                        event,
                        &mut self.tasks,
                        &mut self.store,
                        &mut self.status_message,
                        match self.page_manager.active_page {
                            crate::page::PageType::Home => {
                                &mut self.page_manager.home_page_state.task_list_state
                            }
                            crate::page::PageType::Planning => {
                                &mut self.page_manager.planning_page_state.task_list_state
                            }
                            _ => unreachable!(),
                        },
                    );
                } else if self.help_state.visible {
                    if key.code == KeyCode::Esc || key.code == KeyCode::Char('?') {
                        self.help_state.hide();
                    }
                } else {
                    self.on_key_event(key);

                    self.page_manager.handle_key_event(
                        key,
                        &mut self.tasks,
                        &mut self.task_form,
                        &mut self.store,
                        &mut self.status_message,
                    );
                }
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
            (_, KeyCode::Char('?')) => {
                self.help_state.toggle();
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
