use crate::app::App;
use chrono::{Local, Timelike};

pub fn displayed_hms(app: &App) -> (u32, u32, u32) {
    if let Some(remaining) = app.remaining {
        let secs = remaining.as_secs();
        let h = (secs / 3600) as u32;
        let m = ((secs % 3600) / 60) as u32;
        let s = (secs % 60) as u32;
        (h, m, s)
    } else {
        let now = Local::now();
        let mut h = now.hour();
        if app.args.twelve_hour {
            if h > 12 {
                h -= 12;
            }
            if h == 0 {
                h = 12;
            }
        }
        (h, now.minute(), now.second())
    }
}

pub fn digit_indices(h: u32, m: u32, s: u32, show_seconds: bool) -> Vec<usize> {
    let mut digits = Vec::new();

    // Hours
    if h >= 10 {
        digits.push((h / 10) as usize);
    } else {
        digits.push(0);
    }
    digits.push((h % 10) as usize);

    // Colon
    digits.push(10);

    // Minutes
    digits.push((m / 10) as usize);
    digits.push((m % 10) as usize);

    // Seconds
    if show_seconds {
        digits.push(10);
        digits.push((s / 10) as usize);
        digits.push((s % 10) as usize);
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_digits_without_seconds() {
        let digits = digit_indices(9, 5, 7, false);

        assert_eq!(digits, vec![0, 9, 10, 0, 5]);
    }

    #[test]
    fn builds_digits_with_seconds() {
        let digits = digit_indices(12, 34, 56, true);

        assert_eq!(digits, vec![1, 2, 10, 3, 4, 10, 5, 6]);
    }
}
