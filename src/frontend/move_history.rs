use crate::game::Position;
use macroquad::prelude::*;

struct MoveEntry {
    from: Position,
    to: Position,
    added_at: f64,
}

pub struct MoveHistory {
    moves: Vec<MoveEntry>,
    collapsed: bool,
    open_progress: f32,
}

impl MoveHistory {
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            collapsed: false,
            open_progress: 1.0,
        }
    }

    pub fn add_move(&mut self, from: Position, to: Position) {
        self.moves.push(MoveEntry {
            from,
            to,
            added_at: get_time(),
        });
    }

    pub fn position_to_algebraic(pos: &Position) -> String {
        format!("{}{}", (b'a' + pos.col as u8) as char, pos.row + 1)
    }

    fn format_move(&self, idx: usize, from: Position, to: Position) -> String {
        let move_num = (idx / 2) + 1;
        let from_sq = Self::position_to_algebraic(&from);
        let to_sq = Self::position_to_algebraic(&to);

        if idx % 2 == 0 {
            format!("{}. {}{}", move_num, from_sq, to_sq)
        } else {
            format!("{}{}", from_sq, to_sq)
        }
    }

    pub fn draw(&mut self, tile_size: f32, current_time: f64) {
        self.update_animation();

        let panel_width = tile_size * 3.2;
        let panel_height = screen_height() - 40.0;
        let collapsed_width = tile_size * 0.5;
        let panel_width_current =
            collapsed_width + ((panel_width - collapsed_width) * self.open_progress);
        let panel_x = screen_width() - panel_width_current - 20.0;
        let panel_y = 20.0;
        let button_size = tile_size * 0.33;
        let collapse_x = panel_x + panel_width_current - (button_size * 2.0) - 18.0;
        let close_x = panel_x + panel_width_current - button_size - 10.0;
        let button_y = panel_y + 8.0;

        draw_rectangle(
            panel_x,
            panel_y,
            panel_width_current,
            panel_height,
            Color::from_rgba(40, 40, 40, 230),
        );
        draw_rectangle_lines(
            panel_x,
            panel_y,
            panel_width_current,
            panel_height,
            2.0,
            WHITE,
        );

        let header_font = tile_size * 0.28;
        let content_font = tile_size * 0.24;
        let header_y = panel_y + tile_size * 0.38;
        let start_y = panel_y + tile_size * 0.62;
        let line_height = tile_size * 0.32;

        if self.open_progress > 0.15 {
            draw_text("Move History", panel_x + 10.0, header_y, header_font, WHITE);
            self.draw_button(collapse_x, button_y, button_size, "<");
            self.draw_button(close_x, button_y, button_size, "X");
        } else {
            draw_text("<", panel_x + 10.0, header_y, header_font * 1.1, WHITE);
        }

        let max_visible = ((panel_height - tile_size * 0.75) / line_height).floor() as usize;
        let start_idx = if self.moves.len() > max_visible {
            self.moves.len() - max_visible
        } else {
            0
        };

        if self.open_progress > 0.6 {
            for (i, entry) in self.moves.iter().enumerate().skip(start_idx) {
                let move_text = self.format_move(i, entry.from, entry.to);
                let visible_chars =
                    typed_char_count(&move_text, current_time - entry.added_at, 28.0);
                let typed_text: String = move_text.chars().take(visible_chars).collect();
                let y_pos = start_y + ((i - start_idx) as f32 * line_height);
                draw_text(&typed_text, panel_x + 15.0, y_pos, content_font, WHITE);
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if self.open_progress <= 0.15 && is_hovered(panel_x, panel_y, panel_width_current, 48.0)
            {
                self.collapsed = false;
            } else if self.open_progress > 0.15
                && is_hovered(collapse_x, button_y, button_size, button_size)
            {
                self.collapsed = true;
            } else if self.open_progress > 0.15
                && is_hovered(close_x, button_y, button_size, button_size)
            {
                self.collapsed = true;
            }
        }
    }

    fn update_animation(&mut self) {
        let target = if self.collapsed { 0.0 } else { 1.0 };
        let speed = 8.0 * get_frame_time();

        if self.open_progress < target {
            self.open_progress = (self.open_progress + speed).min(target);
        } else if self.open_progress > target {
            self.open_progress = (self.open_progress - speed).max(target);
        }
    }

    fn draw_button(&self, x: f32, y: f32, size: f32, label: &str) {
        let hovered = is_hovered(x, y, size, size);
        draw_rectangle(
            x,
            y,
            size,
            size,
            if hovered {
                Color::from_rgba(80, 80, 90, 255)
            } else {
                Color::from_rgba(60, 60, 70, 255)
            },
        );
        draw_rectangle_lines(x, y, size, size, 2.0, WHITE);
        draw_text(label, x + size * 0.22, y + size * 0.72, size * 0.65, WHITE);
    }
}

fn is_hovered(x: f32, y: f32, width: f32, height: f32) -> bool {
    let (mx, my) = mouse_position();
    mx >= x && mx <= x + width && my >= y && my <= y + height
}

fn typed_char_count(text: &str, elapsed: f64, chars_per_second: f64) -> usize {
    let total = text.chars().count();
    if elapsed <= 0.0 {
        0
    } else {
        ((elapsed * chars_per_second).floor() as usize).min(total)
    }
}
