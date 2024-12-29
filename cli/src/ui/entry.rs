use derive_setters::Setters;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

#[derive(Debug, Default, Setters)]
pub struct EntryPopup<'a> {
    #[setters(into)]
    content: Text<'a>,
}

impl Widget for EntryPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);

        let block = Block::new().borders(Borders::ALL);

        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .block(block)
            .scroll((0 as u16, 0))
            .render(area, buf);
    }
}
