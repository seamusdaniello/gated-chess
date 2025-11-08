// =======================================================
// Project: GatedChess
// File: game.rs
// Description: Defines game rules.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

use std::sync::PoisonError;

impl Game {
    pub fn make_move(&mut self, from: Position, to: Position) -> Result<(), MoveError> {
        let legal_moves = self.get_legal_moves(from);
        if !legal_moves.contains(&to) {
            return Err(MoveError::IllegalMove);
        }

        self.make_move_unchecked(from, to);
        self.switch_turn();
        Ok(());
    }

    pub(crate) fn make_move_unchecked(&mut self, from: Position, to: Position) {
        self.board[to.row][to.col].piece = self.board[from.row][from.col].piece.take();
    }
}
