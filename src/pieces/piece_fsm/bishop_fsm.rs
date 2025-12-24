use super::piece_fsm::PieceFSM;
use crate::pieces::{Piece, PieceState};
use crate::game::Position;

pub struct BishopFSM {
    piece: Piece,
    pub position: Position,
}

impl BishopFSM {
    pub fn new(piece: Piece, position: Position) -> Self {
        Self { piece, position }
    }
}

impl PieceFSM for BishopFSM {
    fn update_state(&mut self) {}
    fn update_kind(&mut self) {}
    fn update_color(&mut self) {}
    fn update_has_moved(&mut self) {}

    fn piece(&self) -> &Piece { &self.piece }
    fn piece_mut(&mut self) -> &mut Piece { &mut self.piece }

    fn check_idle(&mut self) {
        if (self.piece.get_state() == PieceState::Idle) {
            
        }
    }
}