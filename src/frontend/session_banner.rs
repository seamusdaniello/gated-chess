use macroquad::prelude::*;

pub fn draw_status(title: &str, detail: &str) {
    let width = 560.0;
    let height = 140.0;
    let x = (screen_width() - width) / 2.0;
    let y = 24.0;

    draw_rectangle(x, y, width, height, Color::from_rgba(20, 20, 28, 220));
    draw_rectangle_lines(x, y, width, height, 3.0, GOLD);
    draw_text(title, x + 24.0, y + 44.0, 34.0, WHITE);
    draw_text(detail, x + 24.0, y + 92.0, 24.0, LIGHTGRAY);
}
