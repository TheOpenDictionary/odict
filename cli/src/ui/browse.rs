use color_eyre::eyre::Result;

use odict::{find::FindOptions, ArchivedDictionary};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    init,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListState, Padding, Paragraph},
    DefaultTerminal, Frame,
};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::CLIContext;

/// App holds the state of the application
struct App<'a> {
    input: Input,
    title: String,
    dictionary: &'a ArchivedDictionary,
    terms: Vec<&'a str>,
}

pub fn browse(ctx: &CLIContext, path_or_alias: &str) -> Result<()> {
    let file = ctx
        .reader
        .read_from_path_or_alias_with_manager(&path_or_alias, &ctx.alias_manager)?;

    let archive = file.to_archive()?;

    let mut app = App {
        input: Input::default(),
        title: archive
            .name
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(file.path.as_ref().unwrap().to_str().unwrap())
            .to_string(),
        dictionary: archive,
        terms: archive.lexicon(),
    };

    let terminal = init();
    let result = run(terminal, &mut app);

    ratatui::restore();

    result
}

/// Run the application.
fn run<'a>(mut terminal: DefaultTerminal, app: &mut App<'a>) -> Result<()> {
    let mut list_state = ListState::default();

    list_state.select_first();

    loop {
        terminal.draw(|frame| draw(frame, app, &mut list_state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break Ok(()),
                KeyCode::Down => list_state.select_next(),
                KeyCode::Up => list_state.select_previous(),
                _ => {
                    app.input.handle_event(&Event::Key(key));

                    let terms = app
                        .dictionary
                        .find_terms(app.input.value(), FindOptions::default());

                    app.terms = terms;

                    list_state.select_first();
                }
            }
        }
    }
}

fn draw<'a>(frame: &mut Frame, app: &App<'a>, list_state: &mut ListState) {
    let container = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
        .spacing(1)
        .split(frame.area());

    let inner = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).split(container[1]);
    let width = inner[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.input.visual_scroll(width as usize);

    let input_field = Paragraph::new(app.input.value())
        .scroll((0, scroll as u16))
        .block(Block::bordered().title("Search"));

    let terms = &app.terms;

    let list = List::new(terms.clone())
        .block(Block::bordered().title("Entries"))
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    let title = Paragraph::new(vec![
        Line::from(Span::from("")),
        Line::from(Span::from(&app.title).bold()),
        Line::from(
            Span::from("Press 'q' to quit and arrow keys to navigate")
                .dim()
                .italic(),
        ),
    ]);

    frame.render_widget(title.centered(), container[0]);
    frame.render_widget(input_field, inner[1]);
    frame.render_stateful_widget(list, inner[0], list_state);
    frame.set_cursor_position((
        // Put cursor past the end of the input text
        inner[1].x + ((app.input.visual_cursor().max(scroll) - scroll) as u16) + 1,
        // Move one line down, from the border to the input line
        inner[1].y + 1,
    ));
}
