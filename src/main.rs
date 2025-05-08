use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::Clear,
};

mod store;
mod task;
mod task_form;
mod task_list;
mod toml;

use store::{Store, get_config_dir};
use task::Task;
use task_form::{TaskForm, TaskFormState};
use task_list::{TaskList, TaskListState};
use toml::TomlStore;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Default, Copy, Clone)]
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
    tasks: Vec<Task>,
    store: TomlStore,
}

impl App {
    pub fn new() -> Self {
        let mut store = TomlStore::new().unwrap();
        let mut tasks = store.load_all_tasks().unwrap();
        let data_dir = get_config_dir();
        tasks.push(Task {
            title: data_dir.unwrap().into_os_string().into_string().unwrap(),
            ..Default::default()
        });
        store.save_all_tasks(tasks.clone()).unwrap();

        let task_list1 = TaskListState::new(true);
        let task_list2 = TaskListState::new(false);
        let task_form = TaskFormState::new();
        Self {
            running: true,
            active_widget: ActiveWidget::TaskList1,
            task_list1,
            task_list2,
            task_form,
            tasks,
            store,
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
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(49), Constraint::Percentage(51)])
                .areas(frame.area());

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        frame.render_widget(Clear, left);
        frame.render_stateful_widget(TaskList::new(&self.tasks), left, &mut self.task_list1);
        frame.render_widget(Clear, right);
        frame.render_stateful_widget(TaskList::new(&self.tasks), right, &mut self.task_list2);

        frame.render_stateful_widget(TaskForm {}, frame.area(), &mut self.task_form);
        // frame.render_stateful_widget(TaskList::default(), left, &mut self.task_list);
    }

    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
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
            (_, KeyCode::Char('j')) | (_, KeyCode::Down) => {
                if self.task_form.is_modal_open {
                    return;
                }
                match self.active_widget {
                    ActiveWidget::TaskList1 => {
                        if self.task_form.is_modal_open {
                            return;
                        }
                        self.task_list1.list_state.select_next()
                    }
                    ActiveWidget::TaskList2 => self.task_list2.list_state.select_next(),
                };
            }
            (_, KeyCode::Char('k')) | (_, KeyCode::Up) => {
                if self.task_form.is_modal_open {
                    return;
                }
                match self.active_widget {
                    ActiveWidget::TaskList1 => {
                        if self.task_form.is_modal_open {
                            return;
                        }
                        self.task_list1.list_state.select_previous()
                    }
                    ActiveWidget::TaskList2 => self.task_list2.list_state.select_previous(),
                };
            }
            (KeyModifiers::SHIFT, KeyCode::Tab) => {
                if self.task_form.is_modal_open {
                    return;
                }
                match self.active_widget {
                    ActiveWidget::TaskList1 => self.set_active_widget(ActiveWidget::TaskList2),
                    ActiveWidget::TaskList2 => self.set_active_widget(ActiveWidget::TaskList1),
                };
            }
            (_, KeyCode::Tab) => {
                if self.task_form.is_modal_open {
                    return;
                }
                match self.active_widget {
                    ActiveWidget::TaskList1 => self.set_active_widget(ActiveWidget::TaskList2),
                    ActiveWidget::TaskList2 => self.set_active_widget(ActiveWidget::TaskList1),
                };
            }
            (_, KeyCode::Char(' ')) => {
                if self.task_form.is_modal_open {
                    return;
                }
                match self.active_widget {
                    ActiveWidget::TaskList1 => {
                        self.task_list1
                            .toggle_pick_task(self.task_list1.list_state.selected().unwrap());
                    }
                    ActiveWidget::TaskList2 => self
                        .task_list2
                        .toggle_pick_task(self.task_list2.list_state.selected().unwrap()),
                };
            }
            (_, KeyCode::Char('a')) => match self.active_widget {
                ActiveWidget::TaskList1 => {
                    if let Some(i) = self.task_list1.list_state.selected() {
                        self.task_list1.last_selected = Some(i);
                        self.task_list1.list_state.select(None);
                    } else {
                        self.task_list1
                            .list_state
                            .select(self.task_list1.last_selected);
                    }
                    self.task_form.is_modal_open = !self.task_form.is_modal_open
                }
                ActiveWidget::TaskList2 => {}
            },
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

// impl Widget for &App {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         TasksWidget::default().render(area, buf, &mut self.widgets.task_list);
//     }
// }
