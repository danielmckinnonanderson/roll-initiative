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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Participant {
    name: String,
    hp_max: u16,
    hp_current: u16,
    initiative_rolls: Vec<u16>,
}

impl Participant {
    pub fn new(name: String, hp_current: u16, hp_max: u16, initiative_rolls: Vec<u16>) -> Self {
        Participant {
            name,
            hp_max,
            hp_current,
            initiative_rolls,
        }
    }
}
