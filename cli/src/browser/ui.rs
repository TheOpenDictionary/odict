use super::{models::App, popup::EntryPopup};

use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, List, Paragraph},
    Frame,
};

pub fn render_browser(model: &mut App, frame: &mut Frame) {
    let area = frame.area();

    let [title_area, content_area] = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
        .spacing(1)
        .areas(area);

    let [list_area, search_area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(content_area);

    let width = search_area.width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = model.search_field.visual_scroll(width as usize);

    let title = Paragraph::new(vec![
        Line::from(Span::from("")),
        Line::from(Span::from(&model.title).bold()),
        Line::from(
            Span::from("Press 'q' to quit and arrow keys to navigate")
                .dim()
                .italic(),
        ),
    ]);

    let search_field = Paragraph::new(model.search_field.value())
        .scroll((0, scroll as u16))
        .block(Block::bordered().title("Search"));

    let entry_list = List::new(model.entries.items.clone())
        .block(Block::bordered().title("Entries"))
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_widget(title.centered(), title_area);
    frame.render_widget(search_field, search_area);
    frame.render_stateful_widget(entry_list, list_area, &mut model.entries.state);
    frame.set_cursor_position((
        // Put cursor past the end of the input text
        search_area.x + ((model.search_field.visual_cursor().max(scroll) - scroll) as u16) + 1,
        // Move one line down, from the border to the input line
        search_area.y + 1,
    ));
}

pub fn render_popup(model: &mut super::models::EntryPopup, frame: &mut Frame) {
    let area = frame.area();
    let md = &model.content;
    let popup = EntryPopup::default().content(format!("{}", md));

    frame.render_stateful_widget(popup, area, model);
}
