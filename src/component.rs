use anyhow::Result;
use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{prelude::*, Frame, widgets::*};

pub trait Component : WidgetRef {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent);

    fn update(&mut self);
}


pub struct Main;


impl WidgetRef for Main {
    fn render_ref(&self, area:Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::all())
            .title("Hello!")
            .render_ref(area, buf);
    }
}

impl Component for Main {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('a') => {

            },
            KeyCode::Char('q') => {

            },
            _ => {

            }
        }
    }

    fn update(&mut self) {
        
    }
}

