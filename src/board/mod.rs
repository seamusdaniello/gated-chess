// =======================================================
// Project: GatedChess
// File: board/mod.rs
// Description: Defines board creation and modifications.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

pub mod square;
pub use square::Square;

use crate::pieces::{PieceType, Color}; // Import from pieces module
use crate::BOARD_SIZE; // Import board size constant
use crate::gates::{ GateType };

fn print_board(board: &[[Square; BOARD_SIZE]; BOARD_SIZE]) {
    println!();
    for row in board.iter().rev() { // Print from top (row 7) to bottom (row 0)
        for square in row.iter() {
            if let Some(piece) = square.piece {
                let symbol = match piece.kind {
                    PieceType::Pawn => "P",
                    PieceType::Rook => "R",
                    PieceType::Knight => "N",
                    PieceType::Bishop => "B",
                    PieceType::Queen => "Q",
                    PieceType::King => "K",
                };
                let display = match piece.color {
                    Color::White => symbol.to_uppercase(),
                    Color::Black => symbol.to_lowercase(),
                };
                print!("{} ", display);
            } else if let Some(gate) = square.gate {
                match gate {
                    GateType::Standard { duration } => print!("S "),
                    GateType::Decay { duration } => print!("D "),
                }
            } else {
                print!(". "); // empty square
            }
        }
        println!();
    }
    println!();
}