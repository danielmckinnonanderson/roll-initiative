use ratatui::style::{Color, Style};

mod colors {
    use super::*;

    const BLACK: Color = Color::Rgb(0, 0, 0);
    const GRANITE: Color = Color::Rgb(17, 17, 17);
    const CHARCOAL: Color = Color::Rgb(34, 34, 34);
    const SLATE: Color = Color::Rgb(68, 68, 68);

    const BRUISE: Color = Color::Rgb(20, 11, 22);
    const EGGPLANT: Color = Color::Rgb(36, 17, 38);

    const LIGHT_GRAY: Color = Color::Rgb(211, 211, 211);
    const WHITE: Color = Color::Rgb(255, 255, 255);

    const LAVENDER: Color = Color::Rgb(230, 230, 250);
    const LILAC: Color = Color::Rgb(200, 162, 200);

    const STRAW: Color = Color::Rgb(255, 204, 102);
    const AMBER: Color = Color::Rgb(255, 191, 0);

    // Theme palette
    pub const BG_DARKEST: Color = BLACK;
    pub const BG_DARKER: Color = GRANITE;
    pub const BG_DARK: Color = CHARCOAL;
    pub const BG_DARKISH: Color = SLATE;
    pub const BG_DARK_IMPORTANT: Color = BRUISE;

    pub const BG_LIGHT: Color = LAVENDER;
    pub const BG_LIGHTEST: Color = LILAC;

    pub const TEXT_BODY: Color = LIGHT_GRAY;
    pub const TEXT_BODY_SUBTLE: Color = SLATE;
    pub const TEXT_IMPORTANT: Color = WHITE;
    pub const TEXT_HEADING: Color = AMBER;
    pub const TEXT_TITLE: Color = STRAW;

    // Light refers to light-background
    pub const TEXT_BODY_LIGHT: Color = GRANITE;
    pub const TEXT_IMPORTANT_LIGHT: Color = BLACK;
}

pub struct Theme {
    pub root: Style,
    pub app_title: Style,

    pub tabs: Style,
    pub tab_selected: Style,

    pub edit_participants_tab: EditParticipantsTabTheme,

    pub controls_panel: ControlsPanelTheme,
}

pub struct ControlsPanelTheme {
    pub bg: Style,
    pub text: Style,
    pub delimiter: Style,
    pub border_color: Color
}

pub struct EditParticipantsTabTheme {
    pub root: Style,
    pub column_heading: Style,
    pub participant_row: Style,
    pub participant_row_alt: Style,
    pub participant_row_focused: Style,
    pub participant_col_focused: Style,
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
        root: Style::new().fg(colors::TEXT_BODY),
        app_title: Style::new().fg(colors::TEXT_TITLE),

        tabs: Style::new().fg(colors::TEXT_BODY_SUBTLE),
        tab_selected: Style::new().fg(colors::TEXT_IMPORTANT).bg(colors::BG_DARKISH),

        controls_panel: ControlsPanelTheme {
            bg: Style::new().fg(colors::TEXT_BODY).bg(colors::BG_DARKISH),
            text: Style::new().fg(colors::TEXT_BODY),
            delimiter: Style::new().fg(colors::TEXT_BODY_SUBTLE),
            border_color: colors::BG_DARK_IMPORTANT,
        },
        edit_participants_tab: EditParticipantsTabTheme {
            root: Style::new().fg(colors::TEXT_BODY),
            column_heading: Style::new().fg(colors::TEXT_HEADING).bg(colors::BG_DARK_IMPORTANT),
            participant_row: Style::new().fg(colors::TEXT_BODY),
            participant_row_alt: Style::new().fg(colors::TEXT_IMPORTANT),
            participant_row_focused: Style::new().fg(colors::TEXT_BODY_LIGHT).bg(colors::BG_LIGHT),
            participant_col_focused: Style::new().fg(colors::TEXT_IMPORTANT_LIGHT).bg(colors::BG_LIGHTEST),
        },
    }
};
