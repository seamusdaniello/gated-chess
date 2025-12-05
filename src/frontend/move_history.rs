use macroquad::prelude::*;
use crate::game::Position;

pub struct MoveHistory {
    moves: Vec<(Position, Position)>
}

impl MoveHistory {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    pub fn add_move(&mut self, from: Position, to: Position) {
        self.moves.push((from, to));
    }

    pub fn position_to_algebraic(pos: &Position) -> String {
        format!("{}{}",
        (b'a' + pos.col as u8) as char,
        pos.row + 1
        )
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

    pub fn draw(&self, tile_size: f32) {
        let panel_y = 20.0;
        let panel_width = 250.0;
        let panel_height = screen_height() - 40.0;
        let panel_x = screen_width() - panel_width - 20.0;
        let panel_y = 20.0;
        
        // Draw background panel
        draw_rectangle(panel_x, panel_y, panel_width, panel_height, Color::from_rgba(40, 40, 40, 230));
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, WHITE);
        
        // Draw title
        draw_text("Move History", panel_x + 10.0, panel_y + 30.0, 24.0, WHITE);
        
        // Draw moves
        let start_y = panel_y + 50.0;
        let line_height = 25.0;
        let max_visible = ((panel_height - 60.0) / line_height).floor() as usize;
        let start_idx = if self.moves.len() > max_visible {
            self.moves.len() - max_visible
        } else {
            0
        };
        
        for (i, (from, to)) in self.moves.iter().enumerate().skip(start_idx) {
            let move_text = self.format_move(i, *from, *to);
            let y_pos = start_y + ((i - start_idx) as f32 * line_height);
            draw_text(&move_text, panel_x + 15.0, y_pos, 20.0, WHITE);
        }
    }
}