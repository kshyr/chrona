use crate::task::Task;
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph},
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageType {
    Dashboard,
    Planning,
    Focus,
    Stats,
}

impl Display for PageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PageType::Dashboard => write!(f, "Dashboard"),
            PageType::Planning => write!(f, "Planning"),
            PageType::Focus => write!(f, "Focus"),
            PageType::Stats => write!(f, "Stats"),
        }
    }
}

pub trait Page {
    fn render(&mut self, frame: &mut Frame, area: Rect, tasks: &[Task]);

    fn render_not_implemented(
        &self,
        frame: &mut Frame,
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
            .border_type(BorderType::Thick);

        if !is_active {
            block = block
                .border_style(Style::default().add_modifier(Modifier::DIM))
                .border_type(BorderType::Plain)
                .dim();
        }

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let text = Text::from("Not implemented yet");
        frame.render_widget(
            Paragraph::new(text)
                .style(Style::default().fg(Color::DarkGray))
                .centered(),
            inner,
        );
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<PageAction>;

    fn select_widget(&mut self, index: usize);

    fn on_activate(&mut self) {}

    fn on_deactivate(&mut self) {}
}

pub enum PageAction {
    SelectWidget(usize),
    NavigateTo(PageType),
    None,
}

pub struct PageManager {
    active_page: PageType,
    dashboard_page: Box<dyn Page>,
    planning_page: Box<dyn Page>,
    focus_page: Box<dyn Page>,
    stats_page: Box<dyn Page>,
}

impl PageManager {
    pub fn new(
        dashboard_page: Box<dyn Page>,
        planning_page: Box<dyn Page>,
        focus_page: Box<dyn Page>,
        stats_page: Box<dyn Page>,
    ) -> Self {
        Self {
            active_page: PageType::Dashboard,
            dashboard_page,
            planning_page,
            focus_page,
            stats_page,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, tasks: &[Task]) {
        match self.active_page {
            PageType::Dashboard => self.dashboard_page.render(frame, area, tasks),
            PageType::Planning => self.planning_page.render(frame, area, tasks),
            PageType::Focus => self.focus_page.render(frame, area, tasks),
            PageType::Stats => self.stats_page.render(frame, area, tasks),
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<PageAction> {
        let global_action = match (key.modifiers, key.code) {
            (_, KeyCode::Char('!')) => Some(PageAction::NavigateTo(PageType::Dashboard)),
            (_, KeyCode::Char('@')) => Some(PageAction::NavigateTo(PageType::Planning)),
            (_, KeyCode::Char('#')) => Some(PageAction::NavigateTo(PageType::Focus)),
            (_, KeyCode::Char('$')) => Some(PageAction::NavigateTo(PageType::Stats)),

            (_, KeyCode::Char('1')) => Some(PageAction::SelectWidget(0)),
            (_, KeyCode::Char('2')) => Some(PageAction::SelectWidget(1)),
            (_, KeyCode::Char('3')) => Some(PageAction::SelectWidget(2)),
            _ => None,
        };

        let page_action = match self.active_page {
            PageType::Dashboard => self.dashboard_page.handle_key_event(key),
            PageType::Planning => self.planning_page.handle_key_event(key),
            PageType::Focus => self.focus_page.handle_key_event(key),
            PageType::Stats => self.stats_page.handle_key_event(key),
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
        match self.active_page {
            PageType::Dashboard => self.dashboard_page.select_widget(index),
            PageType::Planning => self.planning_page.select_widget(index),
            PageType::Focus => self.focus_page.select_widget(index),
            PageType::Stats => self.stats_page.select_widget(index),
        }
    }

    pub fn active_page(&self) -> PageType {
        self.active_page
    }

    pub fn change_page(&mut self, page_type: PageType) {
        if self.active_page == page_type {
            return;
        }

        match self.active_page {
            PageType::Dashboard => self.dashboard_page.on_deactivate(),
            PageType::Planning => self.planning_page.on_deactivate(),
            PageType::Focus => self.focus_page.on_deactivate(),
            PageType::Stats => self.stats_page.on_deactivate(),
        }

        self.active_page = page_type;

        match self.active_page {
            PageType::Dashboard => self.dashboard_page.on_activate(),
            PageType::Planning => self.planning_page.on_activate(),
            PageType::Focus => self.focus_page.on_activate(),
            PageType::Stats => self.stats_page.on_activate(),
        }
    }
}
