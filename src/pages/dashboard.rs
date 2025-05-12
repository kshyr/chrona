use crate::page::{Page, PageAction};
use crate::task::Task;
use crate::task_list::{TaskList, TaskListState};
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
};

pub struct DashboardPage {
    task_list_state: TaskListState,
    active_widget: DashboardWidget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DashboardWidget {
    TaskList,
    CurrentSession,
    RecentActivity,
}

impl DashboardPage {
    pub fn new() -> Self {
        Self {
            task_list_state: TaskListState::new(true),
            active_widget: DashboardWidget::TaskList,
        }
    }
}

impl Page for DashboardPage {
    fn render(&mut self, frame: &mut Frame, area: Rect, tasks: &[Task]) {
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

        frame.render_stateful_widget(
            TaskList::new(tasks),
            top_chunks[0],
            &mut self.task_list_state,
        );

        self.render_not_implemented(
            frame,
            top_chunks[1],
            "Current Session",
            self.active_widget == DashboardWidget::CurrentSession,
            1,
        );

        self.render_not_implemented(
            frame,
            main_chunks[1],
            "Recent Activity",
            self.active_widget == DashboardWidget::RecentActivity,
            2,
        );
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<PageAction> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('j')) | (_, KeyCode::Down)
                if self.active_widget == DashboardWidget::TaskList =>
            {
                self.task_list_state.list_state.select_next();
                Some(PageAction::None)
            }
            (_, KeyCode::Char('k')) | (_, KeyCode::Up)
                if self.active_widget == DashboardWidget::TaskList =>
            {
                self.task_list_state.list_state.select_previous();
                Some(PageAction::None)
            }

            (modifiers, KeyCode::Tab) => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    self.active_widget = match self.active_widget {
                        DashboardWidget::TaskList => DashboardWidget::RecentActivity,
                        DashboardWidget::CurrentSession => DashboardWidget::TaskList,
                        DashboardWidget::RecentActivity => DashboardWidget::CurrentSession,
                    };
                } else {
                    self.active_widget = match self.active_widget {
                        DashboardWidget::TaskList => DashboardWidget::CurrentSession,
                        DashboardWidget::CurrentSession => DashboardWidget::RecentActivity,
                        DashboardWidget::RecentActivity => DashboardWidget::TaskList,
                    };
                }
                Some(PageAction::None)
            }

            _ => None,
        }
    }

    fn on_activate(&mut self) {
        self.task_list_state.is_active = true;
        self.active_widget = DashboardWidget::TaskList;
    }

    fn on_deactivate(&mut self) {
        self.task_list_state.is_active = false;
    }

    fn select_widget(&mut self, index: usize) {
        self.active_widget = match index {
            0 => DashboardWidget::TaskList,
            1 => DashboardWidget::CurrentSession,
            2 => DashboardWidget::RecentActivity,
            _ => self.active_widget,
        };
    }
}
