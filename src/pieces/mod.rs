// =======================================================
// Project: GatedChess
// File: pieces/mod.rs
// Description: Defines pieces used in-game.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub kind: PieceType,
    pub color: Color,
    pub has_moved: Option<bool>,
}

impl Piece {
    pub fn new(kind: PieceType, color: Color) -> Piece {
        Piece {
            kind,
            color,
            has_moved: Some(false), 
        }
    }
}