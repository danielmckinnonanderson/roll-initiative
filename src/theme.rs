use ratatui::style::{Color, Style};

// Palette
pub const BLACK: Color = Color::Rgb(0, 0, 0);
pub const LIGHT_GRAY: Color = Color::Rgb(211, 211, 211);
pub const WHITE: Color = Color::Rgb(255, 255, 255);

pub const LIGHT_PURPLE: Color = Color::Rgb(255, 0, 255);
pub const DEEP_PURPLE: Color = Color::Rgb(128, 0, 128);

pub struct Theme {
    pub root: Style,
    pub content: Style,
    pub app_title: Style,
    pub tabs: Style,
    pub tabs_selected: Style,
    pub borders: Style,
    pub description: Style,
    pub description_title: Style,
    pub edit_participants_tab: EditParticipantsTabTheme,
    // pub running_combat_tab: RunningCombatTabTheme,
}

pub struct EditParticipantsTabTheme {
    pub root: Style,
    pub participant: Style,
    pub participant_focused: Style,
}

// TODO - Come back to this after edit participants tab works
#[allow(dead_code)]
pub struct RunningCombatTabTheme {
    pub root: Style,
    pub participant: Style,
    pub participant_focused: Style,
}

pub const THEME: Theme = {
    Theme {
        root: Style::new().fg(LIGHT_GRAY).bg(BLACK),
        content: Style::new(),
        app_title: Style::new(),
        tabs: Style::new(),
        tabs_selected: Style::new(),
        borders: Style::new(),
        description: Style::new(),
        description_title: Style::new(),
        edit_participants_tab: EditParticipantsTabTheme {
            root: Style::new(),
            participant: Style::new(),
            participant_focused: Style::new().fg(LIGHT_PURPLE),
        },
        // running_combat_tab: RunningCombatTabTheme {
        //     root: Style::new(),
        //     participant: Style::new(),
        //     participant_focused: Style::new(),
        // }
    }
};
