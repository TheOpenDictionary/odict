use ratatui::crossterm;

#[derive(PartialEq)]
pub enum Message {
    Lookup,
    Input(crossterm::event::KeyEvent),
    Filter,
    Back,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
