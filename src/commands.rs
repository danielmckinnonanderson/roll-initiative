use crossterm::event::KeyCode;

use crate::app::AppState;

#[derive(Debug, PartialEq, Eq)]
pub enum AppCommand {
    Quit,
    NoOp
}

/// Given the current AppState, induce action and return the resulting AppState.
pub type StateInducer = fn (AppState) -> AppState;

impl From<KeyCode> for AppCommand {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Char('q') => AppCommand::Quit,
            _ => AppCommand::NoOp,
        }
    }
}

impl From<AppCommand> for StateInducer {
    fn from(value: AppCommand) -> Self {
        match value {
            AppCommand::Quit => {
                |_state: AppState| AppState::Quitting
            },
            AppCommand::NoOp => {
                |state: AppState| state
            },
        }
    }
}


