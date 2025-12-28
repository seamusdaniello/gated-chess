use macroquad::prelude::*;
use crate::pieces::{Color, PieceType};
use crate::pieces::piece_animations::loaders::bishop_loader;
use crate::pieces::piece_animations::loaders::pawn_loader;
use crate::pieces::piece_animations::loaders::knight_loader;
use crate::pieces::piece_animations::loaders::rook_loader;
use crate::pieces::piece_animations::loaders::queen_loader;
use crate::pieces::piece_animations::loaders::king_loader;
use std::collections::HashMap;

/// Represents different animation states for pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationState {
    Idle,
    Moving,
    Capturing,
    Selected,
}

impl AnimationState {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnimationState::Idle => "idle",
            AnimationState::Moving => "moving",
            AnimationState::Capturing => "capturing",
            AnimationState::Selected => "selected",
        }
    }
}

impl From<&str> for AnimationState {
    fn from(s: &str) -> Self {
        match s {
            "idle" => AnimationState::Idle,
            "moving" => AnimationState::Moving,
            "capturing" => AnimationState::Capturing,
            "selected" => AnimationState::Selected,
            _ => AnimationState::Idle,
        }
    }
}

pub struct PieceTextures {
    pub textures: HashMap<(PieceType, Color), Texture2D>,
    animations: HashMap<(PieceType, Color, AnimationState), Vec<Texture2D>>,
}

impl PieceTextures {
    pub async fn load() -> Self {
        let mut textures = HashMap::new();
        let mut animations = HashMap::new();

        // Base textures (static fallback images)
        let base_pieces = [
            (PieceType::Pawn, Color::Black, "images/black/pawn.png"),
            (PieceType::Pawn, Color::White, "images/white/pawn.png"),
            (PieceType::Bishop, Color::Black, "images/black/bishop.png"),
            (PieceType::Bishop, Color::White, "images/white/bishop.png"),
            (PieceType::Knight, Color::Black, "images/black/knight.png"),
            (PieceType::Knight, Color::White, "images/white/knight.png"),
            (PieceType::Rook, Color::Black, "images/black/rook.png"),
            (PieceType::Rook, Color::White, "images/white/rook.png"),
            (PieceType::King, Color::Black, "images/black/king.png"),
            (PieceType::King, Color::White, "images/white/king.png"),
            (PieceType::Queen, Color::Black, "images/black/queen.png"),
            (PieceType::Queen, Color::White, "images/white/queen.png"),
        ];

        for (kind, color, path) in base_pieces {
            match load_texture(path).await {
                Ok(tex) => {
                    tex.set_filter(FilterMode::Nearest);
                    textures.insert((kind, color), tex);
                }
                Err(e) => {
                    eprintln!("Failed to load texture {}: {}", path, e);
                }
            }
        }

        // Load animations using the dedicated loaders
        Self::load_bishop_animations(&mut animations).await;
        Self::load_pawn_animations(&mut animations).await;
        Self::load_knight_animations(&mut animations).await;
        Self::load_rook_animations(&mut animations).await;
        Self::load_queen_animations(&mut animations).await;
        Self::load_king_animations(&mut animations).await;
        
        // Add more animation loaders here as you implement them:
        // Self::load_knight_animations(&mut animations).await;
        // Self::load_queen_animations(&mut animations).await;

        Self { textures, animations }
    }

    async fn load_bishop_animations(
        animations: &mut HashMap<(PieceType, Color, AnimationState), Vec<Texture2D>>
    ) {
        // White Bishop Idle Animation
        let white_bishop_idle = bishop_loader::load_bishop_frames("white").await;
        if !white_bishop_idle.is_empty() {
            animations.insert(
                (PieceType::Bishop, Color::White, AnimationState::Idle),
                white_bishop_idle
            );
        }

        // White Bishop Idle Animation
        let white_bishop_idle = bishop_loader::load_bishop_frames("white").await;
        if !white_bishop_idle.is_empty() {
            animations.insert(
                (PieceType::Bishop, Color::White, AnimationState::Idle),
                white_bishop_idle
            );
        }

        // Black Bishop Idle Animation
        let black_bishop_idle = bishop_loader::load_bishop_frames("black").await;
        if !black_bishop_idle.is_empty() {
            animations.insert(
                (PieceType::Bishop, Color::Black, AnimationState::Idle),
                black_bishop_idle
            );
        }

        // Black Bishop Idle Animation
        // let black_bishop_idle = bishop_loader::load_bishop_frames("black").await;
        // if !black_bishop_idle.is_empty() {
        //     animations.insert(
        //         (PieceType::Bishop, Color::Black, AnimationState::Idle),
        //         black_bishop_idle
        //     );
        // }
    }

