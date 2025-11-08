// =======================================================
// Project: GatedChess
// File: generation.rs
// Description: Generates legal moves for pieces.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

use crate::game::{Game, Position};
use crate::pieces::{Color, PieceType};

pub fn get_piece_moves(game: &Game, pos: Position, color: Color) -> Vec<Position> {
    if let Some(piece) = game.board[pos.row][pos.col].piece {
        if piece.color != color {
            return Vec::new();
        }

        match piece.kind {
            PieceType::Pawn => {
                let mut moves = game.pawn_moves(pos, color);
                moves.extend(game.pawn_attacks(pos, color));
                moves
            }
            PieceType::Rook => game.rook_moves(pos, color),
            PieceType::Knight => game.knight_moves(pos, color),
            PieceType::Bishop => game.bishop_moves(pos, color),
            PieceType::Queen => game.queen_moves(pos, color),
            PieceType::King => game.king_moves(pos, color),
        }
    } else {
        Vec::new()
    }
}