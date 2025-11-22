use macroquad::prelude::*;
use crate::game::{Game, Position};
use crate::gates::update_gate_animation;
use crate::gates::update_gates;

use crate::pieces::Color::{White, Black};

mod load_pieces;
mod load_gates;
mod load_frame;
mod move_history;

use load_pieces::PieceTextures;
use load_gates::GateTextures;
use load_frame::BoardFrame;
use move_history::MoveHistory;

static mut SELECTED: Option<Position> = None;
static mut HOVERED: Option<Position> = None;

pub async fn run_ui(mut game: Game) {

    let board_frame = BoardFrame::load("images/frames/frame-1.png").await;
    let piece_textures = PieceTextures::load().await;
    let gate_textures = GateTextures::load().await;
    let mut last_turn = game.current_turn;

    let light_tile = load_texture("images/panel/white-panel.png").await.unwrap();
    let dark_tile = load_texture("images/panel/black-panel.png").await.unwrap();

    let mut last_update = 0.0;
    let mut move_history = MoveHistory::new(); // Create move history instance

    loop {
        let now = get_time();

        clear_background(BLACK);

        if game.current_turn != last_turn {
            update_gates(&mut game);
            last_turn = game.current_turn;
        }

        if now - last_update >= 0.5 {
            update_gate_animation(&mut game);
            last_update = now;
        }
        
        let tile_size = f32::min(screen_width(), screen_height()) / 8.0;
        let board_center = vec2(4.0 * tile_size, 4.0 * tile_size);

            let mut camera = Camera2D {
                target: board_center,
                zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()), // remove negative y
                rotation: 0.5,
                ..Default::default()
            };


        let world = camera.screen_to_world(mouse_position().into());
        let mx = world.x;
        let my = world.y;

        unsafe {
            if mx >= 0.0 && my >= 0.0 && mx < 8.0 * tile_size && my < 8.0 * tile_size {
                HOVERED = Some(Position {
                    row: (my / tile_size).floor() as usize,
                    col: (mx / tile_size).floor() as usize,
                });
            } else {
                HOVERED = None;
            }
        }

        // Update rotation and zoom depending on turn
        if game.current_turn == White {
            camera.rotation = 0.0;
            camera.zoom = vec2(2.0 / screen_width(), -2.0 / screen_height());
        } else {
            camera.rotation = 0.0;
            camera.zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());
        }

        set_camera(&camera);

        board_frame.draw(tile_size);
        draw_board(&game, &light_tile, &dark_tile, &gate_textures.tex_vector, tile_size);
        draw_pieces(&game, &piece_textures, &camera, tile_size);
        draw_selected(tile_size);

        set_default_camera(); // return to UI space

        // Process clicks and add to move history when successful
        if let Some((from, to)) = process_click(&game, &camera, tile_size) {
            if game.make_move(from, to).is_ok() {
                move_history.add_move(from, to); // Add move to history
            }
            unsafe { SELECTED = None; }
        }

        // Draw move history panel (in UI space, after set_default_camera)
        move_history.draw(tile_size);

        next_frame().await;
    }
}

fn draw_board(game: &Game, light: &Texture2D, dark: &Texture2D, gates: &[Texture2D], tile_size: f32) {
    for row in 0..8 {
        for col in 0..8 {
            let tex: &Texture2D = if game.board[row][col].gate.is_some() {
                let frame = game.board[row][col].animation_frame.unwrap_or(0);
                &gates[frame]
            } else if (row + col) % 2 == 0 {
                light
            } else {
                dark
            };

            draw_texture_ex(
                tex,
                col as f32 * tile_size,
                row as f32 * tile_size,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tile_size, tile_size)),
                    ..Default::default()
                },
            );
        }
    }
}

fn draw_pieces(game: &Game, textures: &PieceTextures, camera: &Camera2D, current_tile_size: f32) {
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = game.board[row][col].piece {
                if let Some(tex) = textures.get(piece.kind, piece.color) {
                    // Calculate piece position
                    let mut draw_pos = vec2(col as f32 * current_tile_size, row as f32 * current_tile_size);

                    // Flip the position for black pieces relative to camera
                    let rotation = match game.current_turn {
                        White => camera.rotation + (std::f32::consts::PI * 1.0),
                        Black => camera.rotation, // upside down
                    };

                    draw_texture_ex(
                        tex,
                        draw_pos.x,
                        draw_pos.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(current_tile_size, current_tile_size)),
                            rotation,
                            pivot: None,
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}

fn process_click(game: &Game, camera: &Camera2D, current_tile_size: f32) -> Option<(Position, Position)> {
    if is_mouse_button_pressed(MouseButton::Left) {

        // Convert screen position → world position using the SAME camera
        let world = camera.screen_to_world(mouse_position().into());

        let mx = world.x;
        let my = world.y;

        if mx < 0.0 || my < 0.0 {
            return None;
        }

        let col = (mx / current_tile_size).floor() as isize;
        let row = (my / current_tile_size).floor() as isize;

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

fn draw_selected(current_tile_size: f32) {
    unsafe {
        if let Some(pos) = SELECTED {
            draw_rectangle(
                pos.col as f32 * current_tile_size,
                pos.row as f32 * current_tile_size,
                current_tile_size,
                current_tile_size,
                Color::from_rgba(255, 255, 0, 80)
            );
        }
    }
}