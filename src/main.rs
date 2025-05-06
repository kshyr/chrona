use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::Clear,
};

mod task;
mod task_list;
mod toml;

use task_list::{TaskList, TaskListState};
use toml::{Store, TomlStore};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default, Copy, Clone)]
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
}

impl App {
    pub fn new() -> Self {
        let store = TomlStore::new();
        let mut task_list1 = TaskListState::new(store.clone(), true);
        task_list1.focus_current();
        let task_list2 = TaskListState::new(store.clone(), false);
        Self {
            running: true,
            active_widget: ActiveWidget::TaskList1,
            task_list1,
            task_list2,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
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
        frame.render_stateful_widget(TaskList::default(), left, &mut self.task_list1);
        frame.render_widget(Clear, right);
        frame.render_stateful_widget(TaskList::default(), right, &mut self.task_list2);
        // frame.render_stateful_widget(TaskList::default(), left, &mut self.task_list);
    }

    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
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
            (_, KeyCode::Char('j')) | (_, KeyCode::Down) => match self.active_widget {
                ActiveWidget::TaskList1 => self.task_list1.focus_next(),
                ActiveWidget::TaskList2 => self.task_list2.focus_next(),
            },
            (_, KeyCode::Char('k')) | (_, KeyCode::Up) => match self.active_widget {
                ActiveWidget::TaskList1 => self.task_list1.focus_previous(),
                ActiveWidget::TaskList2 => self.task_list2.focus_previous(),
            },
            (KeyModifiers::SHIFT, KeyCode::Tab) => match self.active_widget {
                ActiveWidget::TaskList1 => self.set_active_widget(ActiveWidget::TaskList2),
                ActiveWidget::TaskList2 => self.set_active_widget(ActiveWidget::TaskList1),
            },
            (_, KeyCode::Tab) => match self.active_widget {
                ActiveWidget::TaskList1 => self.set_active_widget(ActiveWidget::TaskList2),
                ActiveWidget::TaskList2 => self.set_active_widget(ActiveWidget::TaskList1),
            },
            (_, KeyCode::Char(' ')) => match self.active_widget {
                ActiveWidget::TaskList1 => self
                    .task_list1
                    .toggle_pick_task(self.task_list1.list_state.selected().unwrap()),
                ActiveWidget::TaskList2 => self
                    .task_list2
                    .toggle_pick_task(self.task_list2.list_state.selected().unwrap()),
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
