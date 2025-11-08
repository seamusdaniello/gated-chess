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

pub struct Square {
    pub piece: Option<Piece>,
    pub gate: Option<GateType>,
}

impl Square {
    fn new() -> Square {
        Square {
            piece: None,
            gate: None,
        }
    }
}