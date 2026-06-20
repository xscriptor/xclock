use crate::color::AppColor;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Center the clock on the screen
    #[arg(short, long)]
    pub center: bool,

    /// Enable countdown mode (format: 1h30m, 5m, 10s)
    #[arg(short = 'C', long)]
    pub countdown: Option<String>,

    /// Show seconds
    #[arg(short, long)]
    pub seconds: bool,

    /// Color (red, green, blue, yellow, cyan, magenta, white, black)
    #[arg(short = 'r', long, value_enum, default_value = "green")]
    pub color: AppColor,

    /// 12-hour format
    #[arg(short = 't', long)]
    pub twelve_hour: bool,

    /// Hide the box borders
    #[arg(short = 'B', long)]
    pub no_box: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_color() {
        let args = Args::try_parse_from(["xclock", "--color", "cyan"]).unwrap();

        assert_eq!(args.color, AppColor::Cyan);
    }

    #[test]
    fn rejects_invalid_color() {
        let result = Args::try_parse_from(["xclock", "--color", "purple"]);

        assert!(result.is_err());
    }
}
