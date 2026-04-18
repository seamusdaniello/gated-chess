use super::piece_fsm::PieceFSM;
use crate::pieces::Piece;

pub struct QueenFSM {
    piece: Piece,
}

impl QueenFSM {
    pub fn new(piece: Piece) -> Self {
        Self { piece }
    }
}

impl PieceFSM for QueenFSM {
    fn update_state(&mut self) {}
    fn update_color(&mut self) {}

    fn piece(&self) -> &Piece {
        &self.piece
    }
}
