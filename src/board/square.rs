// =======================================================
// Project: GatedChess
// File: square.rs
// Description: Defines the square object and implementation.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

use crate::pieces::{ Piece }; // Import from pieces module
use crate::gates::{ GateType };

#[derive(Clone, Copy, Debug)]
pub struct Square {
    pub piece: Option<Piece>,
    pub gate: Option<GateType>,
    pub animation_frame: Option<usize>,
    pub animation_direction: Option<i8>,
}

impl Square {
    pub fn new() -> Square {
        Square {
            piece: None,
            gate: None,
            animation_frame: None,
            animation_direction: None,
        }
    }
}