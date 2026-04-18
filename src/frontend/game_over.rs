use crate::pieces::Color as PieceColor;
use macroquad::prelude::*;

pub struct GameOverBanner;

impl GameOverBanner {
    pub fn draw(winner: Option<PieceColor>) -> bool {
        let banner_width = 600.0;
        let banner_height = 200.0;
        let banner_x = (screen_width() - banner_width) / 2.0;
        let banner_y = (screen_height() - banner_height) / 2.0;
        let close_size = 32.0;
        let close_x = banner_x + banner_width - close_size - 12.0;
        let close_y = banner_y + 12.0;

        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 150),
        );

        draw_rectangle(
            banner_x,
            banner_y,
            banner_width,
            banner_height,
            Color::from_rgba(40, 40, 40, 255),
        );
        draw_rectangle_lines(banner_x, banner_y, banner_width, banner_height, 4.0, GOLD);

        let title = "GAME OVER";
        draw_text(title, banner_x + 170.0, banner_y + 60.0, 48.0, WHITE);

        let winner_text = match winner {
            Some(PieceColor::White) => "White won.",
            Some(PieceColor::Black) => "Black won.",
            None => "Draw.",
        };
        draw_text(winner_text, banner_x + 200.0, banner_y + 120.0, 36.0, GOLD);

        draw_text(
            "Press ESC to exit",
            banner_x + 210.0,
            banner_y + 170.0,
            20.0,
            LIGHTGRAY,
        );

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
        draw_text("X", close_x + 8.0, close_y + 24.0, 24.0, WHITE);

        hovered && is_mouse_button_pressed(MouseButton::Left)
    }
}

fn is_hovered(x: f32, y: f32, width: f32, height: f32) -> bool {
    let (mx, my) = mouse_position();
    mx >= x && mx <= x + width && my >= y && my <= y + height
}
