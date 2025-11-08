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
mod gates;
mod game;   // Loads game module
mod config; // Loads config module

use crate::gates::update_gates;
use crate::board::{create_board, print_board};
use crate::game::{Game, Position, MoveError, GameResult};
use crate::pieces::Color;

use std::io::{self};

fn main() {
    let board = create_board();
    let mut game = Game::new(board);

    loop {
        // Check if game is over
        match game.result {
            GameResult::Checkmate(winner) => {
                print_board(&game.board);
                let winner_str = match winner {
                    Color::White => "White",
                    Color::Black => "Black",
                };
                println!("Checkmate! {} wins!", winner_str);
                break;
            }
            GameResult::Stalemate => {
                print_board(&game.board);
                println!("Stalemate! The game is a draw.");
                break;
            }
            GameResult::InProgress => {
                // Continue with normal game loop
            }
        }
        
        print_board(&game.board);

        let turn_str = match game.current_turn {
            Color::White => "White",
            Color::Black => "Black",
        };
        println!("{}'s turn", turn_str);
        println!("Enter a move (e.g., 1 0 2 0 to move from row1,col0 to row2,col0):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coords: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        if coords.len() != 4 {
            println!("Invalid input, please enter 4 numbers (from_row from_col to_row to_col).");
            continue;
        }

        let (from_row, from_col, to_row, to_col) = (coords[0], coords[1], coords[2], coords[3]);

        let from_pos = match Position::new(from_row, from_col) {
            Some(pos) => pos,
            None => {
                println!("Invalid starting position! Row and column must be 0-7.");
                continue;
            }
        };

        let to_pos = match Position::new(to_row, to_col) {
            Some(pos) => pos,
            None => {
                println!("Invalid destination position! Row and column must be 0-7.");
                continue;
            }
        };

        // Use the game's make_move method which enforces all rules
        match game.make_move(from_pos, to_pos) {
            Ok(()) => {
                // Move was successful, update gates
                update_gates(&mut game.board);
                
                // Check message is now handled by check_game_state()
                // But we can still show check warnings
                let opponent_color = match game.current_turn {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                };
                if game.is_king_in_check(opponent_color) && game.result == GameResult::InProgress {
                    println!("Check!");
                }
            }
            Err(MoveError::IllegalMove) => {
                println!("Illegal move! That move violates chess rules.");
            }
            Err(MoveError::MustGetOutOfCheck) => {
                println!("You are in check! You must move to get out of check.");
            }
            Err(MoveError::NoPieceAtPosition) => {
                println!("No piece at the starting square!");
            }
            Err(MoveError::InvalidPosition) => {
                println!("Invalid position!");
            }
            Err(MoveError::GameNotInProgress) => {
                println!("The game has ended. Cannot make more moves.");
            }
        }
    }
}