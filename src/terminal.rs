use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

pub type AppTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct TerminalGuard {
    terminal: Option<AppTerminal>,
}

impl TerminalGuard {
    pub fn new(terminal: AppTerminal) -> Self {
        Self {
            terminal: Some(terminal),
        }
    }

    pub fn terminal_mut(&mut self) -> &mut AppTerminal {
        self.terminal
            .as_mut()
            .expect("terminal should be available before restore")
    }

    pub fn restore(&mut self) -> io::Result<()> {
        if let Some(mut terminal) = self.terminal.take() {
            restore_terminal(&mut terminal)?;
        }

        Ok(())
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

pub fn setup_terminal() -> io::Result<AppTerminal> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn restore_terminal(terminal: &mut AppTerminal) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
