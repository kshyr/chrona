use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{
    app::{App, StatusMessage},
    pages::{
        FocusPage, HomePage, PlanningPage, StatsPage, focus::FocusPageState, home::HomePageState,
        planning::PlanningPageState, stats::StatsPageState,
    },
    task::Task,
    toml::TomlStore,
    widgets::{
        task_form::TaskFormState,
        task_list::{TaskList, TaskListState},
    },
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use tui_logger::TuiWidgetState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Default)]
pub enum PageType {
    #[default]
    Home,
    Planning,
    Focus,
    Stats,
}

pub fn cycle_widget_index<T: IntoEnumIterator + PartialEq + Copy + Default>(
    current_widget: T,
    shift_pressed: bool,
) -> T {
    let all_widgets: Vec<T> = T::iter().collect();
    let current_idx = all_widgets
        .iter()
        .position(|&w| w == current_widget)
        .unwrap_or(0);
    let next_idx = if shift_pressed {
        (current_idx + all_widgets.len() - 1) % all_widgets.len()
    } else {
        (current_idx + 1) % all_widgets.len()
    };
    all_widgets[next_idx]
}

pub trait Page {
    type State: 'static + Default;

    fn render_not_implemented(
        buf: &mut Buffer,
        area: Rect,
        title: &str,
        is_active: bool,
        index: usize,
    ) {
        let mut border_style = Style::default().dim();
        if is_active {
            border_style = border_style.not_dim().yellow();
        }

        let block = Block::bordered()
            .title(Line::from(vec![
                Span::from("["),
                Span::from((index + 1).to_string()),
                Span::from("] "),
                Span::from(title).bold(),
            ]))
            .border_type(BorderType::Rounded)
            .border_style(border_style);

        let text = Text::from("Not implemented yet");
        Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(Color::DarkGray))
            .centered()
            .render(area, buf);
    }

    fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        page_state: &mut Self::State,
        tasks: &[Task],
        debug_state: &TuiWidgetState,
    );

    fn handle_key_event(
        &self,
        key: KeyEvent,
        page_state: &mut Self::State,
        tasks: &mut Vec<Task>,
        task_form: &mut TaskFormState,
        store: &mut TomlStore,
        status_message: &mut StatusMessage,
        debug_state: &mut TuiWidgetState,
    ) -> Option<PageAction>;

    fn select_widget(&self, index: usize, state: &mut Self::State);
    fn on_activate(&self, state: &mut Self::State);
    fn on_deactivate(&self, state: &mut Self::State);
}

pub trait PageWidget {
    fn handle_key_event(&self, key: KeyEvent, app: &mut App);
    fn render_help(&self, area: Rect, buf: &mut Buffer);
}

pub enum PageAction {
    SelectWidget(usize),
    NavigateTo(PageType),
    None,
}

#[derive(Default)]
pub struct PageManager {
    active_widget_index: usize,
    pub active_page: PageType,
    pub home_page_state: HomePageState,
    pub planning_page_state: PlanningPageState,
    pub focus_page_state: FocusPageState,
    pub stats_page_state: StatsPageState,

    home_page: HomePage,
    planning_page: PlanningPage,
    focus_page: FocusPage,
    stats_page: StatsPage,
}

