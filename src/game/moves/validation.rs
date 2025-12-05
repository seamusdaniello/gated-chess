// =======================================================
// Project: GatedChess
// File: validation.rs
// Description: Validates moves and checks.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

use crate::game::state_machine::GameStateManager;
use crate::game::{Game, Position};
use crate::pieces::Color;

impl Game {
    pub(crate) fn leaves_king_in_check(&self, from: Position, to: Position, color: Color) -> bool {
        // Create a temporary board by copying
        let mut temp_board = self.board;
        
        // Make the move on the temporary board
        temp_board[to.row][to.col].piece = temp_board[from.row][from.col].piece.take();
        
        // Create temporary game to check for check
        let temp_game = Game {
            board: temp_board,
            state_manager: GameStateManager::new(),
            current_turn: self.current_turn,
            result: crate::game::GameResult::InProgress,
        };

        temp_game.is_king_in_check(color)
    }
}