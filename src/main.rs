mod args;
mod app;
mod clock;
mod color;
mod duration;
mod font;
mod input;
mod runtime;
mod terminal;
mod ui;

use args::Args;
use clap::Parser;
use app::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut terminal = terminal::setup_terminal()?;

    let mut app = App::new(args)?;
    let res = runtime::run_app(&mut terminal, &mut app);
    terminal::restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
