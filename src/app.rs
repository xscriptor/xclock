use crate::{args::Args, duration::parse_duration};
use std::{
    error::Error,
    time::{Duration, Instant},
};

pub struct App {
    pub args: Args,
    pub should_quit: bool,
    pub countdown_target: Option<Instant>,
    pub remaining: Option<Duration>,
}

impl App {
    pub fn new(args: Args) -> Result<Self, Box<dyn Error>> {
        let (countdown_target, initial_remaining) = if let Some(ref s) = args.countdown {
            let duration = parse_duration(s)?;
            (Some(Instant::now() + duration), Some(duration))
        } else {
            (None, None)
        };

        Ok(Self {
            args,
            should_quit: false,
            countdown_target,
            remaining: initial_remaining,
        })
    }

    pub fn on_tick(&mut self) {
        if let Some(target) = self.countdown_target {
            let now = Instant::now();
            if now >= target {
                self.remaining = Some(Duration::from_secs(0));
            } else {
                self.remaining = Some(target - now);
            }
        }
    }
}