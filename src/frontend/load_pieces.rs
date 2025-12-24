use macroquad::prelude::*;
use crate::pieces::Color;
use crate::pieces::PieceType;
use crate::pieces::piece_animations::loaders::bishop_loader;
use std::collections::HashMap;

pub struct PieceTextures {
    pub textures: HashMap<(PieceType, Color), Texture2D>,
    pub animations: HashMap<(PieceType, Color, String), Vec<Texture2D>>, // Changed to String
}

impl PieceTextures {
    pub async fn load() -> Self {
        let mut textures = HashMap::new();
        let mut animations = HashMap::new();

        // Base textures (static fallback images)
        let base = vec![
            (PieceType::Pawn, Color::Black,  "images/black/pawn.png"),
            (PieceType::Pawn, Color::White,  "images/white/pawn.png"),
            (PieceType::Bishop, Color::Black, "images/black/bishop.png"),
            (PieceType::Bishop, Color::White, "images/white/bishop.png"),
            (PieceType::Knight, Color::Black, "images/black/knight.png"),
            (PieceType::Knight, Color::White, "images/white/knight.png"),
            (PieceType::Rook, Color::Black,   "images/black/rook.png"),
            (PieceType::Rook, Color::White,   "images/white/rook.png"),
            (PieceType::King, Color::Black,   "images/black/king.png"),
            (PieceType::King, Color::White,   "images/white/king.png"),
            (PieceType::Queen, Color::Black,  "images/black/queen.png"),
            (PieceType::Queen, Color::White,  "images/white/queen.png"),
        ];

        for (kind, color, path) in base {
            let tex = load_texture(path).await.unwrap();
            tex.set_filter(FilterMode::Nearest);
            textures.insert((kind, color), tex);
        }

        // Load animations using the dedicated loaders
        // White Bishop Idle Animation
        let white_bishop_idle = bishop_loader::load_bishop_frames("white").await;
        if !white_bishop_idle.is_empty() {
            animations.insert((PieceType::Bishop, Color::White, "idle".to_string()), white_bishop_idle);
        }

        // Black Bishop Idle Animation (if you have it)
        // let black_bishop_idle = bishop_loader::load_bishop_frames("black").await;
        // if !black_bishop_idle.is_empty() {
        //     animations.insert((PieceType::Bishop, Color::Black, "idle".to_string()), black_bishop_idle);
        // }

        Self { textures, animations }
    }

    pub fn get(&self, kind: PieceType, color: Color) -> Option<&Texture2D> {
        self.textures.get(&(kind, color))
    }

    pub fn get_animation(
        &self, 
        kind: PieceType, 
        color: Color, 
        state: &str
    ) -> Option<&Vec<Texture2D>> {
        self.animations.get(&(kind, color, state.to_string())) // Convert to String for lookup
    }
}