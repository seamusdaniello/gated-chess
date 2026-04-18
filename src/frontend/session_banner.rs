use macroquad::prelude::*;

pub fn draw_status(title: &str, detail: &str, visible_chars: usize) -> bool {
    let width = 560.0;
    let height = 140.0;
    let x = (screen_width() - width) / 2.0;
    let y = 24.0;
    let close_size = 30.0;
    let close_x = x + width - close_size - 12.0;
    let close_y = y + 12.0;

    draw_rectangle(x, y, width, height, Color::from_rgba(20, 20, 28, 220));
    draw_rectangle_lines(x, y, width, height, 3.0, GOLD);
    draw_text(title, x + 24.0, y + 44.0, 34.0, WHITE);
    let typed_detail: String = detail.chars().take(visible_chars).collect();
    draw_text(&typed_detail, x + 24.0, y + 92.0, 24.0, LIGHTGRAY);

    let hovered = is_hovered(close_x, close_y, close_size, close_size);
    draw_rectangle(
        close_x,
        close_y,
        close_size,
        close_size,
        if hovered {
            Color::from_rgba(120, 55, 55, 255)
        } else {
            Color::from_rgba(70, 35, 35, 255)
        },
    );
    draw_rectangle_lines(close_x, close_y, close_size, close_size, 2.0, WHITE);
    draw_text("X", close_x + 8.0, close_y + 22.0, 22.0, WHITE);

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn is_hovered(x: f32, y: f32, width: f32, height: f32) -> bool {
    let (mx, my) = mouse_position();
    mx >= x && mx <= x + width && my >= y && my <= y + height
}
