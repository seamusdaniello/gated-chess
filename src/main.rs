// =======================================================
// Project: GatedChess
// File: main.rs
// Description: Program entry point — delegates to frontend.
// Author: Seamus Daniello
// Created: 2025-11-14
// License: MIT
// =======================================================

mod board;
mod pieces;
mod gates;
mod game;
mod config;
mod parser;
mod frontend; // <-- your GUI code lives in src/frontend/mod.rs

use crate::board::create_board;
use crate::game::Game;

#[macroquad::main("GatedChess")]
async fn main() {
        // create engine state
    let board = create_board();
    let game = Game::new(board);

    frontend::run_ui(game).await;
}
