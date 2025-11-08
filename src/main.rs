// =======================================================
// Project: GatedChess
// File: main.rs
// Description: Enables GatedChess execution.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

mod board;   // Loads board module
mod pieces;  // Loads pieces module
mod gates;   // Loads gates module

use crate::gates::{ update_gates, GateType };
use crate::board::{ create_board, print_board };

use std::io::{self};

const BOARD_SIZE: usize = 8;

fn main() {
    let mut board = create_board();

    loop {
        print_board(&board);

        println!("Enter a move (e.g., 1 0 2 0 to move from row1,col0 to row2,col0):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coords: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        if coords.len() != 4 {
            println!("invalid input, please enter 4 numbers.");
            continue;
        }

        let (from_row, from_col, to_row, to_col) = (coords[0], coords[1], coords[2], coords[3]);

        if let Some(piece) = board[from_row][from_col].piece.take() {
            board[to_row][to_col].piece = Some(piece);

            board[to_row][to_col].gate = Some(GateType::Standard { duration: 1 });
        } else {
            println!("No piece at the starting square!");
        }

        update_gates(&mut board);
    }
}