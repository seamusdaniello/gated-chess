use super::piece_fsm::PieceFSM;
use crate::pieces::Piece;

pub struct KnightFSM {
    piece: Piece,
}

impl KnightFSM {
    pub fn new(piece: Piece) -> Self {
        Self { piece }
    }
}

impl PieceFSM for KnightFSM {
    fn update_state(&mut self) {}
    fn update_color(&mut self) {}

    fn piece(&self) -> &Piece { &self.piece }
}
