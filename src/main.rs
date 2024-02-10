use std::{time::{ Duration, Instant }, default};

use anyhow::{Result, Context};
use app::AppState;
use commands::{AppCommand, StateInducer};
use crossterm::event::{KeyCode, Event, KeyEventKind};
use lazy_static::lazy_static;
use ratatui::{Terminal, prelude::CrosstermBackend, Frame, widgets::Block};

use crate::component::Main;

mod app;
mod commands;
mod component;

pub const FRAMES_PER_SECOND: u64 = 24;

lazy_static! {
    pub static ref FRAME_WAIT_DURATION: Duration = {
        let frame_wait_millis: u64 = (1000 / (FRAMES_PER_SECOND * 1000)).into();
        Duration::from_millis(frame_wait_millis)
    };
}

fn main() -> Result<()> {
    // Define application state
    let mut state = AppState::Initializing;

    // Enable raw mode
    crossterm::terminal::enable_raw_mode().context("Failed to enable raw mode.")?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)
        .context("Failed to enter alternate screen.")?;

    // Initialize the terminal backend
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))
        .context("Failed to start new Terminal with CrosstermBackend.")?;

    let mut last_frame_time = Instant::now();

    while state != AppState::Quitting {
        // Draw the TUI for the current frame
        terminal.draw(|frame: &mut Frame| {
            // frame.render_widget_ref(main, frame.size())
            frame.render_widget(Block::default().title("Yo"), frame.size());
        })?;

        // Check for keypress events.
        let key_opt = poll_for_keypress().context("Couldn't poll for keypress.").unwrap();
        // Map keypress to command.
        let cmd = match key_opt {
            Some(key) => AppCommand::from(key),
            None => AppCommand::NoOp,
        };

        // If the pressed key corresponds to a command, run command against state.
        state = StateInducer::from(cmd)(state);

        // Calculate the time elapsed since the last frame.
        let elapsed = Instant::now().duration_since(last_frame_time);

        // Update the last frame time to now (frame `n`), since we have 
        //  finished using the frame time of the previous frame (`n-1`).
        last_frame_time = std::time::Instant::now();

        // Sleep until next iteration (`n+1`)
        if elapsed < *FRAME_WAIT_DURATION {
            std::thread::sleep(*FRAME_WAIT_DURATION - elapsed);
        }
    }

    // Broke out of loop because AppState was 'Quitting', initiate shutdown.
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

fn poll_for_keypress() -> Result<Option<KeyCode>> {
    if let Event::Key(key) = crossterm::event::read().context("Could not read event.")? {
        if key.kind == KeyEventKind::Press {
            Ok(Some(key.code))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
