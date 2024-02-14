use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum AppMode {
    Initializing,
    Running(RunMode),
    Quitting,
}

impl AppMode {
    pub fn new() -> Self {
        AppMode::Initializing
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

impl Participant {
    pub fn new(
        name: String,
        hp_current: u16,
        hp_max: u16,
        initiative_rolls: [Option<u8>; 8],
    ) -> Self {
        Participant {
            name,
            hp_max,
            hp_current,
            initiative_rolls,
        }
    }
}
