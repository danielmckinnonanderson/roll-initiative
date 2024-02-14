use crossterm::event::KeyCode;

use crate::app::{AppMode, RunMode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppCommand {
    Quit,
    NoOp
}

// FIXME - Update this type & related funcs to reflect operations on
//         an a State struct AND mode, instead of just changing the mode.
/// Given the current AppState, induce action and return the resulting AppState.
pub type StateInducer = fn (AppMode) -> AppMode;

/// This block is the key mappings for the various 'modes' of the app.
impl From<(&AppMode, Option<KeyCode>)> for AppCommand {
    fn from(value: (&AppMode, Option<KeyCode>)) -> Self {
        value.1.and_then(|key| {
            Some(match value.0 {
                // There are no commands in the initializing or quitting states
                AppMode::Initializing | AppMode::Quitting => AppCommand::NoOp,

                AppMode::Running(run_mode) => match run_mode {
                    RunMode::EditingEncounter => match key {
                        // Vim movement
                        KeyCode::Char('j') => AppCommand::NoOp,
                        KeyCode::Char('k') => AppCommand::NoOp,
                        KeyCode::Char('h') => AppCommand::NoOp,
                        KeyCode::Char('l') => AppCommand::NoOp,

                        // Edit the current participant row
                        | KeyCode::Enter
                        | KeyCode::Char('e') => AppCommand::NoOp,

                        // Add a new participant row
                        KeyCode::Char('a') => AppCommand::NoOp,

                        // Remove the current participant row
                        | KeyCode::Char('d')
                        | KeyCode::Backspace
                        | KeyCode::Delete => AppCommand::NoOp,

                        // Quit the app
                        KeyCode::Char('q') => AppCommand::Quit,

                        _ => AppCommand::NoOp,
                    }
                },
            })
        }).unwrap_or(AppCommand::NoOp)
    }
}

impl From<AppCommand> for StateInducer {
    fn from(value: AppCommand) -> Self {
        match value {
            AppCommand::Quit => {
                |_state: AppMode| AppMode::Quitting
            },
            AppCommand::NoOp => {
                |state: AppMode| state.clone()
            },
        }
    }
}


