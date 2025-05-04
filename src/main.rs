use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    symbols,
    text::Line,
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState},
};

mod task;
mod task_list;

use task::Task;
use task_list::{TaskList, TaskListState};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    task_list: TaskListState,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            task_list: TaskListState::default(),
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

    fn render(&mut self, frame: &mut Frame) {
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(49), Constraint::Percentage(51)])
                .areas(frame.area());

        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY);

        let items: Vec<ListItem> = self
            .task_list
            .tasks
            .iter()
            .enumerate()
            .map(|(_, todo_item)| ListItem::from(todo_item))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        let mut state = ListState::default();

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        frame.render_stateful_widget(list, left, &mut state);
        frame.render_stateful_widget(TaskList::default(), right, &mut self.task_list);
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
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

impl From<&Task> for ListItem<'_> {
    fn from(task: &Task) -> Self {
        ListItem::new(task.title.clone())
    }
}

// impl Widget for &App {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         TasksWidget::default().render(area, buf, &mut self.widgets.task_list);
//     }
// }
