use derive_setters::Setters;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
        Widget,
    },
};

#[derive(Debug, Default, Setters)]
pub struct EntryPopup<'a> {
    #[setters(into)]
    content: Text<'a>,
}

impl StatefulWidget for EntryPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let text = Text::from(self.content);
        let lines = text.lines.len().clone();
        let paragraph = Paragraph::new(text)
            .scroll((state.scroll as u16, 0))
            .block(Block::new().borders(Borders::ALL));

        state.scroll_state = state.scroll_state.content_length(lines);

        paragraph.render(area, buf);
        scrollbar.render(area, buf, &mut state.scroll_state);
    }

    type State = super::models::EntryPopup;
}
