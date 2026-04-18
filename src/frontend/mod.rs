use crate::frontend::load_pieces::AnimationState;
use crate::game::{Game, Position};
use crate::gates::update_gate_animation;
use crate::gates::update_gates;
use crate::network::{NetworkCommand, NetworkEvent, OnlineSession, SessionConfig, SessionRole};
use crate::pieces::Color as PieceColor;
use crate::pieces::Color::{Black, White};
use macroquad::prelude::*;

mod game_over;
mod load_frame;
mod load_gates;
mod load_pieces;
mod move_history;
mod session_banner;
mod start_menu;

use game_over::GameOverBanner;
use load_frame::BoardFrame;
use load_gates::GateTextures;
use load_pieces::PieceTextures;
use move_history::MoveHistory;
use session_banner::draw_status;
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
    let mut start_menu = StartMenu::new();
    let mut session_config = SessionConfig::Local;

    while in_menu {
        clear_background(BLACK);

        if let Some(config) = start_menu.draw() {
            session_config = config;
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
    let session = match &session_config {
        SessionConfig::Local => None,
        SessionConfig::Host { bind_addr } => Some(OnlineSession::host(bind_addr.clone())),
        SessionConfig::Join { server_addr } => Some(OnlineSession::join(server_addr.clone())),
    };
    let mut status_message = match &session_config {
        SessionConfig::Local => None,
        SessionConfig::Host { bind_addr } => Some(format!("Hosting on {}", bind_addr)),
        SessionConfig::Join { server_addr } => Some(format!("Connecting to {}", server_addr)),
    };
    let mut connection_ready = matches!(session_config, SessionConfig::Local);

    let mut game_over = false;
    let mut winner: Option<crate::pieces::Color> = None;

    // Initialize piece animation state
    let mut piece_anim_state = PieceAnimationState::new(0.3); // 10 FPS animation

    loop {
        let now = get_time();
        let board_perspective = board_perspective(&session, game.current_turn);

        while let Some(event) = session.as_ref().and_then(|online| online.try_recv()) {
            match event {
                NetworkEvent::WaitingForOpponent(addr) => {
                    status_message = Some(format!("Waiting for opponent on {}", addr));
                    connection_ready = false;
                }
                NetworkEvent::Connected(addr) => {
                    let role = session
                        .as_ref()
                        .map(|online| online.role())
                        .unwrap_or(SessionRole::Local);

                    status_message = Some(match role {
                        SessionRole::Host => format!("Opponent connected from {}", addr),
                        SessionRole::Client => format!("Connected to {}", addr),
                        SessionRole::Local => addr,
                    });
                    connection_ready = true;
                }
                NetworkEvent::RemoteMove(from, to) => {
                    let is_host = session
                        .as_ref()
                        .map(|online| online.role() == SessionRole::Host)
                        .unwrap_or(false);

                    if is_host {
                        if game.make_move(from, to).is_ok() {
                            move_history.add_move(from, to);
                            status_message = Some("Opponent move applied".to_string());

                            if let Some(online) = &session {
                                online.send(NetworkCommand::BroadcastMove(from, to));
                            }

                            update_game_over_state(&game, &mut game_over, &mut winner);
                        } else if let Some(online) = &session {
                            online.send(NetworkCommand::RejectMove("Illegal move".to_string()));
                        }
                    } else if game.make_move(from, to).is_ok() {
                        move_history.add_move(from, to);
                        status_message = Some("Move synced".to_string());
                        update_game_over_state(&game, &mut game_over, &mut winner);
                    }
                }
                NetworkEvent::InvalidMove(reason) => {
                    status_message = Some(format!("Move rejected: {}", reason));
                }
                NetworkEvent::Disconnected(reason) | NetworkEvent::Error(reason) => {
                    status_message = Some(reason);
                    connection_ready = false;
                }
            }
        }

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
        if board_perspective == White {
            camera.rotation = 0.0;
            camera.zoom = vec2(2.0 / screen_width(), -2.0 / screen_height());
        } else {
            camera.rotation = 0.0;
            camera.zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());
        }

        set_camera(&camera);

        board_frame.draw(tile_size);
        draw_board(
            &game,
            &light_tile,
            &dark_tile,
            &gate_textures.tex_vector,
            tile_size,
        );

        // Draw highlighted column before pieces
        draw_highlighted_column(tile_size);

        // Update and draw pieces with animations
        draw_pieces(
            &game,
            &piece_textures,
            &camera,
            tile_size,
            &mut piece_anim_state,
            now as f32,
            board_perspective,
        );
        draw_selected(tile_size);

        set_default_camera();

        // Draw row numbers after resetting camera (so they're not affected by board rotation)
        draw_row_numbers(board_perspective, tile_size);

        // Process clicks only if game is not over
        let local_turn = can_interact(&session, connection_ready, game.current_turn);

        if !game_over && local_turn {
            // Process keyboard input first
            if let Some((from, to)) = process_keyboard_input(&game) {
                try_local_move(
                    &mut game,
                    &mut move_history,
                    &session,
                    &mut game_over,
                    &mut winner,
                    &mut status_message,
                    from,
                    to,
                );
                unsafe {
                    SELECTED = None;
                    HIGHLIGHTED_COLUMN = None;
                    TYPING_MODE = false;
                }
            }

            // Then process mouse clicks
            if let Some((from, to)) = process_click(&game, &camera, tile_size) {
                try_local_move(
                    &mut game,
                    &mut move_history,
                    &session,
                    &mut game_over,
                    &mut winner,
                    &mut status_message,
                    from,
                    to,
                );
                unsafe {
                    SELECTED = None;
                }
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

        if let Some(message) = &status_message {
            let title = if connection_ready {
                "Online Play"
            } else {
                "Connection"
            };
            draw_status(title, message);
        }

        next_frame().await;
    }
}

fn draw_board(
    game: &Game,
    light: &Texture2D,
    dark: &Texture2D,
    gates: &[Texture2D],
    tile_size: f32,
) {
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
    perspective: PieceColor,
) {
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = game.board[row][col].piece {
                // Try to get animation first, fall back to static texture
                let tex = if let Some(frames) =
                    textures.get_animation(piece.kind, piece.color, AnimationState::Idle)
                {
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
                let draw_pos = vec2(
                    col as f32 * current_tile_size,
                    row as f32 * current_tile_size,
                );

                // Flip the position for black pieces relative to camera
                let rotation = match perspective {
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

fn process_click(
    _game: &Game,
    camera: &Camera2D,
    current_tile_size: f32,
) -> Option<(Position, Position)> {
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

        let pos = Position {
            row: row as usize,
            col: col as usize,
        };

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

fn process_keyboard_input(_game: &Game) -> Option<(Position, Position)> {
    unsafe {
        // Check for Escape to cancel everything (like vim)
        if is_key_pressed(KeyCode::Escape) {
            SELECTED = None;
            TYPING_MODE = false;
            HIGHLIGHTED_COLUMN = None;
            return None;
        }

        // Check for Enter key to enable typing mode
        if is_key_pressed(KeyCode::Enter) {
            if let Some(_) = SELECTED {
                TYPING_MODE = true;
                HIGHLIGHTED_COLUMN = None;
            }
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
    if is_key_pressed(KeyCode::A) {
        return Some(0);
    }
    if is_key_pressed(KeyCode::B) {
        return Some(1);
    }
    if is_key_pressed(KeyCode::C) {
        return Some(2);
    }
    if is_key_pressed(KeyCode::D) {
        return Some(3);
    }
    if is_key_pressed(KeyCode::E) {
        return Some(4);
    }
    if is_key_pressed(KeyCode::F) {
        return Some(5);
    }
    if is_key_pressed(KeyCode::G) {
        return Some(6);
    }
    if is_key_pressed(KeyCode::H) {
        return Some(7);
    }
    None
}

fn process_chess_row() -> Option<usize> {
    if is_key_pressed(KeyCode::Key1) {
        return Some(0);
    } // Row 1 = index 0 (top)
    if is_key_pressed(KeyCode::Key2) {
        return Some(1);
    }
    if is_key_pressed(KeyCode::Key3) {
        return Some(2);
    }
    if is_key_pressed(KeyCode::Key4) {
        return Some(3);
    }
    if is_key_pressed(KeyCode::Key5) {
        return Some(4);
    }
    if is_key_pressed(KeyCode::Key6) {
        return Some(5);
    }
    if is_key_pressed(KeyCode::Key7) {
        return Some(6);
    }
    if is_key_pressed(KeyCode::Key8) {
        return Some(7);
    } // Row 8 = index 7 (bottom)
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
                    Color::from_rgba(100, 150, 255, 60),
                );
            }
        }
    }
}

fn draw_row_numbers(perspective: PieceColor, current_tile_size: f32) {
    unsafe {
        let show_numbers = if let Some(_) = SELECTED {
            true
        } else {
            TYPING_MODE
        };

        if show_numbers {
            let font_size = current_tile_size * 0.4;
            let screen_width = screen_width();
            let screen_height = screen_height();
            let board_size = current_tile_size * 8.0;

            // Calculate board position on screen
            let board_left = (screen_width - board_size) / 2.0;
            let board_top = (screen_height - board_size) / 2.0;

            let offset_x = board_left - current_tile_size * 0.5; // Position to the left of the board

            for row in 0..8 {
                // Flip row numbering based on current turn
                let number = if perspective == White {
                    (8 - row).to_string() // White's perspective: 8 at top, 1 at bottom
                } else {
                    (row + 1).to_string() // Black's perspective: 1 at top, 8 at bottom
                };

                let y_pos =
                    board_top + (row as f32 * current_tile_size) + (current_tile_size * 0.5);

                draw_text(&number, offset_x, y_pos, font_size, WHITE);
            }
        }
    }
}

fn board_perspective(session: &Option<OnlineSession>, current_turn: PieceColor) -> PieceColor {
    session
        .as_ref()
        .map(|online| online.local_color())
        .unwrap_or(current_turn)
}

fn can_interact(
    session: &Option<OnlineSession>,
    connection_ready: bool,
    current_turn: PieceColor,
) -> bool {
    match session {
        Some(online) => connection_ready && online.local_color() == current_turn,
        None => true,
    }
}

fn update_game_over_state(game: &Game, game_over: &mut bool, winner: &mut Option<PieceColor>) {
    if game.is_checkmate(game.current_turn) {
        *game_over = true;
        *winner = Some(if game.current_turn == White {
            Black
        } else {
            White
        });
    } else if game.is_stalemate(game.current_turn) {
        *game_over = true;
        *winner = None;
    }
}

fn try_local_move(
    game: &mut Game,
    move_history: &mut MoveHistory,
    session: &Option<OnlineSession>,
    game_over: &mut bool,
    winner: &mut Option<PieceColor>,
    status_message: &mut Option<String>,
    from: Position,
    to: Position,
) {
    match session.as_ref().map(|online| online.role()) {
        Some(SessionRole::Client) => {
            if game.get_legal_moves(from).contains(&to) {
                if let Some(online) = session {
                    online.send(NetworkCommand::SubmitMove(from, to));
                    *status_message = Some("Move sent to host".to_string());
                }
            } else {
                *status_message = Some("Illegal move".to_string());
            }
        }
        Some(SessionRole::Host) => {
            if game.make_move(from, to).is_ok() {
                move_history.add_move(from, to);
                update_game_over_state(game, game_over, winner);

                if let Some(online) = session {
                    online.send(NetworkCommand::BroadcastMove(from, to));
                }

                *status_message = Some("Move sent to opponent".to_string());
            } else {
                *status_message = Some("Illegal move".to_string());
            }
        }
        Some(SessionRole::Local) | None => {
            if game.make_move(from, to).is_ok() {
                move_history.add_move(from, to);
                update_game_over_state(game, game_over, winner);
            } else {
                *status_message = Some("Illegal move".to_string());
            }
        }
    }
}

fn draw_selected(current_tile_size: f32) {
    unsafe {
        if let Some(pos) = SELECTED {
            // Use different color based on typing mode
            let color = if TYPING_MODE {
                Color::from_rgba(0, 255, 0, 100) // Green in typing mode
            } else {
                Color::from_rgba(255, 255, 0, 80) // Yellow normally
            };

            draw_rectangle(
                pos.col as f32 * current_tile_size,
                pos.row as f32 * current_tile_size,
                current_tile_size,
                current_tile_size,
                color,
            );
        }
    }
}
