use odict::{ArchivedDictionary, DictionaryFile};
use ratatui::widgets::{ListState, ScrollbarState};
use tui_input::Input;

use super::enums::RunningState;

#[derive(Debug, Default)]
pub struct EntryPopup {
    pub content: String,
    pub content_length: usize,
    pub scroll: usize,
    pub scroll_state: ScrollbarState,
}

impl EntryPopup {
    pub fn new(content: String) -> EntryPopup {
        let lines = content.lines().count();
        let md = format!("{}\n", termimad::term_text(content.as_str()));

        EntryPopup {
            content: md,
            content_length: lines,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct EntryList {
    pub items: Vec<String>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct App<'a> {
    pub search_field: Input,
    pub entries: EntryList,
    pub entry: Option<EntryPopup>,
    pub running_state: RunningState,
    pub title: String,
    pub dictionary: &'a ArchivedDictionary,
}

impl App<'_> {
    pub fn new<'a>(dictionary: &'a DictionaryFile) -> color_eyre::Result<App<'a>> {
        let archive = dictionary.to_archive()?;

        let entries = archive
            .lexicon()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut list_state = ListState::default();

        list_state.select_first();

        let model = App {
            search_field: Input::default(),
            entries: EntryList {
                items: entries,
                state: list_state,
            },
            entry: None,
            running_state: RunningState::Running,
            title: archive
                .name
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or(dictionary.path.as_ref().unwrap().to_str().unwrap())
                .to_string(),
            dictionary: archive,
        };

        Ok(model)
    }
}
