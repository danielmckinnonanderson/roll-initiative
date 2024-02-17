use std::str::FromStr;

use anyhow::{Result, Context};
use crossterm::event::{KeyCode, KeyEventKind, Event};
use ratatui::{prelude::Backend, text::Text, style::{Style, Color}};

use crate::{FRAMES_PER_SECOND, commands::{AppCommand, StateInducer}, ui::splash_screen};


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppMode {
    Initializing(InitializeState),
    Running(RunMode),
    Quitting(QuittingState),
}

impl Default for AppMode {
    fn default() -> Self {
        AppMode::Initializing(InitializeState::default())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializeState {
    pub splash_screen_frames_remaining: u32,
}

impl Default for InitializeState {
    fn default() -> Self {
        InitializeState {
            splash_screen_frames_remaining: (FRAMES_PER_SECOND * 2) as u32,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuittingState {
    pub quitting_screen_frames_remaining: u32,
}

impl Default for QuittingState {
    fn default() -> Self {
        QuittingState {
            quitting_screen_frames_remaining: (FRAMES_PER_SECOND * 2) as u32,
        }
    }
}

impl AppMode {
    /// Update function. Poll for inputs from the user, apply the command (if any), and return the
    /// next state. Returns `None` if the application should quit.
    pub fn next_state(&self) -> Result<Option<AppMode>> {
        match self {
            AppMode::Initializing(state) => {
                // TODO - Show the splash screen for a certain amount of frames before switching
                // state to Running
                // We're inside of our main loop now, so we're done initializing.
                // Update the state to reflect that and then continue to next frame.
                if state.splash_screen_frames_remaining != 0 {
                    Ok(Some(AppMode::Initializing(InitializeState {
                        splash_screen_frames_remaining: state.splash_screen_frames_remaining - 1,
                    })))
                } else {
                    // We're done initializing, move to the next state.
                    // (For now, we're just going to move to the next state, but in the future
                    // we might want to do some cleanup here.)
                    Ok(Some(AppMode::Running(RunMode::EditingEncounter(vec![]))))
                }
            },
            AppMode::Running(_run_mode) => {
                // Check for keypress events.
                let key_opt = poll_for_keypress().context("Couldn't poll for keypress.")?;

                // // Map keypress to command based on current mode & pressed key.
                let command: AppCommand = (self, key_opt).into();

                // // If the pressed key corresponds to a command, run command against state.
                let next = StateInducer::from(command)(self);

                // // TODO - Other stuff while running.
                Ok(Some(next))
            },
            AppMode::Quitting(quitting_state) => {
                // If our state says we're quitting, we're gonna quit. Break out of
                // the outer loop and beginning cleaning up after ourselves.
                if quitting_state.quitting_screen_frames_remaining != 0 {
                    Ok(Some(AppMode::Quitting(QuittingState {
                        quitting_screen_frames_remaining: quitting_state.quitting_screen_frames_remaining - 1,
                    })))
                } else {
                    Ok(None)
                }
            },
        }
    }

    pub fn draw(&self, terminal: &mut crate::Terminal<impl Backend>) -> Result<()> {
        match self {
            AppMode::Initializing(_state) => {
                terminal.draw(|frame| {
                    splash_screen(frame);
                })?;

                Ok(())
            },
            AppMode::Running(_run_mode) => {
                Ok(())
            },
            AppMode::Quitting(_quitting_state) => {
                terminal.draw(|frame| {
                    let style = Style::default()
                        .fg(Color::LightMagenta)
                        .bg(Color::Black);

                    frame.render_widget(
                        Text::styled("Bye for now!", style),
                        frame.size()
                    );
                })?;

                Ok(())
            }
        }
    }
}

fn poll_for_keypress() -> Result<Option<KeyCode>> {
    if let Event::Key(key) = crossterm::event::read().context("Could not read event.")? {
        if key.kind == KeyEventKind::Press {
            Ok(Some(key.code))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RunMode {
    EditingEncounter(Vec<Participant>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppState {
    mode: AppMode,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            mode: AppMode::Initializing(InitializeState::default()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Participant {
    name: String,
    initiative_rolls: [Option<u8>; 4],
}

impl Participant {
    pub fn new(name: &str) -> Self {
        Participant {
            name: String::from(name),
            initiative_rolls: [None; 4],
        }
    }
}

impl FromStr for Participant {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Participant {
            name: s.to_string(),
            initiative_rolls: [None; 4],
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatTurn {
    name: String,
    initiative_roll_value: u8,
    unconscious: bool,
    dead: bool,
}

impl From<Participant> for Vec<CombatTurn> {
    fn from(value: Participant) -> Self {
        let mut turns = Vec::new();
        for roll in value.initiative_rolls.iter().filter_map(|x| *x) {
            turns.push(CombatTurn {
                name: value.name.clone(),
                initiative_roll_value: roll,
                unconscious: false,
                dead: false,
            });
        }
        turns
    }
}

pub fn participants_to_ordered_combat_turns(participants: Vec<Participant>) -> Vec<CombatTurn> {
    let mut result = participants.into_iter().fold(Vec::new(), |mut acc: Vec<CombatTurn>, p: Participant| {
        let turns: Vec<CombatTurn> = p.into();
        acc.extend(turns);
        acc
    });

    result.sort_by(|a, b| b.initiative_roll_value.cmp(&a.initiative_roll_value));

    result
}

mod tests {
    use super::*;

    #[test]
    fn test_participant_constructor() {
        let result = Participant::new("Goku");

        assert_eq!(result.name, "Goku");
        assert_eq!(result.initiative_rolls, [None; 4]);
    }

    #[test]
    fn test_single_participant_to_combat_turns() {
        let participant = Participant {
            name: "Gandalf".to_string(),
            initiative_rolls: [Some(1), Some(2), Some(3), Some(4)],
        };

        let turns: Vec<CombatTurn> = participant.into();

        assert_eq!(turns.len(), 4);
        assert_eq!(turns[0].name, "Gandalf");
        assert_eq!(turns[0].initiative_roll_value, 1);
        assert_eq!(turns[1].name, "Gandalf");
        assert_eq!(turns[1].initiative_roll_value, 2);
        assert_eq!(turns[2].name, "Gandalf");
        assert_eq!(turns[2].initiative_roll_value, 3);
        assert_eq!(turns[3].name, "Gandalf");
        assert_eq!(turns[3].initiative_roll_value, 4);
    }

    #[test]
    fn test_multiple_participants_to_combat_turns_sorted() {
        let gandalf = Participant {
            name: "Gandalf".to_string(),
            initiative_rolls: [Some(20), Some(17), None, None],
        };

        let legolas = Participant {
            name: "Legolas".to_string(),
            initiative_rolls: [Some(15), None, None, None],
        };

        let gimli = Participant {
            name: "Gimli".to_string(),
            initiative_rolls: [Some(7), None, None, None],
        };

        let balrog = Participant {
            name: "Balrog".to_string(),
            initiative_rolls: [Some(25), Some(22), None, None],
        };

        let result = participants_to_ordered_combat_turns(vec![gandalf, legolas, gimli, balrog]);

        assert_eq!(result.len(), 6);

        assert_eq!(result[0], CombatTurn {
            name: "Balrog".to_string(),
            initiative_roll_value: 25,
            unconscious: false,
            dead: false
        });
        assert_eq!(result[1], CombatTurn { name: "Balrog".to_string(),
            initiative_roll_value: 22,
            unconscious: false,
            dead: false
        });
        assert_eq!(result[2], CombatTurn {
            name: "Gandalf".to_string(),
            initiative_roll_value: 20,
            unconscious: false,
            dead: false
        });
        assert_eq!(result[3], CombatTurn {
            name: "Gandalf".to_string(),
            initiative_roll_value: 17,
            unconscious: false,
            dead: false
        });
        assert_eq!(result[4], CombatTurn {
            name: "Legolas".to_string(),
            initiative_roll_value: 15,
            unconscious: false,
            dead: false
        });
        assert_eq!(result[5], CombatTurn {
            name: "Gimli".to_string(),
            initiative_roll_value: 7,
            unconscious: false,
            dead: false
        });
    }

    #[test]
    fn test_app_mode_initial_state() {
        let app = AppMode::default();

        let result = app.next_state().unwrap();

        assert_eq!(result, Some(AppMode::Initializing(InitializeState {
            splash_screen_frames_remaining: (FRAMES_PER_SECOND * 2) as u32 - 1,
        })));
    }

    #[test]
    fn test_app_mode_initial_state_to_running() {
        let app = AppMode::Initializing(InitializeState {
            splash_screen_frames_remaining: 0,
        });

        let result = app.next_state().unwrap();

        assert_eq!(result, Some(AppMode::Running(RunMode::EditingEncounter(vec![]))));
    }

    #[test]
    fn test_app_mode_running_state_decrements_per_frame() {
        let app = AppMode::Quitting(QuittingState {
            quitting_screen_frames_remaining: 1,
        });

        let result = app.next_state().unwrap();

        assert_eq!(result, Some(AppMode::Quitting(QuittingState {
            quitting_screen_frames_remaining: 0,
        })));
    }

    #[test]
    fn test_app_mode_quitting_state_to_none() {
        let app = AppMode::Quitting(QuittingState {
            quitting_screen_frames_remaining: 0,
        });

        let result = app.next_state().unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn test_quitting_command_induce_state_change() {
        let app = AppMode::Running(RunMode::EditingEncounter(vec![]));

        let func = StateInducer::from(AppCommand::Quit);
        let result = func(&app);

        assert_eq!(result, AppMode::Quitting(QuittingState {
            quitting_screen_frames_remaining: (FRAMES_PER_SECOND * 2) as u32,
        }));
    }
}

