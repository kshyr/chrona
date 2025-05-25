use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Clear, Paragraph, StatefulWidget, Widget},
};

use crate::ui::modal::centered_rect;

pub struct HelpState {
    pub visible: bool,
}

impl Default for HelpState {
    fn default() -> Self {
        Self { visible: false }
    }
}

impl HelpState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    // pub fn show(&mut self) {
    //     self.visible = true;
    // }

    pub fn hide(&mut self) {
        self.visible = false;
    }
}

pub struct HelpScreen;

impl StatefulWidget for HelpScreen {
    type State = HelpState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if !state.visible {
            return;
        }

        Block::default().dim().render(area, buf);

        let help_area = centered_rect(80, 80, area);
        Clear.render(help_area, buf);

        let help_block = Block::bordered()
            .border_type(BorderType::Thick)
            .title("Chrona Help")
            .style(Style::default());

        let inner_area = help_block.inner(help_area);
        help_block.render(help_area, buf);

        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(2),
            ])
            .split(inner_area);

        let intro_text = Text::from(vec![
            Line::from(vec![
                Span::styled("Welcome to ", Style::default()),
                Span::styled(
                    "Chrona",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" - Task Management TUI", Style::default()),
            ]),
            Line::from("Press ? to toggle this help screen at any time"),
        ]);

        Paragraph::new(intro_text)
            .alignment(Alignment::Center)
            .render(chunks[0], buf);

        let help_items = [
            (
                "Global",
                vec![
                    ("q, Esc", "Quit the application"),
                    ("?", "Toggle help screen"),
                    ("Tab", "Switch between task lists"),
                    ("Shift+Tab", "Switch between task lists (reverse)"),
                ],
            ),
            (
                "Task Navigation",
                vec![
                    ("j, ↓", "Move selection down"),
                    ("k, ↑", "Move selection up"),
                    ("Space", "Toggle selection/mark task"),
                    ("Home/End", "Jump to top/bottom"),
                ],
            ),
            (
                "Task Management",
                vec![
                    ("a", "Add new task"),
                    ("e", "Edit selected task"),
                    ("d", "Delete selected task"),
                    ("r", "Refresh task list"),
                ],
            ),
            (
                "Form Navigation",
                vec![
                    ("Tab", "Next field"),
                    ("Shift+Tab", "Previous field"),
                    ("Enter", "Submit form"),
                    ("Esc", "Cancel and close form"),
                ],
            ),
        ];

        let mut help_text = Vec::new();
        for (section, items) in help_items {
            help_text.push(Line::from(Span::styled(
                section,
                Style::default().bold().fg(Color::Yellow),
            )));

            for (key, description) in items {
                help_text.push(Line::from(vec![
                    Span::styled(format!("  {:<12}", key), Style::default().fg(Color::Cyan)),
                    Span::styled(description, Style::default()),
                ]));
            }
            help_text.push(Line::default());
        }

        Paragraph::new(Text::from(help_text)).render(chunks[1], buf);

        let footer_text = Text::from(vec![Line::from(vec![
            Span::styled("Press ", Style::default()),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::styled(" to close this help screen", Style::default()),
        ])]);

        Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .render(chunks[2], buf);
    }
}
