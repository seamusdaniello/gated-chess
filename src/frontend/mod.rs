use macroquad::prelude::*;
use crate::game::{Game, Position};
use crate::pieces::Piece;

const TILE_SIZE: f32 = 80.0;

pub async fn run_ui(mut game: Game) {

    loop {
        clear_background(BLACK);

        draw_board();
        draw_pieces(&game);

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some((from, to)) = process_click() {

                let _ = game.make_move(from, to);
            }
        }

        next_frame().await;
    }
}

fn draw_board() {
    for row in 0..8 {
        for col in 0..8 {
            let color = if (row + col) % 2 == 0 {
                LIGHTGRAY
            } else {
                DARKGRAY
            };

            draw_rectangle(
                col as f32 * TILE_SIZE,
                row as f32 * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
                color,
            );
        }
    }
}

fn draw_pieces(game: &Game) {
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = game.board.get_piece(row, col) {
                let symbol = match piece.kind {
                    crate::pieces::PieceType::Pawn => "♙",
                    crate::pieces::PieceType::Knight => "♘",
                    crate::pieces::PieceType::Bishop => "♗",
                    crate::pieces::PieceType::Rook => "♖",
                    crate::pieces::PieceType::Queen => "♕",
                    crate::pieces::PieceType::King => "♔",
                };

                let color = if piece.color == crate::pieces::Color::White {
                    WHITE
                } else {
                    BLACK
                };

                draw_text(
                    symbol,
                    col as f32 * TILE_SIZE + 25.0,
                    row as f32 * TILE_SIZE + 55.0,
                    50.0,
                    color,
                );
            }
        }
    }
}

fn process_click() -> Option<(Position, Position)> {
    None
}