// =======================================================
// Project: GatedChess
// File: game.rs
// Description: Defines game rules.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-07
// License: MIT
// =======================================================

pub mod moves;

use crate::board::Square;
use crate::pieces::{Color, PieceType};
use crate::config::BOARD_SIZE;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Option<Position> {
        if row < BOARD_SIZE && col < BOARD_SIZE {
            Some(Position { row, col })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveError {
    IllegalMove,
    NoPieceAtPosition,
    InvalidPosition,
    MustGetOutOfCheck,
    GameNotInProgress,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameResult {
    Checkmate(Color),
    Stalemate,
    InProgress,
}

pub struct Game {
    pub board: [[Square; BOARD_SIZE]; BOARD_SIZE],
    pub current_turn: Color,
    pub result: GameResult,
}

impl Game {
    pub fn new(board: [[Square; BOARD_SIZE]; BOARD_SIZE]) -> Game {
        Game {
            board,
            current_turn: Color::White,
            result: GameResult::InProgress,
        }
    }

    pub fn get_all_legal_moves(&self, color: Color) -> Vec<Position> {
        use moves::generation::get_piece_moves;
        let mut all_moves = Vec::new();
        
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board[row][col].piece {
                    if piece.color == color {
                        let pos = Position { row, col };
                        // Get moves for this piece without checking current_turn
                        let mut moves = get_piece_moves(self, pos, color);
                        // Filter out moves that leave king in check
                        moves.retain(|&to| !self.leaves_king_in_check(pos, to, color));
                        all_moves.extend(moves);
                    }
                }
            }
        }
        all_moves
    }

    pub fn make_move(&mut self, from: Position, to: Position) -> Result<(), MoveError> {
        if self.result != GameResult::InProgress {
            return Err(MoveError::GameNotInProgress);
        }
        // Check if the move is legal
        let legal_moves = self.get_legal_moves(from);
        if !legal_moves.contains(&to) {
            return Err(MoveError::IllegalMove);
        }
        if self.leaves_king_in_check(from, to, self.current_turn) {
            return Err(MoveError::MustGetOutOfCheck);
        }

        self.make_move_unchecked(from, to);
        self.switch_turn();

        self.check_game_result();
        
        Ok(())
    }

    pub(crate) fn make_move_unchecked(&mut self, from: Position, to: Position) {
        self.board[to.row][to.col].piece = self.board[from.row][from.col].piece.take();
    }

    pub fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    pub fn get_legal_moves(&self, pos: Position) -> Vec<Position> {
        use moves::generation::get_piece_moves;
        
        if let Some(piece) = self.board[pos.row][pos.col].piece {
            if piece.color != self.current_turn {
                return Vec::new();
            }
            
            let mut moves = get_piece_moves(self, pos, piece.color);
            
            // Filter out moves that leave king in check
            moves.retain(|&to| !self.leaves_king_in_check(pos, to, piece.color));
            
            moves
        } else {
            Vec::new()
        }
    }

    pub(crate) fn apply_offset(&self, pos: Position, dr: i32, dc: i32) -> Option<Position> {
        let new_row = pos.row as i32 + dr;
        let new_col = pos.col as i32 + dc;
        
        if new_row >= 0 && new_row < BOARD_SIZE as i32 &&
           new_col >= 0 && new_col < BOARD_SIZE as i32 {
            Some(Position {
                row: new_row as usize,
                col: new_col as usize,
            })
        } else {
            None
        }
    }

    pub(crate) fn can_move_to(&self, pos: Position, color: Color) -> bool {
        if let Some(piece) = self.board[pos.row][pos.col].piece {
            piece.color != color
        } else {
            true
        }
    }

    pub(crate) fn slide_in_direction(&self, pos: Position, dr: i32, dc: i32, color: Color) -> Vec<Position> {
        let mut moves = Vec::new();
        let mut current = pos;
        
        loop {
            if let Some(next) = self.apply_offset(current, dr, dc) {
                if let Some(piece) = self.board[next.row][next.col].piece {
                    if piece.color != color {
                        moves.push(next);
                    }
                    break;
                } else {
                    moves.push(next);
                    current = next;
                }
            } else {
                break;
            }
        }
        
        moves
    }

    pub(crate) fn is_king_in_check(&self, color: Color) -> bool {
        // Find the king
        let king_pos = self.find_king(color);
        if king_pos.is_none() {
            return false;
        }
        let king_pos = king_pos.unwrap();
        
        // Check if any opponent piece can attack the king
        let opponent_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board[row][col].piece {
                    if piece.color == opponent_color {
                        let pos = Position { row, col };
                        let moves = moves::generation::get_piece_moves(self, pos, opponent_color);
                        if moves.contains(&king_pos) {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        // Checkmate: king is in check AND has no legal moves
        self.is_king_in_check(color) && self.get_all_legal_moves(color).is_empty()
    }
    
    pub fn is_stalemate(&self, color: Color) -> bool {
        // Stalemate: king is NOT in check BUT has no legal moves
        !self.is_king_in_check(color) && self.get_all_legal_moves(color).is_empty()
    }
    
    pub fn check_game_result(&mut self) {
        // Check the state of the game after each move
        let opponent_color = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        
        if self.is_checkmate(opponent_color) {
            // The opponent is in checkmate, so current player wins
            self.result = GameResult::Checkmate(self.current_turn);
        } else if self.is_stalemate(opponent_color) {
            self.result = GameResult::Stalemate;
        }
    }

    fn find_king(&self, color: Color) -> Option<Position> {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board[row][col].piece {
                    if piece.kind == PieceType::King && piece.color == color {
                        return Some(Position { row, col });
                    }
                }
            }
        }
        None
    }
}
