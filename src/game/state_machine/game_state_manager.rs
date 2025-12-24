use std::collections::HashMap;
use std::hash::Hash;
use crate::board::{Square};
use crate::config::BOARD_SIZE;
use crate::pieces::{Color, Piece, PieceId, PieceType};
use crate::pieces::piece_fsm::*;
use crate::pieces::piece_fsm::PieceFSM;
use crate::game::Position;

pub struct GameStateManager {
    pub piece_fsms: HashMap<PieceId, Box<dyn PieceFSM>>,
    pub piece_positions: HashMap<PieceId, Position>,
}

impl<'a> GameStateManager {
    pub fn new() -> Self {
        Self {
            piece_fsms: HashMap::new(),
            piece_positions: HashMap::new(),
        }
    }

    pub fn register_all_pieces(
        &mut self,
        board: &mut [[Square; crate::config::BOARD_SIZE]; crate::config::BOARD_SIZE],
    ) {
        for row in 0..crate::config::BOARD_SIZE {
            for col in 0..crate::config::BOARD_SIZE {
                if let Some(piece) = board[row][col].piece {
                    self.register_piece(piece, Position { row, col });
                }
            }
        }
    }

    fn register_piece(&mut self, piece: crate::pieces::Piece, pos: Position) {
        let fsm: Box<dyn PieceFSM> = match piece.kind {
            crate::pieces::PieceType::Pawn => Box::new(PawnFSM::new(piece, pos)),
            crate::pieces::PieceType::Rook => Box::new(RookFSM::new(piece, pos)),
            crate::pieces::PieceType::Knight => Box::new(KnightFSM::new(piece, pos)),
            crate::pieces::PieceType::Bishop => Box::new(BishopFSM::new(piece, pos)),
            crate::pieces::PieceType::Queen => Box::new(QueenFSM::new(piece, pos)),
            crate::pieces::PieceType::King => Box::new(KingFSM::new(piece, pos)),
        };

        self.piece_fsms.insert(piece.id, fsm);
        self.piece_positions.insert(piece.id, pos);
    }

    pub fn update_all_fsm(
        &mut self,
        board: &mut [[Square; crate::config::BOARD_SIZE]; crate::config::BOARD_SIZE],
    ) {
        let mut updates: Vec<(Position, Piece)> = Vec::new();

        for (piece_id, fsm) in self.piece_fsms.iter_mut() {
            fsm.update_state();

            if let Some(&pos) = self.piece_positions.get(piece_id) {
                let updated_piece = *fsm.piece();
                updates.push((pos, updated_piece));
            }
        }
        
        for (pos, piece) in updates {
            if board[pos.row][pos.col].piece.is_some() {
                board[pos.row][pos.col].piece = Some(piece);
            }
        }
    }
}