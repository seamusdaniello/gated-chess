use macroquad::prelude::*;
use crate::game::{Game, Position};
use crate::gates::GateType;
use crate::pieces::{PieceType, Piece};

mod load_pieces;

use load_pieces::PieceTextures;

const TILE_SIZE: f32 = 80.0;

static mut SELECTED: Option<Position> = None;

pub async fn run_ui(mut game: Game) {

    let piece_textures = PieceTextures::load().await;

    let light_tile = load_texture("images/panel/white-panel.png").await.unwrap();
    let dark_tile = load_texture("images/panel/black-panel.png").await.unwrap();

    loop {
        clear_background(BLACK);

        draw_board(&game, &light_tile, &dark_tile);
        draw_pieces(&game, &piece_textures);

        draw_selected();

        if let Some((from, to)) = process_click() {
            dbg!("UI Clicked!");
            let _ = game.make_move(from, to);
            unsafe { SELECTED = None; } // reset selection after move
        }

        next_frame().await;
    }
}

fn draw_board(game: &Game, light: &Texture2D, dark: &Texture2D) {
    let gate_color = BLACK;

    for row in 0..8 {
        for col in 0..8 {
            let tex = if game.board[row][col].gate.is_some() {
                light
            } else if (row + col) % 2 == 0 {
                light
            } else {
                dark
            };

            draw_texture_ex(
                tex,
                col as f32 * TILE_SIZE,
                row as f32 * TILE_SIZE,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
            );
        }
    }
}

fn draw_pieces(game: &Game, textures: &PieceTextures) {
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = game.board[row][col].piece {
                if let Some(tex) = textures.get(piece.kind, piece.color) {
                    draw_texture_ex(
                        tex,
                        col as f32 * TILE_SIZE,
                        row as f32 * TILE_SIZE,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                    );
                } else {
                    // fallback to text if no texture is available
                    let symbol = match piece.kind {
                        PieceType::Pawn => "♙",
                        PieceType::Knight => "♘",
                        PieceType::Bishop => "♗",
                        PieceType::Rook => "♖",
                        PieceType::Queen => "♕",
                        PieceType::King => "♔",
                    };
                    let color = if piece.color == crate::pieces::Color::White { WHITE } else { BLACK };
                    draw_text(
                        symbol,
                        col as f32 * TILE_SIZE + 25.0,
                        row as f32 * TILE_SIZE + 55.0,
                        50.0,
                        color
                    );
                }
            }
        }
    } // <-- close the loops
} // <-- close the function


fn process_click() -> Option<(Position, Position)> {
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse = mouse_position();
        let col = (mouse.0 / TILE_SIZE) as usize;
        let row = (mouse.1 / TILE_SIZE) as usize;

        if row < 8 && col < 8 {
            let pos = Position { row, col };

            unsafe {
                if let Some(from) = SELECTED {
                    return Some((from, pos));
                } else {
                    SELECTED = Some(pos);
                }
            }
        }
    }

    None
}

fn draw_selected() {
    unsafe {
        if let Some(pos) = SELECTED {
            draw_rectangle(
                pos.col as f32 * TILE_SIZE,
                pos.row as f32 * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
                Color::from_rgba(255, 255, 0, 80)
            );
        }
    }
}
