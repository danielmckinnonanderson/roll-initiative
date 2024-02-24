use core::fmt;
use std::fmt::{Display, Formatter};

use crossterm::event::KeyCode;

use crate::app::{AppMode, QuittingState, RunMode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppCommand {
    Quit,
    NoOp,
}

/// This block determines how commands are presented in words.
/// Used in the help text for the key bindings.
impl Display for AppCommand {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AppCommand::Quit => write!(f, "Quit"),
            AppCommand::NoOp => Ok(()),
        }
    }
}


/// Given the current AppState, induce action and return the resulting AppState.
pub type StateInducer = fn(&AppMode) -> AppMode;

/// This block is the key mappings for the various 'modes' of the app.
#[rustfmt::skip]
impl From<(&AppMode, Option<KeyCode>)> for AppCommand {
    fn from(value: (&AppMode, Option<KeyCode>)) -> Self {
        value.1.map(|key| match value.0 {
                // There are no commands in the initializing or quitting states
                AppMode::Initializing(_) | AppMode::Quitting(_) => AppCommand::NoOp,

                AppMode::Running(run_mode) => match run_mode {
                    RunMode::EditingEncounter(_state) => match key {
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
            AppCommand::Quit => |state: &AppMode| match state {
                AppMode::Quitting(quitting_state) => AppMode::Quitting(quitting_state.clone()),
                _ => AppMode::Quitting(QuittingState::default()),
            },
            AppCommand::NoOp => {
                // Do nothing
                |state: &AppMode| state.clone()
            }
        }
    }
}

mod test {
    use super::*;
    use crate::app::*;

    #[test]
    fn test_app_command_from() {
        let app_mode =
            AppMode::Running(RunMode::EditingEncounter(EditingEncounterState::default()));
        let key = Some(KeyCode::Char('q'));
        let app_command = AppCommand::from((&app_mode, key));
        assert_eq!(app_command, AppCommand::Quit);
    }

    #[test]
    fn test_app_command_from_none() {
        let app_mode =
            AppMode::Running(RunMode::EditingEncounter(EditingEncounterState::default()));
        let key = None;
        let app_command = AppCommand::from((&app_mode, key));
        assert_eq!(app_command, AppCommand::NoOp);
    }

    #[test]
    fn test_state_inducer_from() {
        let app_command = AppCommand::Quit;
        let state_inducer = StateInducer::from(app_command);
        let app_mode =
            AppMode::Running(RunMode::EditingEncounter(EditingEncounterState::default()));
        let new_app_mode = state_inducer(&app_mode);
        assert_eq!(new_app_mode, AppMode::Quitting(QuittingState::default()));
    }

    #[test]
    fn test_noop_induces_no_change() {
        let app_command = AppCommand::NoOp;
        let state_inducer = StateInducer::from(app_command);
        let app_mode =
            AppMode::Running(RunMode::EditingEncounter(EditingEncounterState::default()));
        let new_app_mode = state_inducer(&app_mode);
        assert_eq!(new_app_mode, app_mode);
    }

    #[test]
    fn test_quit_induces_quitting() {
        let app_command = AppCommand::Quit;
        let state_inducer = StateInducer::from(app_command);
        let app_mode =
            AppMode::Running(RunMode::EditingEncounter(EditingEncounterState::default()));
        let new_app_mode = state_inducer(&app_mode);
        assert_eq!(new_app_mode, AppMode::Quitting(QuittingState::default()));
    }
}