impl PageManager {
    pub fn new() -> Self {
        Self {
            active_widget_index: 0,
            active_page: PageType::default(),
            home_page_state: HomePageState::new(),
            planning_page_state: PlanningPageState::new(),
            focus_page_state: FocusPageState::new(),
            stats_page_state: StatsPageState::new(),
            home_page: HomePage::default(),
            planning_page: PlanningPage::default(),
            focus_page: FocusPage::default(),
            stats_page: StatsPage::default(),
        }
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        tasks: &[Task],
        debug_state: &TuiWidgetState,
    ) {
        let buf = frame.buffer_mut();
        let [main_area, help_area] =
            Layout::vertical([Constraint::Percentage(100), Constraint::Min(1)])
                .spacing(-1)
                .areas(area);
        self.render_help(help_area, buf);

        match self.active_page {
            PageType::Home => {
                self.home_page.render(
                    main_area,
                    buf,
                    &mut self.home_page_state,
                    tasks,
                    debug_state,
                );
            }
            PageType::Planning => {
                self.planning_page.render(
                    main_area,
                    buf,
                    &mut self.planning_page_state,
                    tasks,
                    debug_state,
                );
            }
            PageType::Focus => {
                self.focus_page.render(
                    main_area,
                    buf,
                    &mut self.focus_page_state,
                    tasks,
                    debug_state,
                );
            }
            PageType::Stats => {
                self.stats_page.render(
                    main_area,
                    buf,
                    &mut self.stats_page_state,
                    tasks,
                    debug_state,
                );
            }
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
        let dummy_tasks = [];
        let mut dummy_task_list_state = TaskListState::new();

        match self.active_page {
            PageType::Home => match self.home_page_state.active_widget {
                crate::pages::home::HomeWidget::TaskList => {
                    TaskList::new(&dummy_tasks, &mut dummy_task_list_state).render_help(area, buf);
                }
                _ => self.unimplemented(area, buf),
            },
            PageType::Planning => match self.planning_page_state.active_widget {
                crate::pages::planning::PlanningWidget::TaskList => {
                    TaskList::new(&dummy_tasks, &mut dummy_task_list_state).render_help(area, buf);
                }
                _ => self.unimplemented(area, buf),
            },
            PageType::Focus => {
                Paragraph::new("Focus Page Help: [t] select task, [p] pause, [s] stop")
                    .style(Style::default().fg(Color::DarkGray))
                    .render(area, buf);
            }
            PageType::Stats => {
                Paragraph::new("Stats Page Help: (No specific actions yet)")
                    .style(Style::default().fg(Color::DarkGray))
                    .render(area, buf);
            }
        };
    }

    pub fn handle_key_event(
        &mut self,
        key: KeyEvent,
        tasks: &mut Vec<Task>,
        task_form: &mut TaskFormState,
        store: &mut TomlStore,
        status_message: &mut StatusMessage,
        debug_state: &mut TuiWidgetState,
    ) -> Option<PageAction> {
        let global_action = match (key.modifiers, key.code) {
            (_, KeyCode::Char('H')) => Some(PageAction::NavigateTo(PageType::Home)),
            (_, KeyCode::Char('P')) => Some(PageAction::NavigateTo(PageType::Planning)),
            (_, KeyCode::Char('F')) => Some(PageAction::NavigateTo(PageType::Focus)),
            (_, KeyCode::Char('S')) => Some(PageAction::NavigateTo(PageType::Stats)),
            (_, KeyCode::Char('1')) => Some(PageAction::SelectWidget(0)),
            (_, KeyCode::Char('2')) => Some(PageAction::SelectWidget(1)),
            (_, KeyCode::Char('3')) => Some(PageAction::SelectWidget(2)),
            _ => None,
        };

        if let Some(action) = global_action {
            if let PageAction::SelectWidget(index) = action {
                self.active_widget_index = index;
            }
            self.apply_action_internal(action);
            return Some(PageAction::None);
        }

        let page_specific_action_option = match self.active_page {
            PageType::Home => self.home_page.handle_key_event(
                key,
                &mut self.home_page_state,
                tasks,
                task_form,
                store,
                status_message,
                debug_state,
            ),
            PageType::Planning => self.planning_page.handle_key_event(
                key,
                &mut self.planning_page_state,
                tasks,
                task_form,
                store,
                status_message,
                debug_state,
            ),
            PageType::Focus => self.focus_page.handle_key_event(
                key,
                &mut self.focus_page_state,
                tasks,
                task_form,
                store,
                status_message,
                debug_state,
            ),
            PageType::Stats => self.stats_page.handle_key_event(
                key,
                &mut self.stats_page_state,
                tasks,
                task_form,
                store,
                status_message,
                debug_state,
            ),
        };

        if let Some(page_action) = page_specific_action_option {
            self.apply_action_internal(page_action);
            return Some(PageAction::None);
        }

        None
    }

    fn apply_action_internal(&mut self, action: PageAction) {
        match action {
            PageAction::NavigateTo(page_type) => {
                self.change_page(page_type);
            }
            PageAction::SelectWidget(index) => {
                self.select_widget(index);
            }
            PageAction::None => {}
        }
    }

    pub fn select_widget(&mut self, index: usize) {
        self.active_widget_index = index;
        match self.active_page {
            PageType::Home => {
                self.home_page
                    .select_widget(index, &mut self.home_page_state);
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
            PageType::Home => self.home_page.on_deactivate(&mut self.home_page_state),
            PageType::Planning => self
                .planning_page
                .on_deactivate(&mut self.planning_page_state),
            PageType::Focus => self.focus_page.on_deactivate(&mut self.focus_page_state),
            PageType::Stats => self.stats_page.on_deactivate(&mut self.stats_page_state),
        }

        self.active_page = page_type;
        self.active_widget_index = 0;

        match self.active_page {
            PageType::Home => self.home_page.on_activate(&mut self.home_page_state),
            PageType::Planning => self
                .planning_page
                .on_activate(&mut self.planning_page_state),
            PageType::Focus => self.focus_page.on_activate(&mut self.focus_page_state),
            PageType::Stats => self.stats_page.on_activate(&mut self.stats_page_state),
        }
        self.select_widget(0);
    }
}
