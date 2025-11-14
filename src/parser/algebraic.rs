// =======================================================
// Project: GatedChess
// File: algebraic.rs
// Description: Converts chess positions into grid values for the board.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

use crate::game::Position;

pub fn parse_square(input: &str) -> Option<Position> {
    if input.len() != 2 { return None; }

    let col = input.chars().nth(0)?.to_ascii_lowercase();
    let row = input.chars().nth(1)?;

    let col_index  = (col as u8).wrapping_sub(b'a') as usize;
    let row_index = (row.to_digit(10)? - 1) as usize;

    Position::new(row_index, col_index)
}

pub fn parse_move(input: &str) -> Option<(Position, Position)> {
    let input = input.trim();
    if input.len() != 4 { return None; }

    let from = parse_square(&input[0..2])?;
    let to = parse_square(&input[2..4])?;

    Some((from, to))
}