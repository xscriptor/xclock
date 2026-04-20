use crate::{app::App, clock, color, font};
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();
    let color = color::parse_color(&app.args.color);

    let block = if app.args.no_box {
        Block::default()
    } else {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
    };

    let inner_area = block.inner(size);
    f.render_widget(block, size);

    let (h, m, s) = clock::displayed_hms(app);
    let digits = clock::digit_indices(h, m, s, app.args.seconds);

    let digit_width = font::WIDTH;
    let base_spacing = 2;
    let base_width = digits.len() * digit_width + (digits.len() - 1) * base_spacing;
    let base_height = font::HEIGHT;

    let available_w = inner_area.width as usize;
    let available_h = inner_area.height as usize;

    let scale = if base_width > 0 && base_height > 0 {
        std::cmp::min(available_w / base_width, available_h / base_height)
    } else {
        1
    };
    let scale = std::cmp::max(1, scale);

    let scaled_width = base_width * scale;
    let scaled_height = base_height * scale;
    let spacing = base_spacing * scale;

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

    for row in 0..scaled_height {
        let font_y = row / scale;
        if font_y >= font::HEIGHT {
            continue;
        }

        let mut line_text = String::new();

        for (i, digit_idx) in digits.iter().enumerate() {
            let digit = font::get_digit(*digit_idx);
            let digit_row = digit[font_y];
            for &filled in &digit_row {
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

        let line = Line::from(vec![Span::styled(line_text, Style::default().fg(color))]);
        let paragraph = Paragraph::new(line);
        let area = Rect {
            x,
            y: y + row as u16,
            width: scaled_width as u16,
            height: 1,
        };

        if area.y < inner_area.bottom() {
            f.render_widget(paragraph, area);
        }
    }
}