    async fn load_pawn_animations(
        animations: &mut HashMap<(PieceType, Color, AnimationState), Vec<Texture2D>>
    ) {
        // White Pawn Idle Animation
        let white_pawn_idle = pawn_loader::load_pawn_frames("white").await;
        if !white_pawn_idle.is_empty() {
            animations.insert(
                (PieceType::Pawn, Color::White, AnimationState::Idle),
                white_pawn_idle
            );
        }

        let black_pawn_idle = pawn_loader::load_pawn_frames("black").await;
        if !black_pawn_idle.is_empty() {
            animations.insert(
                (PieceType::Pawn, Color::Black, AnimationState::Idle),
                black_pawn_idle
            );
        }
    }

    async fn load_knight_animations(
        animations: &mut HashMap<(PieceType, Color, AnimationState), Vec<Texture2D>>
    ) {
        // White Knight Idle Animation
        let white_knight_idle = knight_loader::load_knight_frames("white").await;
        if !white_knight_idle.is_empty() {
            animations.insert(
                (PieceType::Knight, Color::White, AnimationState::Idle),
                white_knight_idle
            );
        }
    }

    async fn load_rook_animations(
        animations: &mut HashMap<(PieceType, Color, AnimationState), Vec<Texture2D>>
    ) {
        // White Rook Idle Animation
        let white_rook_idle = rook_loader::load_rook_frames("white").await;
        if !white_rook_idle.is_empty() {
            animations.insert(
                (PieceType::Rook, Color::White, AnimationState::Idle),
                white_rook_idle
            );
        }
    }

    async fn load_queen_animations(
        animations: &mut HashMap<(PieceType, Color, AnimationState), Vec<Texture2D>>
    ) {
        // White Queen Idle Animation
        let white_queen_idle = queen_loader::load_queen_frames("white").await;
        if !white_queen_idle.is_empty() {
            animations.insert(
                (PieceType::Queen, Color::White, AnimationState::Idle),
                white_queen_idle
            );
        }
    }

    async fn load_king_animations(
        animations: &mut HashMap<(PieceType, Color, AnimationState), Vec<Texture2D>>
    ) {
        // White King Idle Animation
        let white_king_idle = king_loader::load_king_frames("white").await;
        if !white_king_idle.is_empty() {
            animations.insert(
                (PieceType::King, Color::White, AnimationState::Idle),
                white_king_idle
            );
        }
    }

    /// Get the base texture for a piece (fallback if no animation)
    pub fn get(&self, kind: PieceType, color: Color) -> Option<&Texture2D> {
        self.textures.get(&(kind, color))
    }

    /// Get animation frames for a piece in a specific state
    pub fn get_animation(
        &self,
        kind: PieceType,
        color: Color,
        state: AnimationState
    ) -> Option<&Vec<Texture2D>> {
        self.animations.get(&(kind, color, state))
    }

    /// Get animation frames by string (for backward compatibility)
    pub fn get_animation_by_str(
        &self,
        kind: PieceType,
        color: Color,
        state: &str
    ) -> Option<&Vec<Texture2D>> {
        self.get_animation(kind, color, AnimationState::from(state))
    }

    /// Check if an animation exists for a piece
    pub fn has_animation(
        &self,
        kind: PieceType,
        color: Color,
        state: AnimationState
    ) -> bool {
        self.animations.contains_key(&(kind, color, state))
    }

    /// Get the number of frames in an animation
    pub fn animation_frame_count(
        &self,
        kind: PieceType,
        color: Color,
        state: AnimationState
    ) -> usize {
        self.animations
            .get(&(kind, color, state))
            .map_or(0, |frames| frames.len())
    }
}

/// Helper struct for managing piece animations during gameplay
pub struct PieceAnimator {
    current_frame: usize,
    frame_timer: f32,
    frame_duration: f32,
    looping: bool,
    finished: bool,
}

impl PieceAnimator {
    pub fn new(frame_duration: f32, looping: bool) -> Self {
        Self {
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration,
            looping,
            finished: false,
        }
    }

    pub fn update(&mut self, delta_time: f32, frame_count: usize) {
        if self.finished && !self.looping {
            return;
        }

        self.frame_timer += delta_time;

        if self.frame_timer >= self.frame_duration {
            self.frame_timer -= self.frame_duration;
            self.current_frame += 1;

            if self.current_frame >= frame_count {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = frame_count.saturating_sub(1);
                    self.finished = true;
                }
            }
        }
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.frame_timer = 0.0;
        self.finished = false;
    }

    pub fn get_texture<'a>(
        &self,
        textures: &'a PieceTextures,
        kind: PieceType,
        color: Color,
        state: AnimationState
    ) -> Option<&'a Texture2D> {
        textures
            .get_animation(kind, color, state)
            .and_then(|frames| frames.get(self.current_frame))
    }
}