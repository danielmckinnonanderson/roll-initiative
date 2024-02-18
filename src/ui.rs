use ratatui::{
    prelude::{Buffer, Layout, Rect},
    widgets::{Block, Borders, Padding, Widget},
    Frame,
};

use crate::{app::AppMode, theme::THEME};

pub fn splash_screen(frame: &mut Frame) {
    let theme = THEME.root;

    // Centered title which says "Roll Initiative"
    let splash = Block::default()
        .style(theme)
        .title("Roll Initiative")
        .borders(Borders::ALL);

    frame.render_widget(splash, frame.size());
}
