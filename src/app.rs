use std::collections::HashMap;

#[derive(PartialEq)]
pub enum AppState {
    Initializing,
    EditingParticipants(Vec<Participant>),
    RunningCombat(HashMap<String, Participant>),
    Quitting,
}

impl AppState {
    pub fn new() -> Self {
        AppState::Initializing
    }

    // NOTE - Revisit removing this entirely
    pub fn transition(&mut self) {
        *self = match self {
            AppState::Initializing => {
                AppState::EditingParticipants(Vec::new())
            },
            AppState::EditingParticipants(participants) => {
                // Collect participants into a map
                let combatants: HashMap<String, Participant> = participants.iter().fold(HashMap::new(), |mut acc, item| {
                    acc.insert(item.name.clone(), item.clone());
                    acc
                });
                AppState::RunningCombat(combatants)
            },
            AppState::RunningCombat(_) => AppState::Quitting,
            AppState::Quitting => AppState::Quitting,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
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
