use macroquad::prelude::*;
use crate::pieces::Color;
use crate::pieces::PieceType;
use std::collections::HashMap;
use std::fs;

pub struct PieceTextures {
    pub textures: HashMap<(PieceType, Color), Texture2D>,
    pub animations: HashMap<(PieceType, Color), Vec<Texture2D>>,
}

impl PieceTextures {
    pub async fn load() -> Self {
        let mut textures = HashMap::new();
        let mut animations = HashMap::new();

        // Define which folders we expect to contain animations (for now just bishop)
        let animated_pieces = vec![(PieceType::Bishop, Color::White)];

        // Base textures (same as before)
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
            textures.insert((kind, color), tex);
        }

        // Load bishop animation frames
        for (kind, color) in animated_pieces {
            let folder = match color {
                Color::White => format!("images/white/{:?}/", kind).to_lowercase(),
                Color::Black => format!("images/black/{:?}/", kind).to_lowercase(),
            };

            if let Ok(files) = fs::read_dir(&folder) {
                let mut frames = vec![];
                let mut entries: Vec<_> = files.map(|f| f.unwrap().path()).collect();

                // Sort numerically so 0.png,1.png,... stays in order
                entries.sort();

                for file in entries {
                    if file.extension().and_then(|x| x.to_str()) == Some("png") {
                        let tex = load_texture(file.to_str().unwrap()).await.unwrap();
                        frames.push(tex);
                    }
                }

                if !frames.is_empty() {
                    animations.insert((kind, color), frames);
                }
            }
        }

        Self { textures, animations }
    }

    pub fn get(&self, kind: PieceType, color: Color) -> Option<&Texture2D> {
        self.textures.get(&(kind, color))
    }

    pub fn get_animation(&self, kind: PieceType, color: Color) -> Option<&Vec<Texture2D>> {
        self.animations.get(&(kind, color))
    }
}