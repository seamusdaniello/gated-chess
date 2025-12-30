use macroquad::prelude::*;
use crate::frontend::load_pieces::AnimationState;
use crate::game::{Game, Position};
use crate::gates::update_gate_animation;
use crate::gates::update_gates;
use crate::pieces::Color::{White, Black};

mod load_pieces;
mod load_gates;
mod load_frame;
mod move_history;
mod game_over;
mod start_menu;

use load_pieces::PieceTextures;
use load_gates::GateTextures;
use load_frame::BoardFrame;
use move_history::MoveHistory;
use game_over::GameOverBanner;
use start_menu::StartMenu;

static mut SELECTED: Option<Position> = None;
static mut HOVERED: Option<Position> = None;
static mut HIGHLIGHTED_COLUMN: Option<usize> = None;
static mut TYPING_MODE: bool = false;

// Add animation state tracking
struct PieceAnimationState {
    current_frame: usize,
    last_update: f32,
    frame_duration: f32, // seconds per frame
    direction: i32,
}

impl PieceAnimationState {
    fn new(frame_duration: f32) -> Self {
        Self {
            current_frame: 0,
            last_update: 0.0,
            frame_duration,
            direction: 1,
        }
    }

    fn update(&mut self, current_time: f32, total_frames: usize) {
        if total_frames == 0 {
            return;
        }

        if current_time - self.last_update >= self.frame_duration {
            let next = self.current_frame as i32 + self.direction;

            if next >= total_frames as i32 {
                self.direction = -1;
                self.current_frame = total_frames.saturating_sub(2).max(0);
            } else if next < 0 {
                self.direction = 1;
                self.current_frame = 1.min(total_frames - 1);
            } else {
                self.current_frame = next as usize;
            }

            self.last_update = current_time;
        }
    }
}

