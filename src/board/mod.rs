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

use crate::config::BOARD_SIZE;
use crate::pieces::Piece; // Import board size constant

pub fn create_board() -> [[Square; BOARD_SIZE]; BOARD_SIZE] {
    let mut board = [[Square::new(); BOARD_SIZE]; BOARD_SIZE];

    // Place White pawns
    for i in 0..BOARD_SIZE {
        board[1][i].piece = Some(Piece::new(
            crate::pieces::PieceType::Pawn,
            crate::pieces::Color::White,
        ));
    }

    // Place Black pawns
    for i in 0..BOARD_SIZE {
        board[6][i].piece = Some(Piece::new(
            crate::pieces::PieceType::Pawn,
            crate::pieces::Color::Black,
        ));
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
