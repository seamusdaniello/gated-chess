// =======================================================
// Project: GatedChess
// File: piece_animations/morph.rs
// Description: Defines individual morph animations for piece FSMs.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-30
// License: MIT
// =======================================================

use macroquad::prelude::*;
use crate::pieces::Color;
use super::Animation;

pub struct MorphAnimation {
    frames: Vec<Texture2D>,
    current_index: usize,
    frame_timer: f32,
    frame_duration: f32,
    finished: bool,
}

impl MorphAnimation {
    pub async fn load_bishop(color: Color) -> Self {
        let mut frames = Vec::new();

        let base = match color {
            Color::White => "images/white/bishop/morph_",
            Color::Black => "images/black/bishop/morph_",
        };

        for i in 1..=12 {
            let path = format!("{}{}.png", base, i);
            let tex = load_texture(&path).await.unwrap();
            tex.set_filter(FilterMode::Nearest);
            frames.push(tex);
        }

        Self {
            frames,
            current_index: 0,
            frame_timer: 0.0,
            frame_duration: 0.08,
            finished: false,
        }
    }
}

impl Animation for MorphAnimation {
    fn update(&mut self, dt: f32) {
        if self.finished { return; }

        self.frame_timer += dt;

        if self.frame_timer >= self.frame_duration {
            self.frame_timer = 0.0;
            self.current_index += 1;
            
            if self.current_index >= self.frames.len() {
                self.current_index = self.frames.len() - 1;
                self.finished = true;
            }
        }
    }

    fn current_frame(&self) -> &Texture2D {
        &self.frames[self.current_index]
    }

    fn is_finished(&self) -> bool {
        self.finished
    }
}