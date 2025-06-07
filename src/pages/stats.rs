use crate::{
    page::{Page, PageAction, cycle_widget_index},
    task::Task,
    toml::TomlStore,
    widgets::task_form::TaskFormState,
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
};
use tui_logger::TuiWidgetState;

#[derive(Default)]
pub struct StatsPageState {
    pub active_widget: StatsWidget,
}

impl StatsPageState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct StatsPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum_macros::EnumIter)]
enum StatsWidget {
    #[default]
    TimeSummary,
    TaskCompletion,
    ActivityTimeline,
}

impl StatsPage {
    pub fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        page_state: &mut StatsPageState,
        _tasks: &[Task],
        _debug_state: &TuiWidgetState,
    ) {
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

        Self::render_not_implemented(
            buf,
            top_chunks[0],
            "Time Summary",
            page_state.active_widget == StatsWidget::TimeSummary,
            0,
        );

        Self::render_not_implemented(
            buf,
            top_chunks[1],
            "Task Completion",
            page_state.active_widget == StatsWidget::TaskCompletion,
            1,
        );

        Self::render_not_implemented(
            buf,
            main_chunks[1],
            "Activity Timeline",
            page_state.active_widget == StatsWidget::ActivityTimeline,
            2,
        );
    }
}

impl Page for StatsPage {
    type State = StatsPageState;

    fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        page_state: &mut Self::State,
        tasks: &[Task],
        debug_state: &TuiWidgetState,
    ) {
        StatsPage::render(self, area, buf, page_state, tasks, debug_state);
    }

    fn handle_key_event(
        &self,
        key: KeyEvent,
        page_state: &mut Self::State,
        _tasks: &mut Vec<Task>,
        _task_form: &mut TaskFormState,
        _store: &mut TomlStore,
        _status_message: &mut Option<(String, ratatui::style::Color)>,
        _debug_state: &mut TuiWidgetState,
    ) -> Option<PageAction> {
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
        state.active_widget = StatsWidget::TimeSummary;
    }

    fn on_deactivate(&self, _state: &mut Self::State) {}

    fn select_widget(&self, index: usize, state: &mut Self::State) {
        state.active_widget = match index {
            0 => StatsWidget::TimeSummary,
            1 => StatsWidget::TaskCompletion,
            2 => StatsWidget::ActivityTimeline,
            _ => state.active_widget,
        };
    }
}
