pub mod piece_renderer;
pub mod board_renderer;
pub mod animation;

use macroquad::prelude::*;
use crate::game::Game;
use load_pieces::PieceTextures;

pub struct GameRenderer {
    animation_frame: usize,
}

impl GameRenderer {
    pub fn new() -> Self {
        Self { animation_frame: 0 }
    }

    pub fn render(&mut self, game: &Game, textures: &PieceTextures) {
        board_renderer::render_board(game);
        piece_renderer::render_pieces(game, textures, self.animation_frame);
        self.animation_frame += 1;
    }
}