// =======================================================
// Project: GatedChess
// File: game.rs
// Description: Defines game rules.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

impl Game {
    pub(super) fn leaves_king_in_check(&self, from: Position, to: Position, color: Color) -> bool {
        let mut temp_game = self.clone();

        temp_game.make_move_unchecked(from, to);

        temp_game.is_king_in_check(color)
    }
}