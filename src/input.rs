use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::{io, time::Duration};

pub fn poll_quit_event(timeout: Duration) -> io::Result<bool> {
    if crossterm::event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            return Ok(match key.code {
                KeyCode::Char('q') | KeyCode::Esc => true,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => true,
                _ => false,
            });
        }
    }

    Ok(false)
}