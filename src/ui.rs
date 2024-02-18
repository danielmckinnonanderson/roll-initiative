use ratatui::{
    prelude::{Buffer, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Padding, Widget},
    Frame,
};

use crate::{
    app::{AppMode, EditingEncounterState},
    theme::THEME,
};

pub mod elements {
    use ratatui::{
        style::{Color, Style},
        text::Line,
        widgets::{BorderType, Paragraph},
    };

    use crate::{
        app::{AppState, RunMode},
        theme,
    };

    use super::*;

    pub const EDITING_PARTICIPANTS_CONTROLS_TEXT: &str = "[a] Add | [d (or) Del.] Delete 
        | [e (or) Enter] Toggle editing 
        | [q] Quit | [tab] Change tab | [↑ (or) k] Prev. row 
        | [↓ (or) j] Next row | [← (or) h] Prev. column | [→ (or) l] Next column ";

    pub fn splash_screen(frame: &mut Frame) {
        let theme = THEME.root;

        // Centered title which says "Roll Initiative"
        let splash = Block::default()
            .style(theme)
            .title("Roll Initiative")
            .borders(Borders::ALL);

        let area = centered_rect(50, 50, frame.size());

        frame.render_widget(splash, area);
    }

    pub fn editing_encounter_tab(frame: &mut Frame, state: &EditingEncounterState) {
        let theme = THEME;

        let block = Block::default()
            .style(theme.edit_participants_tab.root)
            .borders(Borders::ALL)
            .title_style(theme.edit_participants_tab.root);

        frame.render_widget(block, frame.size());
    }

    pub fn controls_panel(frame: &mut Frame, run_mode: &RunMode, area: Rect) {
        let theme = THEME;

        let controls_text = match run_mode {
            RunMode::EditingEncounter(_) => EDITING_PARTICIPANTS_CONTROLS_TEXT,
            _ => "",
        };

        let panel = Paragraph::new(Line::from(controls_text))
            .style(Style::new().fg(Color::White))
            .centered()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(theme.controls_panel.border_color))
                    .border_type(BorderType::Double),
            );

        frame.render_widget(panel, area);
    }
}

// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
