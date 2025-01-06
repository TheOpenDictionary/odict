use std::time::Duration;

use odict::{find::FindOptions, format::md::ToMarkdown, DictionaryFile};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    Frame,
};
use tui_input::backend::crossterm::EventHandler;

use super::{
    enums::{Message, RunningState},
    models::{App, EntryPopup},
    ui::{render_browser, render_popup},
};

pub fn launch_browser(file: &DictionaryFile) -> color_eyre::Result<()> {
    super::tui::install_panic_hook();

    let mut terminal = super::tui::init_terminal()?;
    let mut model = App::new(file)?;

    while model.running_state != RunningState::Done {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    super::tui::restore_terminal()?;

    Ok(())
}

fn view(model: &mut App, frame: &mut Frame) {
    if let Some(popup) = &mut model.entry {
        render_popup(popup, frame);
    } else {
        render_browser(model, frame);
    }
}

fn handle_event(_: &App) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(Some(handle_key(key)));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Message {
    match key.code {
        KeyCode::Esc => Message::Back,
        KeyCode::Down => Message::ScrollDown,
        KeyCode::Up => Message::ScrollUp,
        KeyCode::Enter => Message::Lookup,
        _ => Message::Input(key),
    }
}

fn update(model: &mut App, msg: Message) -> Option<Message> {
    match msg {
        Message::Back => {
            if model.entry.is_none() {
                model.running_state = RunningState::Done;
            } else {
                model.entry = None;
            }
        }
        Message::Lookup => {
            let term = model.entries.items.get(model.entries.state.selected()?)?;
            let entry = model.dictionary.entries.get(term.as_str()).unwrap();

            model.entry = Some(EntryPopup::new(entry.to_markdown().unwrap()))
        }
        Message::ScrollDown => {
            if let Some(popup) = &mut model.entry {
                popup.scroll = popup
                    .scroll
                    .saturating_add(1)
                    .min(popup.content_length.saturating_sub(1));
                popup.scroll_state = popup.scroll_state.position(popup.scroll);
            } else {
                model.entries.state.select_next();
            }
        }
        Message::ScrollUp => {
            if let Some(popup) = &mut model.entry {
                popup.scroll = popup.scroll.saturating_sub(1);
                popup.scroll_state = popup.scroll_state.position(popup.scroll);
            } else {
                model.entries.state.select_previous();
            }
        }
        Message::Filter => {
            let terms = model
                .dictionary
                .find_terms(model.search_field.value(), FindOptions::default());

            model.entries.items = terms.into_iter().map(|s| s.to_string()).collect();
            model.entries.state.select_first();
        }
        Message::Input(key) => {
            model.search_field.handle_event(&Event::Key(key));
            return Some(Message::Filter);
        }
    };
    None
}
