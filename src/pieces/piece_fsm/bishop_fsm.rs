use super::piece_fsm::PieceFSM;
use crate::pieces::Piece;

pub struct BishopFSM {
    piece: Piece,
}

impl BishopFSM {
    pub fn new(piece: Piece) -> Self {
        Self { piece }
    }
}

impl PieceFSM for BishopFSM {
    fn update_state(&mut self) {}
    fn update_color(&mut self) {}

    fn piece(&self) -> &Piece {
        &self.piece
    }
}
