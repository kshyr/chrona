use std::sync::{Arc, Mutex};

use crate::app::GlobalState;
use crate::page::{Page, PageAction, PageWidget};
use crate::widgets::task_list::{TaskList, TaskListState};
use ratatui::buffer::Buffer;
use ratatui::widgets::StatefulWidget;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
};

#[derive(Default)]
pub struct DashboardPageState {
    global_state: Arc<Mutex<GlobalState>>,
    pub task_list_state: TaskListState,
    pub active_widget: DashboardWidget,
}

impl DashboardPageState {
    pub fn new(global_state: Arc<Mutex<GlobalState>>) -> Self {
        Self {
            global_state: global_state.clone(),
            task_list_state: TaskListState::new(global_state),
            active_widget: DashboardWidget::TaskList,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum DashboardWidget {
    #[default]
    TaskList,
    CurrentSession,
    RecentActivity,
}

#[derive(Default, Clone, Copy)]
pub struct DashboardPage;

impl StatefulWidget for DashboardPage {
    type State = DashboardPageState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
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

        TaskList::default().render(top_chunks[0], buf, &mut state.task_list_state);

        self.render_not_implemented(
            buf,
            top_chunks[1],
            "Current Session",
            state.active_widget == DashboardWidget::CurrentSession,
            1,
        );

        self.render_not_implemented(
            buf,
            main_chunks[1],
            "Recent Activity",
            state.active_widget == DashboardWidget::RecentActivity,
            2,
        );
    }
}

impl Page for DashboardPage {
    fn handle_key_event(self, key: KeyEvent, state: &mut Self::State) -> Option<PageAction> {
        TaskList::handle_key_event(key, &mut state.task_list_state);
        match (key.modifiers, key.code) {
            (modifiers, KeyCode::Tab) => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    state.active_widget = match state.active_widget {
                        DashboardWidget::TaskList => DashboardWidget::RecentActivity,
                        DashboardWidget::CurrentSession => DashboardWidget::TaskList,
                        DashboardWidget::RecentActivity => DashboardWidget::CurrentSession,
                    };
                } else {
                    state.active_widget = match state.active_widget {
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

    fn on_activate(self, state: &mut Self::State) {
        state.task_list_state.is_active = true;
        state.active_widget = DashboardWidget::TaskList;
    }

    fn on_deactivate(self, state: &mut Self::State) {
        state.task_list_state.is_active = false;
    }

    fn select_widget(self, index: usize, state: &mut Self::State) {
        state.active_widget = match index {
            0 => DashboardWidget::TaskList,
            1 => DashboardWidget::CurrentSession,
            2 => DashboardWidget::RecentActivity,
            _ => state.active_widget,
        };
    }
}
