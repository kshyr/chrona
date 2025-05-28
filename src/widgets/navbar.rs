use crate::{page::PageType, ui::center};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, ToSpan},
    widgets::{Block, BorderType, Borders, Widget},
};
use strum::IntoEnumIterator;

pub struct Navbar {
    active_page: PageType,
}

impl Navbar {
    pub fn new(active_page: PageType) -> Self {
        Navbar { active_page }
    }
}

impl Widget for Navbar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let pages: Vec<(PageType, bool)> = PageType::iter()
            .map(|page| (page, page == self.active_page))
            .collect();

        let areas = Layout::horizontal(
            pages
                .iter()
                .map(|_| Constraint::Length(19))
                .collect::<Vec<Constraint>>(),
        )
        .split(area);

        pages.iter().enumerate().for_each(|(i, (page, active))| {
            let default_style = Style::default().fg(Color::White).dim();
            let active_style = Style::default().fg(Color::Yellow).bold();
            let span_style = if *active { active_style } else { default_style };
            let block_style = if *active { active_style } else { default_style };

            let block = Block::default()
                .borders(Borders::BOTTOM)
                .border_type(if *active {
                    BorderType::Thick
                } else {
                    BorderType::Plain
                })
                .style(block_style);

            let page_name = page.to_string();
            let (key_char, page_name_rest) = page_name.split_at(1);
            let key_char = format!("[{}]", key_char);
            let line = Line::from(vec![
                key_char.to_span().style(span_style).bold(),
                page_name_rest.to_span().style(span_style),
            ]);

            let mut block_area = areas[i];
            block_area.x += 2 * i as u16;
            let line_area = center(
                block.inner(block_area),
                Constraint::Length((line.width() as u16) + 1),
                Constraint::Length(1),
            );
            block.render(block_area, buf);
            line.render(line_area, buf);
        });
    }
}
