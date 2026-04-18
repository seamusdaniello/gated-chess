use super::piece_fsm::PieceFSM;
use crate::pieces::Piece;

pub struct KingFSM {
    piece: Piece,
}

impl KingFSM {
    pub fn new(piece: Piece) -> Self {
        Self { piece }
    }
}

impl PieceFSM for KingFSM {
    fn update_state(&mut self) {}
    fn update_color(&mut self) {}

    fn piece(&self) -> &Piece {
        &self.piece
    }
}
