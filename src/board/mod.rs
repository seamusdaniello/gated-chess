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

use crate::pieces::{ PieceType, Color, Piece }; // Import from pieces module
use crate::config::BOARD_SIZE; // Import board size constant
use crate::gates::{ GateType };

pub fn create_board() -> [[Square; BOARD_SIZE]; BOARD_SIZE] {
    let mut board = [[Square::new(); BOARD_SIZE]; BOARD_SIZE];

    // Place White pawns
    for i in 0..BOARD_SIZE {
        board[1][i].piece = Some(Piece::new(crate::pieces::PieceType::Pawn, crate::pieces::Color::White));
    }

    // Place Black pawns
    for i in 0..BOARD_SIZE {
        board[6][i].piece = Some(Piece::new(crate::pieces::PieceType::Pawn, crate::pieces::Color::Black));
    }

    // Place other pieces
    let piece_order = [
        crate::pieces::PieceType::Rook,
        crate::pieces::PieceType::Knight,
        crate::pieces::PieceType::Bishop,
        crate::pieces::PieceType::Queen,
        crate::pieces::PieceType::King,
        crate::pieces::PieceType::Bishop,
        crate::pieces::PieceType::Knight,
        crate::pieces::PieceType::Rook,
    ];

    for (i, &kind) in piece_order.iter().enumerate() {
        board[0][i].piece = Some(Piece::new(kind, crate::pieces::Color::White));
        board[7][i].piece = Some(Piece::new(kind, crate::pieces::Color::Black));
    }

    board
}

pub fn print_board(board: &[[Square; BOARD_SIZE]; BOARD_SIZE]) {
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
                    GateType::Standard { duration } => print!("S"),
                    GateType::Decay { duration } => print!("D"),
                }
            } else {
                print!(". "); // empty square
            }
        }
        println!();
    }
    println!();
}