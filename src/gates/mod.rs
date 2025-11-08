// =======================================================
// Project: GatedChess
// File: mod.rs
// Description: Defines gate creation and updates.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

use crate::config::BOARD_SIZE;
use crate::board::Square;

#[derive(Clone, Copy, Debug)]
pub enum GateType {
    Standard { duration: u8 },
    Decay { duration: u8 },
}

pub fn update_gates(board: &mut [[Square; BOARD_SIZE]; BOARD_SIZE]) {
    for row in board.iter_mut() {
        for square in row.iter_mut() {
            if let Some(gate) = square.gate {
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