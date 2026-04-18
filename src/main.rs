// =======================================================
// Project: GatedChess
// File: main.rs
// Description: Program entry point — delegates to frontend.
// Author: Seamus Daniello
// Created: 2025-11-14
// License: MIT
// =======================================================

mod board;
mod config;
mod frontend; // <-- your GUI code lives in src/frontend/mod.rs
mod game;
mod gates;
mod network;
mod pieces;
mod time_control;

#[macroquad::main("GatedChess")]
async fn main() {
    frontend::run_ui().await;
}
