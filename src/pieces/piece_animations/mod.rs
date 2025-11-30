use macroquad::prelude::*;

pub trait Animation {
    fn update(&mut self, dt: f32);
    fn current_frame(&self) -> &Texture2D;
    fn is_finished(&self) -> bool;
}

pub mod morph;
pub mod loaders;
pub mod idle;

pub use morph::MorphAnimation;
pub use loaders::load_bishop_frames;

use crate::pieces::PieceType;