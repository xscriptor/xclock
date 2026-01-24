mod args;
mod font;

use args::Args;
use chrono::{Local, Timelike};
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

struct App {
    args: Args,
    should_quit: bool,
    countdown_target: Option<Instant>,
    remaining: Option<Duration>,
}

impl App {
    fn new(args: Args) -> Result<Self, Box<dyn Error>> {
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

    fn on_tick(&mut self) {
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

fn parse_duration(s: &str) -> Result<Duration, Box<dyn Error>> {
    let mut total_secs = 0u64;
    let mut num_str = String::new();

    for c in s.chars() {
        if c.is_digit(10) {
            num_str.push(c);
        } else {
            let num: u64 = num_str.parse()?;
            num_str.clear();
            match c {
                'h' => total_secs += num * 3600,
                'm' => total_secs += num * 60,
                's' => total_secs += num,
                _ => return Err(format!("Unknown time unit: {}", c).into()),
            }
        }
    }
    
    // Handle case where string is just a number (assume seconds)
    if !num_str.is_empty() {
         let num: u64 = num_str.parse()?;
         total_secs += num;
    }

    Ok(Duration::from_secs(total_secs))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(args)?;

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Char('c') => {
                        if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                            app.should_quit = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Determine color
    let color = match app.args.color.to_lowercase().as_str() {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "white" => Color::White,
        "black" => Color::Black,
        _ => Color::Green,
    };

    let block = if app.args.no_box {
        Block::default()
    } else {
        Block::default().borders(Borders::ALL).border_style(Style::default().fg(color))
    };
    
    let inner_area = block.inner(size);
    f.render_widget(block, size);

    // Get time digits
    let (h, m, s) = if let Some(remaining) = app.remaining {
        let secs = remaining.as_secs();
        let h = (secs / 3600) as u32;
        let m = ((secs % 3600) / 60) as u32;
        let s = (secs % 60) as u32;
        (h, m, s)
    } else {
        let now = Local::now();
        let mut h = now.hour();
        if app.args.twelve_hour {
            if h > 12 { h -= 12; }
            if h == 0 { h = 12; }
        }
        (h, now.minute(), now.second())
    };

    let mut digits = Vec::new();
    // Hours
    if h >= 10 {
        digits.push(font::get_digit((h / 10) as usize));
    } else {
        digits.push(font::get_digit(0));
    }
    digits.push(font::get_digit((h % 10) as usize));
    
    // Colon
    digits.push(font::get_digit(10));

    // Minutes
    digits.push(font::get_digit((m / 10) as usize));
    digits.push(font::get_digit((m % 10) as usize));

    // Seconds
    if app.args.seconds {
        digits.push(font::get_digit(10));
        digits.push(font::get_digit((s / 10) as usize));
        digits.push(font::get_digit((s % 10) as usize));
    }

    // Render digits
    let digit_width = font::WIDTH;
    let base_spacing = 2;
    let base_width = digits.len() * digit_width + (digits.len() - 1) * base_spacing;
    let base_height = font::HEIGHT;

    // Calculate scale
    let available_w = inner_area.width as usize;
    let available_h = inner_area.height as usize;
    
    let scale = if base_width > 0 && base_height > 0 {
        std::cmp::min(
            available_w / base_width,
            available_h / base_height
        )
    } else {
        1
    };
    let scale = std::cmp::max(1, scale);

    let scaled_width = base_width * scale;
    let scaled_height = base_height * scale;
    let spacing = base_spacing * scale;

    // Center logic
    let x = if app.args.center {
        inner_area.x + ((available_w.saturating_sub(scaled_width)) / 2) as u16
    } else {
        inner_area.x + 2
    };
    
    let y = if app.args.center {
        inner_area.y + ((available_h.saturating_sub(scaled_height)) / 2) as u16
    } else {
        inner_area.y + 2
    };

    // Render each row of the digits
    for row in 0..scaled_height {
        let font_y = row / scale;
        if font_y >= font::HEIGHT { continue; } // Safety check

        let mut line_spans = Vec::new();
        let mut line_text = String::new();

        for (i, digit) in digits.iter().enumerate() {
            let digit_row = digit[font_y];
            for &filled in digit_row.iter() {
                let s = if filled { "█" } else { " " };
                for _ in 0..scale {
                    line_text.push_str(s);
                }
            }
            if i < digits.len() - 1 {
                for _ in 0..spacing {
                    line_text.push(' ');
                }
            }
        }
        
        line_spans.push(Span::styled(line_text, Style::default().fg(color)));
        
        let p = Paragraph::new(Line::from(line_spans));
        let area = Rect {
            x,
            y: y + row as u16,
            width: scaled_width as u16,
            height: 1,
        };
        // Only render if within bounds
        if area.y < inner_area.bottom() {
             f.render_widget(p, area);
        }
    }
}
