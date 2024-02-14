use crossterm::event::KeyCode;

use crate::app::{AppMode, RunMode, AppState};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppCommand {
    Quit,
    NoOp,
}

/// Given the current AppState, induce action and return the resulting AppState.
pub type StateInducer = fn((&mut AppMode, Option<&mut AppState>));

/// This block is the key mappings for the various 'modes' of the app.
#[rustfmt::skip]
impl From<(&AppMode, Option<KeyCode>)> for AppCommand {
    fn from(value: (&AppMode, Option<KeyCode>)) -> Self {
        value.1.map(|key| match value.0 {
                // There are no commands in the initializing or quitting states
                AppMode::Initializing | AppMode::Quitting => AppCommand::NoOp,

                AppMode::Running(run_mode) => match run_mode {
                    RunMode::EditingEncounter => match key {
                        // Vim & arrow key movement
                        | KeyCode::Char('j') 
                        | KeyCode::Down => AppCommand::NoOp,

                        | KeyCode::Char('k')
                        | KeyCode::Up => AppCommand::NoOp,

                        | KeyCode::Char('h')
                        | KeyCode::Left => AppCommand::NoOp,

                        | KeyCode::Char('l')
                        | KeyCode::Right => AppCommand::NoOp,

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
            }).unwrap_or(AppCommand::NoOp)
    }
}

impl From<AppCommand> for StateInducer {
    fn from(value: AppCommand) -> Self {
        match value {
            AppCommand::Quit => {
                |value: (&mut AppMode, Option<&mut AppState>)| {
                    *value.0 = AppMode::Quitting;
                }
            }
            AppCommand::NoOp => {
                // Do nothing
                |_value: (&mut AppMode, Option<&mut AppState>)| {}
            }
        }
    }
}
