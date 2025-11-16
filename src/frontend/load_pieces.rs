use macroquad::prelude::*;
use crate::pieces::Color;
use crate::pieces::PieceType;
use std::collections::HashMap;

pub struct PieceTextures {
    pub textures: HashMap<(PieceType, Color), Texture2D>,
}

impl PieceTextures {
    pub async fn load() -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            (PieceType::Pawn, Color::Black),
            load_texture("images/black/pawn.png").await.unwrap(),
        );

        textures.insert(
            (PieceType::Pawn, Color::White),
            load_texture("images/white/pawn.png").await.unwrap(),
        );

        textures.insert(
            (PieceType::Bishop, Color::Black),
            load_texture("images/black/bishop.png").await.unwrap(),
        );

        textures.insert(
            (PieceType::Bishop, Color::White),
            load_texture("images/white/bishop.png").await.unwrap(),
        );

        textures.insert(
            (PieceType::Knight, Color::Black),
            load_texture("images/black/knight.png").await.unwrap()
        );

        textures.insert(
            (PieceType::Knight, Color::White),
            load_texture("images/white/knight.png").await.unwrap(),
        );

        textures.insert(
            (PieceType::Rook, Color::Black),
            load_texture("images/black/rook.png").await.unwrap(),
        );

        textures.insert(
            (PieceType::Rook, Color::White),
            load_texture("images/white/rook.png").await.unwrap(),
        );

        textures.insert(
            (PieceType::King, Color::Black),
            load_texture("images/black/king.png").await.unwrap(),
        );

        textures.insert(
            (PieceType::King, Color::White),
            load_texture("images/white/king.png").await.unwrap(),
        );

        textures.insert(
            (PieceType::Queen, Color::Black),
            load_texture("images/black/queen.png").await.unwrap(),
        );

        textures.insert(
            (PieceType::Queen, Color::White),
            load_texture("images/white/queen.png").await.unwrap(),
        );

        Self { textures }
    }

    pub fn get(&self, kind: PieceType, color: Color) -> Option<&Texture2D> {
        self.textures.get(&(kind, color))
    }
}