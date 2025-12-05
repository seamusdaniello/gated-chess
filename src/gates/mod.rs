// =======================================================
// Project: GatedChess
// File: mod.rs
// Description: Defines gate creation and updates.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

pub mod gate_type;
pub mod logic;
pub mod actions;

pub use gate_type::GateType;
pub use logic::{update_gates, update_gate_animation};
