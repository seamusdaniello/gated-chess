// =======================================================
// Project: GatedChess
// File: gate_type.rs
// Description: Defines various gate types used in-game.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

#[derive(Copy, Clone, Debug)]
pub enum GateType {
    Standard { duration: u8 },
    Decay { duration: u8 },
}