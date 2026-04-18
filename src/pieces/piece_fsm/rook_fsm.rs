use super::piece_fsm::PieceFSM;
use crate::pieces::Piece;

pub struct RookFSM {
    piece: Piece,
}

impl RookFSM {
    pub fn new(piece: Piece) -> Self {
        Self { piece }
    }
}

impl PieceFSM for RookFSM {
    fn update_state(&mut self) {}
    fn update_color(&mut self) {}

    fn piece(&self) -> &Piece {
        &self.piece
    }
}
