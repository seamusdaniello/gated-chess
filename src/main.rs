// =======================================================
// Project: GatedChess
// File: main.rs
// Description: Enables GatedChess execution.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-14
// License: MIT
// =======================================================

mod board;   // Loads board module
mod pieces;  // Loads pieces module
mod gates;
mod game;    // Loads game module
mod config;  // Loads config module
mod parser;  // Loads parser module

use crate::gates::update_gates;
use crate::board::{create_board, print_board};
use crate::game::{Game, Position, MoveError, GameResult};
use crate::pieces::Color;

use std::io;

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
        println!("Enter a move in algebraic form (e.g., e2e4):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input using your parser module
        let (from_pos, to_pos) = match parser::parse_move(&input) {
            Some(m) => m,
            None => {
                println!("Invalid move! Format must be 'e2e4', letters a-h and numbers 1-8.");
                continue;
            }
        };

        // Attempt to make the move
        match game.make_move(from_pos, to_pos) {
            Ok(()) => {
                // Move was successful, update gates
                update_gates(&mut game.board);

                // Optional: show check warning
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
