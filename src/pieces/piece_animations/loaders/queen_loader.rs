// =======================================================
// Project: GatedChess
// File: piece_animations/loaders/queen_loader.rs
// Description: Loads all animations for queen FSMs.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-12-24
// License: MIT
// =======================================================

use macroquad::prelude::*;

pub async fn load_queen_frames(color: &str) -> Vec<Texture2D> {
    let mut frames = Vec::new();

    for i in 1..10 {
        let path = format!("images/{}/queen/queen-{}.png", color, i);
        let tex = load_texture(&path).await.unwrap();
        tex.set_filter(FilterMode::Nearest);
        frames.push(tex);
    }

    frames
}