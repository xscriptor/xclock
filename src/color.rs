use clap::ValueEnum;
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum AppColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
    Black,
}

impl AppColor {
    pub fn to_ratatui_color(self) -> Color {
        match self {
            Self::Red => Color::Red,
            Self::Green => Color::Green,
            Self::Blue => Color::Blue,
            Self::Yellow => Color::Yellow,
            Self::Cyan => Color::Cyan,
            Self::Magenta => Color::Magenta,
            Self::White => Color::White,
            Self::Black => Color::Black,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_green_to_terminal_color() {
        assert_eq!(AppColor::Green.to_ratatui_color(), Color::Green);
    }

    #[test]
    fn maps_magenta_to_terminal_color() {
        assert_eq!(AppColor::Magenta.to_ratatui_color(), Color::Magenta);
    }
}
