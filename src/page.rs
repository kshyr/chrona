use std::sync::{Arc, Mutex};

use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};
use strum_macros::{Display, EnumIter};

use crate::{
    app::GlobalState,
    pages::{
        DashboardPage, FocusPage, PlanningPage, StatsPage, dashboard::DashboardPageState,
        focus::FocusPageState, planning::PlanningPageState, stats::StatsPageState,
    },
    widgets::task_list::TaskList,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Default)]
pub enum PageType {
    #[default]
    Dashboard,
    Planning,
    Focus,
    Stats,
}

pub trait Page: StatefulWidget {
    fn render_not_implemented(
        &self,
        buf: &mut Buffer,
        area: Rect,
        title: &str,
        is_active: bool,
        index: usize,
    ) {
        let mut block = Block::bordered()
            .title(Line::from(vec![
                Span::from("["),
                Span::from((index + 1).to_string()),
                Span::from("] "),
                Span::from(title).bold(),
            ]))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().yellow());

        if !is_active {
            block = block
                .border_style(Style::default().add_modifier(Modifier::DIM))
                .border_type(BorderType::Rounded)
        }

        let text = Text::from("Not implemented yet");
        Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(Color::DarkGray))
            .centered()
            .render(area, buf);
    }
    fn handle_key_event(self, key: KeyEvent, state: &mut Self::State) -> Option<PageAction>;
    fn select_widget(self, index: usize, state: &mut Self::State);
    fn on_activate(self, state: &mut Self::State);
    fn on_deactivate(self, state: &mut Self::State);
}

pub trait PageWidget: StatefulWidget {
    fn handle_key_event(key: KeyEvent, state: &mut Self::State);
    fn render_help(&self, area: Rect, buf: &mut Buffer);
}

pub enum PageAction {
    SelectWidget(usize),
    NavigateTo(PageType),
    None,
}

#[derive(Default)]
pub struct PageManager {
    pub global_state: Arc<Mutex<GlobalState>>,
    active_widget_index: usize,
    pub active_page: PageType,
    pub dashboard_page: DashboardPage,
    pub dashboard_page_state: DashboardPageState,
    pub planning_page: PlanningPage,
    pub planning_page_state: PlanningPageState,
    pub focus_page: FocusPage,
    pub focus_page_state: FocusPageState,
    pub stats_page: StatsPage,
    pub stats_page_state: StatsPageState,
}

impl PageManager {
    pub fn new(global_state: Arc<Mutex<GlobalState>>) -> Self {
        Self {
            global_state: global_state.clone(),
            active_widget_index: 0,
            dashboard_page_state: DashboardPageState::new(global_state.clone()),
            planning_page_state: PlanningPageState::new(global_state.clone()),
            focus_page_state: FocusPageState::new(global_state.clone()),
            stats_page_state: StatsPageState::new(global_state.clone()),
            ..Default::default()
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let buf = frame.buffer_mut();
        let [main_area, help_area] =
            Layout::vertical([Constraint::Percentage(100), Constraint::Min(1)])
                .spacing(-1)
                .areas(area);
        self.render_help(help_area, buf);

        match self.active_page {
            PageType::Dashboard => {
                self.dashboard_page
                    .render(main_area, buf, &mut self.dashboard_page_state)
            }
            PageType::Planning => {
                self.planning_page
                    .render(main_area, buf, &mut self.planning_page_state)
            }
            PageType::Focus => self
                .focus_page
                .render(main_area, buf, &mut self.focus_page_state),
            PageType::Stats => self
                .stats_page
                .render(main_area, buf, &mut self.stats_page_state),
        }
    }

    fn unimplemented(&self, area: Rect, buf: &mut Buffer) {
        buf.set_string(
            area.x,
            area.y,
            "Unimplemented",
            Style::default().fg(Color::Red),
        );
    }

    fn render_help(&mut self, area: Rect, buf: &mut Buffer) {
        match self.active_page {
            PageType::Dashboard => match self.active_widget_index {
                0 => TaskList::default().render_help(area, buf),
                _ => self.unimplemented(area, buf),
            },
            PageType::Planning => match self.active_widget_index {
                0 => TaskList::default().render_help(area, buf),
                _ => self.unimplemented(area, buf),
            },
            PageType::Focus => match self.active_widget_index {
                _ => self.unimplemented(area, buf),
            },
            PageType::Stats => match self.active_widget_index {
                _ => self.unimplemented(area, buf),
            },
        };
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<PageAction> {
        let global_action = match (key.modifiers, key.code) {
            (_, KeyCode::Char('D')) => Some(PageAction::NavigateTo(PageType::Dashboard)),
            (_, KeyCode::Char('P')) => Some(PageAction::NavigateTo(PageType::Planning)),
            (_, KeyCode::Char('F')) => Some(PageAction::NavigateTo(PageType::Focus)),
            (_, KeyCode::Char('S')) => Some(PageAction::NavigateTo(PageType::Stats)),

            (_, KeyCode::Char('1')) => Some(PageAction::SelectWidget(0)),
            (_, KeyCode::Char('2')) => Some(PageAction::SelectWidget(1)),
            (_, KeyCode::Char('3')) => Some(PageAction::SelectWidget(2)),
            _ => None,
        };

        let page_action = match self.active_page {
            PageType::Dashboard => self
                .dashboard_page
                .handle_key_event(key, &mut self.dashboard_page_state),
            PageType::Planning => self
                .planning_page
                .handle_key_event(key, &mut self.planning_page_state),
            PageType::Focus => self
                .focus_page
                .handle_key_event(key, &mut self.focus_page_state),
            PageType::Stats => self
                .stats_page
                .handle_key_event(key, &mut self.stats_page_state),
        };

        let action = global_action.or(page_action);

        if let Some(action) = action {
            match action {
                PageAction::NavigateTo(page_type) => {
                    self.change_page(page_type);
                }
                PageAction::SelectWidget(index) => {
                    self.select_widget(index);
                }
                PageAction::None => {}
            }
            return Some(PageAction::None);
        }

        action
    }

    pub fn select_widget(&mut self, index: usize) {
        self.active_widget_index = index;
        match self.active_page {
            PageType::Dashboard => {
                self.dashboard_page
                    .select_widget(index, &mut self.dashboard_page_state);
            }
            PageType::Planning => {
                self.planning_page
                    .select_widget(index, &mut self.planning_page_state);
            }
            PageType::Focus => {
                self.focus_page
                    .select_widget(index, &mut self.focus_page_state);
            }
            PageType::Stats => {
                self.stats_page
                    .select_widget(index, &mut self.stats_page_state);
            }
        }
    }

    pub fn change_page(&mut self, page_type: PageType) {
        if self.active_page == page_type {
            return;
        }

        match self.active_page {
            PageType::Dashboard => self
                .dashboard_page
                .on_deactivate(&mut self.dashboard_page_state),
            PageType::Planning => self
                .planning_page
                .on_deactivate(&mut self.planning_page_state),
            PageType::Focus => self.focus_page.on_deactivate(&mut self.focus_page_state),
            PageType::Stats => self.stats_page.on_deactivate(&mut self.stats_page_state),
        }

        self.active_page = page_type;
        self.active_widget_index = 0;

        match self.active_page {
            PageType::Dashboard => self
                .dashboard_page
                .on_activate(&mut self.dashboard_page_state),
            PageType::Planning => self
                .planning_page
                .on_activate(&mut self.planning_page_state),
            PageType::Focus => self.focus_page.on_activate(&mut self.focus_page_state),
            PageType::Stats => self.stats_page.on_activate(&mut self.stats_page_state),
        }
    }
}
