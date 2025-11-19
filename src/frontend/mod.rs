use macroquad::prelude::*;
use macroquad::camera::*;
use crate::game::{Game, Position};
use crate::gates::GateType;
use crate::pieces::{PieceType, Piece};

mod load_pieces;

use load_pieces::PieceTextures;

const TILE_SIZE: f32 = 80.0;

static mut SELECTED: Option<Position> = None;
static mut HOVERED: Option<Position> = None;

pub async fn run_ui(mut game: Game) {

    let piece_textures = PieceTextures::load().await;

    let light_tile = load_texture("images/panel/white-panel.png").await.unwrap();
    let dark_tile = load_texture("images/panel/black-panel.png").await.unwrap();

    let board_center = vec2(8.0 * TILE_SIZE / 2.0, 8.0 * TILE_SIZE / 2.0);

    let mut camera = Camera2D {
    target: board_center,
    zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()), // remove negative y
    rotation: 0.5,
    ..Default::default()
    };

    loop {
        clear_background(BLACK);

        let world = camera.screen_to_world(mouse_position().into());
        let mx = world.x;
        let my = world.y;

        unsafe {
            if mx >= 0.0 && my >= 0.0 && mx < 8.0 * TILE_SIZE && my < 8.0 * TILE_SIZE {
                HOVERED = Some(Position {
                    row: (my / TILE_SIZE).floor() as usize,
                    col: (mx / TILE_SIZE).floor() as usize,
                });
            } else {
                HOVERED = None;
            }
        }

        // Update rotation and zoom depending on turn
        if game.current_turn == crate::pieces::Color::White {
            camera.rotation = 0.0;
            camera.zoom = vec2(2.0 / screen_width(), -2.0 / screen_height());
        } else {
            camera.rotation = 0.0;
            camera.zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());
        }

        set_camera(&camera);

        draw_board(&game, &light_tile, &dark_tile);
        draw_pieces(&game, &piece_textures);
        draw_selected();

        set_default_camera(); // return to UI space

        if let Some((from, to)) = process_click(&game, &camera) {
            let _ = game.make_move(from, to);
            unsafe { SELECTED = None; }
        }

        next_frame().await;
    }
}

fn draw_board(game: &Game, light: &Texture2D, dark: &Texture2D) {
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


fn process_click(game: &Game, camera: &Camera2D) -> Option<(Position, Position)> {
    if is_mouse_button_pressed(MouseButton::Left) {

        // Convert screen position → world position using the SAME camera
        let world = camera.screen_to_world(mouse_position().into());

        let mx = world.x;
        let my = world.y;

        if mx < 0.0 || my < 0.0 {
            return None;
        }

        let col = (mx / TILE_SIZE).floor() as isize;
        let row = (my / TILE_SIZE).floor() as isize;

        if row < 0 || col < 0 || row >= 8 || col >= 8 {
            return None;
        }

        let pos = Position { row: row as usize, col: col as usize };

        unsafe {
            if let Some(from) = SELECTED {
                return Some((from, pos));
            } else {
                SELECTED = Some(pos);
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