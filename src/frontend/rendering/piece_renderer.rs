use macroquad::prelude::*;
use crate::game::Game;
use crate::pieces::{PieceType, Color, PieceState};
use crate::ui::PieceTextures;
use crate::config::BOARD_SIZE;

pub fn render_pieces(
    game: &Game,
    textures: &PieceTextures,
    animation_frame: usize,
) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if let Some(piece) = game.board[row][col].piece {
                render_single_piece(&piece, row, col, textures, animation_frame);
            }
        }
    }
}

fn render_single_piece(
    piece: &crate::pieces::Piece,
    row: usize,
    col: usize,
    textures: &PieceTextures,
    animation_frame: usize,
) {
    let (x, y) = board_to_screen(row, col);

    if piece.kind == PieceType::Bishop
        && piece.color == Color::White
        && piece.state == PieceState::Idle
    {
        if let Some(frames) = textures.get_animation(
            PieceType::Bishop,
            Color::White,
            "idle"
        ) {
            let frame_idx = animation_frame % frames.len();
            draw_texture(&frames[frame_idx], x, y, WHITE);
            return;
        }
    }

    if let Some(texture) = textures.get(piece.kind, piece.color) {
        draw_texture(texture, x, y, WHITE);
    }
}