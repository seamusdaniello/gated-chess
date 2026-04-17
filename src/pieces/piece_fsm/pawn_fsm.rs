use super::piece_fsm::PieceFSM;
use crate::pieces::Piece;

pub struct PawnFSM {
    piece: Piece,
}

impl PawnFSM {
    pub fn new(piece: Piece) -> Self {
        Self { piece }
    }
}

impl PieceFSM for PawnFSM {
    fn update_state(&mut self) {}
    fn update_color(&mut self) {}

    fn piece(&self) -> &Piece {
        &self.piece
    }
}
