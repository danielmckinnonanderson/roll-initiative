use std::collections::HashMap;

use anyhow::Result;
use ratatui::{prelude::CrosstermBackend, Terminal, Frame};

use crate::ui::splash_screen;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum AppMode {
    #[default]
    Initializing,
    Running(RunMode),
    Quitting,
}

impl AppMode {
    pub fn draw(
        &self, 
        app_state: &AppState,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>
    ) -> Result<()> {
        match self {
            AppMode::Initializing => {
                // Draw the TUI for the current frame
                terminal.draw(|frame: &mut Frame| {
                    splash_screen(frame);
                })?;
            },
            AppMode::Running(_run_mode) => {
                
            }
            AppMode::Quitting => {
                // Print goodbye message
            }
        }

        Ok(())
    }
}



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RunMode {
    EditingEncounter,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppState {
    participants: HashMap<String, Participant>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Participant {
    name: String,
    hp_max: u16,
    hp_current: u16,
    initiative_rolls: [Option<u8>; 8],
}

impl Default for Participant {
    fn default() -> Self {
        Participant {
            name: String::new(),
            hp_max: 0,
            hp_current: 0,
            initiative_rolls: [None; 8],
        }
    }
}

