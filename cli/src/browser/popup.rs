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

use crate::print;

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

        let lines = self.content.lines.len();

        let paragraph = Paragraph::new(self.content)
            .scroll((state.scroll as u16, 0))
            .block(Block::new().borders(Borders::ALL));

        state.content_length = lines.saturating_sub((area.height as usize).saturating_sub(4));
        state.scroll_state = state.scroll_state.content_length(state.content_length);

        paragraph.render(area, buf);
        scrollbar.render(area, buf, &mut state.scroll_state);
    }

    type State = super::models::EntryPopup;
}