pub async fn run_ui(mut game: Game) {
    let mut in_menu = true;

    while in_menu {
        clear_background(BLACK);

        if StartMenu::draw() {
            in_menu = false;
        }

        next_frame().await;
    }

    let board_frame = BoardFrame::load("images/frames/frame-1.png").await;
    let piece_textures = PieceTextures::load().await;
    let gate_textures = GateTextures::load().await;
    let mut last_turn = game.current_turn;

    let light_tile = load_texture("images/panel/white-panel.png").await.unwrap();
    let dark_tile = load_texture("images/panel/black-panel.png").await.unwrap();

    let mut last_update = 0.0;
    let mut move_history = MoveHistory::new();

    let mut game_over = false;
    let mut winner: Option<crate::pieces::Color> = None;
    
    // Initialize piece animation state
    let mut piece_anim_state = PieceAnimationState::new(0.3); // 10 FPS animation
    
    loop {
        let now = get_time();

        clear_background(BLACK);

        if !game_over {
            if game.current_turn != last_turn {
                update_gates(&mut game);
                last_turn = game.current_turn;
            }

            if now - last_update >= 0.5 {
                update_gate_animation(&mut game);
                last_update = now;
            }
        }
        
        let tile_size = f32::min(screen_width(), screen_height()) / 8.0;
        let board_center = vec2(4.0 * tile_size, 4.0 * tile_size);

        let mut camera = Camera2D {
            target: board_center,
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
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
        
        // Draw highlighted column before pieces
        draw_highlighted_column(tile_size);
        
        // Update and draw pieces with animations
        draw_pieces(&game, &piece_textures, &camera, tile_size, &mut piece_anim_state, now as f32);
        draw_selected(tile_size);

        set_default_camera();

        // Process clicks only if game is not over
        if !game_over {
            // Process keyboard input first
            if let Some((from, to)) = process_keyboard_input(&game) {
                if game.make_move(from, to).is_ok() {
                    move_history.add_move(from, to);

                    if game.is_checkmate(game.current_turn) {
                        game_over = true;
                        winner = Some(if game.current_turn == White { Black } else { White });
                    } else if game.is_stalemate(game.current_turn) {
                        game_over = true;
                        winner = None;
                    }
                }
                unsafe { 
                    SELECTED = None;
                    HIGHLIGHTED_COLUMN = None;
                    TYPING_MODE = false;
                }
            }
            
            // Then process mouse clicks
            if let Some((from, to)) = process_click(&game, &camera, tile_size) {
                if game.make_move(from, to).is_ok() {
                    move_history.add_move(from, to);

                    if game.is_checkmate(game.current_turn) {
                        game_over = true;
                        winner = Some(if game.current_turn == White { Black } else { White });
                    } else if game.is_stalemate(game.current_turn) {
                        game_over = true;
                        winner = None;
                    }
                }
                unsafe { SELECTED = None; }
            }
        }

        // Draw move history panel
        move_history.draw(tile_size);

        // Draw game over banner if game ended
        if game_over {
            GameOverBanner::draw(winner);
            
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
        }

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

fn draw_pieces(
    game: &Game, 
    textures: &PieceTextures, 
    camera: &Camera2D, 
    current_tile_size: f32,
    anim_state: &mut PieceAnimationState,
    current_time: f32,
) {
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = game.board[row][col].piece {
                // Try to get animation first, fall back to static texture
                let tex = if let Some(frames) = textures.get_animation(piece.kind, piece.color, AnimationState::Idle) {
                    // Update animation frame
                    anim_state.update(current_time, frames.len());
                    // Get current frame
                    &frames[anim_state.current_frame]
                } else {
                    // Fall back to static texture if no animation exists
                    match textures.get(piece.kind, piece.color) {
                        Some(t) => t,
                        None => continue, // Skip if no texture at all
                    }
                };

                // Calculate piece position
                let draw_pos = vec2(col as f32 * current_tile_size, row as f32 * current_tile_size);

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

fn process_keyboard_input(game: &Game) -> Option<(Position, Position)> {
    unsafe {
        // Check for Enter key to enable typing mode
        if is_key_pressed(KeyCode::Enter) {
            if let Some(_) = SELECTED {
                TYPING_MODE = true;
                HIGHLIGHTED_COLUMN = None;
            }
        }
        
        // Check for Escape to cancel typing mode
        if is_key_pressed(KeyCode::Escape) {
            TYPING_MODE = false;
            HIGHLIGHTED_COLUMN = None;
        }
        
        // Check for column letter press (A-H)
        if let Some(col) = process_chess_column() {
            HIGHLIGHTED_COLUMN = Some(col);
        }
        
        // Check for row number press (1-8)
        if let Some(row) = process_chess_row() {
            if let Some(col) = HIGHLIGHTED_COLUMN {
                let pos = Position { row, col };
                
                if let Some(from) = SELECTED {
                    if TYPING_MODE {
                        // In typing mode, this is the destination
                        HIGHLIGHTED_COLUMN = None;
                        TYPING_MODE = false;
                        return Some((from, pos));
                    } else {
                        // Not in typing mode, select this position
                        SELECTED = Some(pos);
                        HIGHLIGHTED_COLUMN = None;
                    }
                } else {
                    // No piece selected yet, select this position
                    SELECTED = Some(pos);
                    HIGHLIGHTED_COLUMN = None;
                }
            }
        }
        
        None
    }
}

fn process_chess_column() -> Option<usize> {
    if is_key_pressed(KeyCode::A) { return Some(0); }
    if is_key_pressed(KeyCode::B) { return Some(1); }
    if is_key_pressed(KeyCode::C) { return Some(2); }
    if is_key_pressed(KeyCode::D) { return Some(3); }
    if is_key_pressed(KeyCode::E) { return Some(4); }
    if is_key_pressed(KeyCode::F) { return Some(5); }
    if is_key_pressed(KeyCode::G) { return Some(6); }
    if is_key_pressed(KeyCode::H) { return Some(7); }
    None
}

fn process_chess_row() -> Option<usize> {
    if is_key_pressed(KeyCode::Key1) { return Some(0); } // Row 1 = index 0 (top)
    if is_key_pressed(KeyCode::Key2) { return Some(1); }
    if is_key_pressed(KeyCode::Key3) { return Some(2); }
    if is_key_pressed(KeyCode::Key4) { return Some(3); }
    if is_key_pressed(KeyCode::Key5) { return Some(4); }
    if is_key_pressed(KeyCode::Key6) { return Some(5); }
    if is_key_pressed(KeyCode::Key7) { return Some(6); }
    if is_key_pressed(KeyCode::Key8) { return Some(7); } // Row 8 = index 7 (bottom)
    None
}

fn draw_highlighted_column(current_tile_size: f32) {
    unsafe {
        if let Some(col) = HIGHLIGHTED_COLUMN {
            for row in 0..8 {
                draw_rectangle(
                    col as f32 * current_tile_size,
                    row as f32 * current_tile_size,
                    current_tile_size,
                    current_tile_size,
                    Color::from_rgba(100, 150, 255, 60)
                );
            }
        }
    }
}

fn draw_selected(current_tile_size: f32) {
    unsafe {
        if let Some(pos) = SELECTED {
            // Use different color based on typing mode
            let color = if TYPING_MODE {
                Color::from_rgba(0, 255, 0, 100)  // Green in typing mode
            } else {
                Color::from_rgba(255, 255, 0, 80)  // Yellow normally
            };
            
            draw_rectangle(
                pos.col as f32 * current_tile_size,
                pos.row as f32 * current_tile_size,
                current_tile_size,
                current_tile_size,
                color
            );
        }
    }
}