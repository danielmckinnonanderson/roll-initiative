use ratatui::{Frame, widgets::{Block, Borders}};

use crate::theme::THEME;

pub fn splash_screen(frame: &mut Frame) {
    let theme = THEME.root;

    // Centered title which says "Roll Initiative"
    let splash = Block::default()
        .style(theme)
        .title("Roll Initiative")
        .borders(Borders::ALL);

    frame.render_widget(splash, frame.size());
}

