use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use app::AppMode;
use commands::{AppCommand, StateInducer};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use lazy_static::lazy_static;
use ratatui::{prelude::CrosstermBackend, widgets::Block, Frame, Terminal};
use ui::splash_screen;

mod app;
mod commands;
mod theme;
mod ui;

pub const FRAMES_PER_SECOND: u64 = 24;

lazy_static! {
    pub static ref FRAME_WAIT_DURATION: Duration = {
        let frame_wait_millis: u64 = 1000 / (FRAMES_PER_SECOND * 1000);
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
        // Given the current state, check for user interaction
        match mode {
            AppMode::Initializing => {
                // Draw the TUI for the current frame
                terminal.draw(|frame: &mut Frame| {
                    splash_screen(frame);
                })?;

                // We're inside of our main loop now, so we're done initializing.
                // Update the state to reflect that and then continue to next frame.
                mode = AppMode::Running(app::RunMode::EditingEncounter);
                continue;
            }
            AppMode::Running(_run_mode) => {
                // Check for keypress events.
                let key_opt = poll_for_keypress().context("Couldn't poll for keypress.")?;

                // Map keypress to command based on current mode & pressed key.
                let command: AppCommand = (&mode, key_opt).into();

                // If the pressed key corresponds to a command, run command against state.
                StateInducer::from(command)((&mut mode, None));

                // TODO - Other stuff while running.
            }
            AppMode::Quitting => {
                // If our state says we're quitting, we're gonna quit. Break out of
                // the loop and beginning cleaning up after ourselves.
                break;
            }
        }

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

fn ui(frame: &mut Frame) {
    frame.render_widget(Block::default().title("Hello world!"), frame.size());
}
