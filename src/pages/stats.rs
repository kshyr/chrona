use crate::page::{Page, PageAction, PageType};
use crate::task::Task;
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
};

pub struct StatsPage {
    active_widget: StatsWidget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatsWidget {
    TimeSummary,
    TaskCompletion,
    ActivityTimeline,
}

impl StatsPage {
    pub fn new() -> Self {
        Self {
            active_widget: StatsWidget::TimeSummary,
        }
    }
}

impl Page for StatsPage {
    fn render(&mut self, frame: &mut Frame, area: Rect, _tasks: &[Task]) {
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
            frame,
            top_chunks[0],
            "Time Summary",
            self.active_widget == StatsWidget::TimeSummary,
            0,
        );

        self.render_not_implemented(
            frame,
            top_chunks[1],
            format!("{:?}", self.active_widget).as_str(),
            self.active_widget == StatsWidget::TaskCompletion,
            1,
        );

        self.render_not_implemented(
            frame,
            main_chunks[1],
            "Activity Timeline",
            self.active_widget == StatsWidget::ActivityTimeline,
            2,
        );
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<PageAction> {
        match (key.modifiers, key.code) {
            (modifiers, KeyCode::Tab) => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    self.active_widget = match self.active_widget {
                        StatsWidget::TimeSummary => StatsWidget::ActivityTimeline,
                        StatsWidget::TaskCompletion => StatsWidget::TimeSummary,
                        StatsWidget::ActivityTimeline => StatsWidget::TaskCompletion,
                    };
                } else {
                    self.active_widget = match self.active_widget {
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

    fn on_activate(&mut self) {
        self.active_widget = StatsWidget::TimeSummary;
    }

    fn on_deactivate(&mut self) {}

    fn select_widget(&mut self, index: usize) {
        self.active_widget = match index {
            0 => StatsWidget::TimeSummary,
            1 => StatsWidget::TaskCompletion,
            2 => StatsWidget::ActivityTimeline,
            _ => self.active_widget,
        };
    }
}
