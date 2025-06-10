use crate::{
    app::StatusMessage,
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
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget as _},
};

#[derive(Default)]
pub struct PlanningPageState {
    pub task_list_state: TaskListState,
    pub active_widget: PlanningWidget,
}

impl PlanningPageState {
    pub fn new() -> Self {
        Self {
            task_list_state: TaskListState::new(),
            active_widget: PlanningWidget::TaskList,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct PlanningPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum_macros::EnumIter)]
pub enum PlanningWidget {
    #[default]
    TaskList,
    TaskDetail,
}

impl PlanningPage {
    pub fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        page_state: &mut PlanningPageState,
        tasks: &[Task],
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let main_area = chunks[0];

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_area);

        page_state.task_list_state.is_active = page_state.active_widget == PlanningWidget::TaskList;
        TaskList::new(tasks, &mut page_state.task_list_state).render(main_chunks[0], buf);

        self.render_task_detail(main_chunks[1], buf, page_state, tasks);
    }
}

impl PlanningPage {
    fn render_task_detail(
        &self,
        area: Rect,
        buf: &mut Buffer,
        page_state: &mut PlanningPageState,
        tasks: &[Task],
    ) {
        let block_title = if page_state.active_widget == PlanningWidget::TaskDetail {
            "Task Details (Active)"
        } else {
            "Task Details"
        };
        let block = Block::default()
            .title(block_title)
            .borders(Borders::ALL)
            .border_style(if page_state.active_widget == PlanningWidget::TaskDetail {
                Style::default()
            } else {
                Style::default().add_modifier(Modifier::DIM)
            });

        if let Some(selected_idx) = page_state.task_list_state.list_state.selected() {
            if let Some(task) = tasks.get(selected_idx) {
                let status_style = Style::default().fg(task.status.color());

                let created_at = task.created_at.format("%Y-%m-%d %H:%M").to_string();
                let updated_at = task.updated_at.format("%Y-%m-%d %H:%M").to_string();

                let mut text_lines = Vec::new();
                text_lines.push(Line::from(vec![
                    Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&task.title),
                ]));

                text_lines.push(Line::from(vec![
                    Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(format!("{:?}", task.status), status_style),
                ]));

                text_lines.push(Line::from(vec![
                    Span::styled("Created: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(created_at),
                ]));

                text_lines.push(Line::from(vec![
                    Span::styled("Updated: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(updated_at),
                ]));

                text_lines.push(Line::default());
                text_lines.push(Line::from(Span::styled(
                    "Description:",
                    Style::default().add_modifier(Modifier::BOLD),
                )));

                if let Some(desc) = &task.description {
                    for line in desc.lines() {
                        text_lines.push(Line::from(line));
                    }
                } else {
                    text_lines.push(Line::from(Span::styled(
                        "(No description)",
                        Style::default().fg(Color::DarkGray),
                    )));
                }

                Paragraph::new(Text::from(text_lines))
                    .block(block)
                    .render(area, buf);
            } else {
                Paragraph::new("No task selected or task not found")
                    .block(block)
                    .style(Style::default().fg(Color::DarkGray))
                    .centered()
                    .render(area, buf);
            }
        } else {
            Paragraph::new("No task selected")
                .block(block)
                .style(Style::default().fg(Color::DarkGray))
                .centered()
                .render(area, buf);
        }
    }
}

impl Page for PlanningPage {
    type State = PlanningPageState;

    fn render(self, area: Rect, buf: &mut Buffer, page_state: &mut Self::State, tasks: &[Task]) {
        PlanningPage::render(self, area, buf, page_state, tasks);
    }

    fn handle_key_event(
        &self,
        key: KeyEvent,
        page_state: &mut Self::State,
        tasks: &mut Vec<Task>,
        task_form: &mut TaskFormState,
        store: &mut TomlStore,
        status_message: &mut StatusMessage,
    ) -> Option<PageAction> {
        let action_from_widget = match page_state.active_widget {
            PlanningWidget::TaskList => {
                let task_list_action =
                    TaskListState::handle_key_event(key, &mut page_state.task_list_state, tasks);
                match task_list_action {
                    TaskListAction::AddTask => {
                        task_form.open_for_create();
                        Some(PageAction::None)
                    }
                    TaskListAction::EditSelectedTask(task_id) => {
                        if let Some(task) = tasks.iter().find(|t| t.id == task_id).cloned() {
                            task_form.open_for_edit(&task);
                        } else {
                            *status_message =
                                StatusMessage::error("Task not found for editing.".into());
                        }
                        Some(PageAction::None)
                    }
                    TaskListAction::DeleteSelectedTask(task_id) => {
                        if let Some(idx) = tasks.iter().position(|t| t.id == task_id) {
                            tasks.remove(idx);
                            if let Err(e) = store.save_all_tasks(tasks.clone()) {
                                *status_message = StatusMessage::error(format!(
                                    "Failed to save tasks after delete: {}",
                                    e
                                ));
                            } else {
                                *status_message =
                                    StatusMessage::success("Task deleted successfully".into());
                            }
                        } else {
                            *status_message =
                                StatusMessage::error("Task to delete not found.".into());
                        }
                        Some(PageAction::None)
                    }
                    TaskListAction::SelectNext
                    | TaskListAction::SelectPrevious
                    | TaskListAction::TogglePickSelected
                    | TaskListAction::None => Some(PageAction::None),
                }
            }
            PlanningWidget::TaskDetail => None,
        };

        if action_from_widget.is_some() {
            return action_from_widget;
        }

        match (key.modifiers, key.code) {
            (modifiers, KeyCode::Tab) => {
                page_state.active_widget = cycle_widget_index(
                    page_state.active_widget,
                    modifiers.contains(KeyModifiers::SHIFT),
                );

                page_state.task_list_state.is_active =
                    page_state.active_widget == PlanningWidget::TaskList;
                Some(PageAction::None)
            }
            _ => None,
        }
    }

    fn on_activate(&self, state: &mut Self::State) {
        state.active_widget = PlanningWidget::TaskList;
        state.task_list_state.is_active = true;
    }

    fn on_deactivate(&self, state: &mut Self::State) {
        state.task_list_state.is_active = false;
    }

    fn select_widget(&self, index: usize, state: &mut Self::State) {
        state.active_widget = match index {
            0 => PlanningWidget::TaskList,
            1 => PlanningWidget::TaskDetail,
            _ => state.active_widget,
        };
        state.task_list_state.is_active = state.active_widget == PlanningWidget::TaskList;
    }
}
