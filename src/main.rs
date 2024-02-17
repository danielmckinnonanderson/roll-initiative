use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use app::AppMode;
use lazy_static::lazy_static;
use ratatui::{prelude::CrosstermBackend, widgets::Block, Frame, Terminal, text::{Text, Span}};

mod app;
mod commands;
mod theme;
mod ui;

pub const FRAMES_PER_SECOND: u64 = 24;

lazy_static! {
    pub static ref FRAME_WAIT_DURATION: Duration = {
        let frame_wait_millis: u64 = 1000 / FRAMES_PER_SECOND;
        Duration::from_millis(frame_wait_millis)
    };
}

fn main() -> Result<()> {
    // Define application state
    let mut mode = AppMode::default();

    // Enable raw mode
    crossterm::terminal::enable_raw_mode().context("Failed to enable raw mode.")?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)
        .context("Failed to enter alternate screen.")?;

    // Initialize the terminal backend
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))
        .context("Failed to start new Terminal with CrosstermBackend.")?;

    let mut last_frame_time = Instant::now();

    loop {
        // Update the state, or quit if necessary.
        if let Ok(Some(next_mode)) = mode.next_state() {
            mode = next_mode;
        } else {
            // Break out of the loop if there was an error (since we don't expect to recover from
            // errors reading user input & such), or if the next state is `None` (which means the
            // application should quit).
            break;
        }

        // Draw the state to the terminal.
        mode.draw(&mut terminal)?;

        // Calculate the time elapsed since the last frame.
        let elapsed = Instant::now().duration_since(last_frame_time);

        // Sleep until next iteration (`n+1`)
        if elapsed < *FRAME_WAIT_DURATION {
            std::thread::sleep(*FRAME_WAIT_DURATION - elapsed);
        }

        // Update the last frame time to now (frame `n`), since we have
        //  finished using the frame time of the previous frame (`n`) and we will return to
        //  the top of the loop to begin frame `n+1`.
        last_frame_time = std::time::Instant::now();
    }

    // Broke out of loop because AppState was 'Quitting', initiate shutdown.
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

