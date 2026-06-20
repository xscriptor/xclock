mod app;
mod args;
mod clock;
mod color;
mod duration;
mod font;
mod input;
mod runtime;
mod terminal;
mod ui;

use app::App;
use args::Args;
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut app = App::new(args)?;

    let terminal = terminal::setup_terminal()?;
    let mut terminal = terminal::TerminalGuard::new(terminal);

    let res = runtime::run_app(terminal.terminal_mut(), &mut app);
    let restore_res = terminal.restore();

    restore_res?;
    res?;

    Ok(())
}
