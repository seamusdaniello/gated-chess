// =======================================================
// Project: GatedChess
// File: mod.rs
// Description: Defines gate creation and updates.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

use crate::game::Game;
use crate::gates::GateType;

pub fn update_gates(game:&mut Game) {
    for row in game.board.iter_mut() {
        for square in row.iter_mut() {
            if let Some(gate) = square.gate {
                // Decrement duration
                match gate {
                    GateType::Standard { duration } => {
                        if duration > 0 {
                            let new_duration = duration - 1;
                            if new_duration == 0 {
                                square.gate = None;
                            } else {
                                square.gate = Some(GateType::Standard { duration: new_duration });
                            }
                        }
                    }
                    GateType::Decay { duration } => {
                        if duration > 0 {
                            let new_duration = duration - 1;
                            if new_duration == 0 {
                                square.gate = None;
                            } else {
                                square.gate = Some(GateType::Decay { duration: new_duration });
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn update_gate_animation(game: &mut Game) {
    for row in game.board.iter_mut() {
        for square in row.iter_mut() {
            if square.gate.is_some() {
                // Initialize if None
                let frame = square.animation_frame.get_or_insert(0);
                let dir = square.animation_direction.get_or_insert(1);

                let new_frame = *frame as i8 + *dir;
                *frame = new_frame.clamp(0, 7) as usize; // ensure within bounds

                // Update direction at ends
                if new_frame >= 7 {
                    *dir = -1;
                } else if new_frame <= 0 {
                    *dir = 1;
                }
            }
        }
    }
}

