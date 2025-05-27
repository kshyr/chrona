use std::sync::{Arc, Mutex};

use crate::app::GlobalState;
use crate::page::{Page, PageAction};
use ratatui::widgets::StatefulWidget;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
};

#[derive(Default)]
pub struct StatsPageState {
    global_state: Arc<Mutex<GlobalState>>,
    active_widget: StatsWidget,
}

impl StatsPageState {
    pub fn new(global_state: Arc<Mutex<GlobalState>>) -> Self {
        Self {
            global_state,
            ..Default::default()
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct StatsPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum StatsWidget {
    #[default]
    TimeSummary,
    TaskCompletion,
    ActivityTimeline,
}

impl StatefulWidget for StatsPage {
    type State = StatsPageState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let main_area = chunks[0];

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_area);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[0]);

        self.render_not_implemented(
            buf,
            top_chunks[0],
            "Time Summary",
            state.active_widget == StatsWidget::TimeSummary,
            0,
        );

        self.render_not_implemented(
            buf,
            top_chunks[1],
            format!("{:?}", state.active_widget).as_str(),
            state.active_widget == StatsWidget::TaskCompletion,
            1,
        );

        self.render_not_implemented(
            buf,
            main_chunks[1],
            "Activity Timeline",
            state.active_widget == StatsWidget::ActivityTimeline,
            2,
        );
    }
}

impl Page for StatsPage {
    fn handle_key_event(self, key: KeyEvent, state: &mut Self::State) -> Option<PageAction> {
        match (key.modifiers, key.code) {
            (modifiers, KeyCode::Tab) => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    state.active_widget = match state.active_widget {
                        StatsWidget::TimeSummary => StatsWidget::ActivityTimeline,
                        StatsWidget::TaskCompletion => StatsWidget::TimeSummary,
                        StatsWidget::ActivityTimeline => StatsWidget::TaskCompletion,
                    };
                } else {
                    state.active_widget = match state.active_widget {
                        StatsWidget::TimeSummary => StatsWidget::TaskCompletion,
                        StatsWidget::TaskCompletion => StatsWidget::ActivityTimeline,
                        StatsWidget::ActivityTimeline => StatsWidget::TimeSummary,
                    };
                }
                Some(PageAction::None)
            }

            _ => None,
        }
    }

    fn on_activate(self, state: &mut Self::State) {}

    fn on_deactivate(self, state: &mut Self::State) {}

    fn select_widget(self, index: usize, state: &mut Self::State) {
        state.active_widget = match index {
            0 => StatsWidget::TimeSummary,
            1 => StatsWidget::TaskCompletion,
            2 => StatsWidget::ActivityTimeline,
            _ => state.active_widget,
        };
    }
}
