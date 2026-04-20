use std::{error::Error, time::Duration};

pub fn parse_duration(s: &str) -> Result<Duration, Box<dyn Error>> {
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