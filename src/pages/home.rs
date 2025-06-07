use crate::{
    page::{Page, PageAction, cycle_widget_index},
    store::Store,
    task::Task,
    toml::TomlStore,
    widgets::{
        task_form::TaskFormState,
        task_list::{TaskList, TaskListAction, TaskListState},
    },
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::Widget as _,
};
use tui_logger::{TuiWidgetEvent, TuiWidgetState};

#[derive(Default)]
pub struct HomePageState {
    pub task_list_state: TaskListState,
    pub active_widget: HomeWidget,
}

impl HomePageState {
    pub fn new() -> Self {
        Self {
            task_list_state: TaskListState::new(),
            active_widget: HomeWidget::TaskList,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum_macros::EnumIter)]
pub enum HomeWidget {
    #[default]
    TaskList,
    CurrentSession,
    RecentActivity,
}

#[derive(Default, Clone, Copy)]
pub struct HomePage;

impl HomePage {
    pub fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        page_state: &mut HomePageState,
        tasks: &[Task],
        debug_state: &TuiWidgetState,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let main_area = chunks[0];

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(main_area);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[0]);

        page_state.task_list_state.is_active = page_state.active_widget == HomeWidget::TaskList;
        TaskList::new(tasks, &mut page_state.task_list_state).render(top_chunks[0], buf);

        Self::render_not_implemented(
            buf,
            top_chunks[1],
            "Current Session",
            page_state.active_widget == HomeWidget::CurrentSession,
            1,
        );

        Self::render_not_implemented(
            buf,
            main_chunks[1],
            "Recent Activity",
            page_state.active_widget == HomeWidget::RecentActivity,
            2,
        );
    }
}

impl Page for HomePage {
    type State = HomePageState;

    fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        page_state: &mut Self::State,
        tasks: &[Task],
        debug_state: &TuiWidgetState,
    ) {
        HomePage::render(self, area, buf, page_state, tasks, debug_state);
    }

    fn handle_key_event(
        &self,
        key: KeyEvent,
        page_state: &mut Self::State,
        tasks: &mut Vec<Task>,
        task_form: &mut TaskFormState,
        store: &mut TomlStore,
        status_message: &mut Option<(String, Color)>,
        debug_state: &mut TuiWidgetState,
    ) -> Option<PageAction> {
        match page_state.active_widget {
            HomeWidget::TaskList => {
                let task_list_action =
                    TaskListState::handle_key_event(key, &mut page_state.task_list_state, tasks);
                match task_list_action {
                    TaskListAction::AddTask => {
                        task_form.open_for_create();
                        return Some(PageAction::None);
                    }
                    TaskListAction::EditSelectedTask(task_id) => {
                        if let Some(task) = tasks.iter().find(|t| t.id == task_id).cloned() {
                            task_form.open_for_edit(&task);
                        } else {
                            *status_message =
                                Some(("Task not found for editing.".to_string(), Color::Red));
                        }
                        return Some(PageAction::None);
                    }
                    TaskListAction::DeleteSelectedTask(task_id) => {
                        if let Some(idx) = tasks.iter().position(|t| t.id == task_id) {
                            tasks.remove(idx);
                            if let Err(e) = store.save_all_tasks(tasks.clone()) {
                                *status_message = Some((
                                    format!("Failed to save tasks after delete: {}", e),
                                    Color::Red,
                                ));
                            } else {
                                *status_message =
                                    Some(("Task deleted successfully.".to_string(), Color::Green));
                            }
                        } else {
                            *status_message =
                                Some(("Task to delete not found.".to_string(), Color::Red));
                        }
                        return Some(PageAction::None);
                    }
                    TaskListAction::SelectNext
                    | TaskListAction::SelectPrevious
                    | TaskListAction::TogglePickSelected
                    | TaskListAction::None => {
                        return Some(PageAction::None);
                    }
                }
            }
            HomeWidget::CurrentSession => {
                let debug_widget_event = match key.code {
                    KeyCode::Char(' ') => Some(TuiWidgetEvent::SpaceKey),
                    KeyCode::Esc => Some(TuiWidgetEvent::EscapeKey),
                    KeyCode::PageUp | KeyCode::Char('K') => Some(TuiWidgetEvent::PrevPageKey),
                    KeyCode::PageDown | KeyCode::Char('J') => Some(TuiWidgetEvent::NextPageKey),
                    KeyCode::Up => Some(TuiWidgetEvent::UpKey),
                    KeyCode::Down => Some(TuiWidgetEvent::DownKey),
                    KeyCode::Left => Some(TuiWidgetEvent::LeftKey),
                    KeyCode::Right => Some(TuiWidgetEvent::RightKey),
                    KeyCode::Char('+') => Some(TuiWidgetEvent::PlusKey),
                    KeyCode::Char('-') => Some(TuiWidgetEvent::MinusKey),
                    KeyCode::Char('h') => Some(TuiWidgetEvent::HideKey),
                    KeyCode::Char('f') => Some(TuiWidgetEvent::FocusKey),
                    _ => None,
                };

                if let Some(event) = debug_widget_event {
                    debug_state.transition(event);
                    return Some(PageAction::None);
                }
            }
            HomeWidget::RecentActivity => { /* TODO: Handle keys for RecentActivity */ }
        }

        match (key.modifiers, key.code) {
            (modifiers, KeyCode::Tab) => {
                page_state.active_widget = cycle_widget_index(
                    page_state.active_widget,
                    modifiers.contains(KeyModifiers::SHIFT),
                );
                Some(PageAction::None)
            }
            _ => None,
        }
    }

    fn on_activate(&self, state: &mut Self::State) {
        state.task_list_state.is_active = true;
        state.active_widget = HomeWidget::TaskList;
    }

    fn on_deactivate(&self, state: &mut Self::State) {
        state.task_list_state.is_active = false;
    }

    fn select_widget(&self, index: usize, state: &mut Self::State) {
        state.active_widget = match index {
            0 => HomeWidget::TaskList,
            1 => HomeWidget::CurrentSession,
            2 => HomeWidget::RecentActivity,
            _ => state.active_widget,
        };

        state.task_list_state.is_active = state.active_widget == HomeWidget::TaskList;
    }
}
