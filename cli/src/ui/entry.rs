use derive_setters::Setters;

use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    crossterm::{
        event::{self, Event},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
    Frame, Terminal,
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
