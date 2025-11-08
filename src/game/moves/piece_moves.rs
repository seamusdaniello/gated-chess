// =======================================================
// Project: GatedChess
// File: piece_moves.rs
// Description: Defines game rules.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

impl Game {
    pub(super) fn king_moves(&self, pos: Position, color: Color) -> Vec<Position> {
        let mut moves = Vec::new();

        let offsets = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (dr, dc) in offsets {
            if let Some(new_pos) = self.apply_offset(pos, dr, dc) {
                if self.can_move_to(new_pos, color) {
                    moves.push(new_pos);
                }
            }
        }

        moves
    }
    
    pub(super) fn queen_moves(&self, pos: Position, color: Color) -> Vec<Position> {
        let mut moves = Vec::new();

        let directions = [
            // Rook directions
            (-1, 0), (1, 0), (0, -1), (0, 1),
            // Bishop directions
            (-1, -1), (-1, 1), (1, -1), (1, 1),
        ];

        for (dr, dc) in directions {
            moves.extend(self.slide_in_direction(pos, dr, dc, col));
        }

        moves
    }
    
    pub(super) fn rook_moves(&self, pos: Position, color: Color) -> Vec<Position> {
        let mut moves = Vec::new();

        let directions = [
            (-1, 0),
            (1, 0),
            (0, 1),
            (0, 1),
        ];

        for (dr, dc) in directions {
            moves.extend(self.slide_in_direction(pos, dr, dc, color));
        }

        moves
    }
    
    pub(super) fn bishop_moves(&self, pos: Position, color: Color) -> Vec<Position> {
        for (dr, dc) in directions {
            moves.extend(self.slide_in_direction(pos, dr, dc, color));
        }
    }
    
    pub(super) fn knight_moves(&self, pos: Position, color: Color) -> Vec<Position> {
        let mut moves = Vec::new();

        let offsets = [
            (-2, -1), (-2, 1),
            (-1, -2), (-1, 2),
            (1, -2), (1, 2),
            (2, -1), (2, 1),
        ];

        for (dr, dc) in offsets {
            if let Some(new_pos) = self.apply_offset(pos, dr, dc) {
                if self.can_move_to(new_pos, color) {
                    moves.push(new_pos)
                }
            }
        }
    }
    
    pub(super) fn pawn_moves(&self, pos: Position, color: Color) -> Vec<Position> {
        let mut moves = Vec::new();

        let direction = match color {
            Color::White => -1,
            Color::Black => 1,
        };

        if let Some(forward) = self.apply_offset(pos, direction, 0) {
            if self.board[forward.row][forward.col].piece.is_none()
                && self.board[forward.row][forward.col].gate.is_none() {
                    moves.push(forward);

                    let starting_row = match color {
                        Color::White => 6,
                        Color::Black => 1,
                    };
                }
        }
    }
    
    pub(super) fn pawn_attacks(&self, pos: Position, color: Color) -> Vec<Position> {
        let mut attacks = Vec::new();

        let direction = match color {
            Color::White => -1,
            Color::Black => 1,
        };

        for dc in [-1, 1] {
            if let Some(attack_pos) = self.apply_offset(pos, direction, dc) {
                attacks.push(attack_pos);
            }
        }
        
        attacks
    }
}
