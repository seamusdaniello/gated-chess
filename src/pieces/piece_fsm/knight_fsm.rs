use super::piece_fsm::PieceFSM;
use crate::pieces::Piece;
use crate::game::Position;

pub struct KnightFSM {
    piece: Piece,
    pub position: Position,
}

impl KnightFSM {
    pub fn new(piece: Piece, position: Position) -> Self {
        Self { piece, position }
    }
}

impl PieceFSM for KnightFSM {
    fn update_state(&mut self) {}
    fn update_kind(&mut self) {}
    fn update_color(&mut self) {}
    fn update_has_moved(&mut self) {}

    fn piece(&self) -> &Piece { &self.piece }
    fn piece_mut(&mut self) -> &mut Piece { &mut self.piece }
}