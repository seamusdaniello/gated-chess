use super::piece_fsm::PieceFSM;
use crate::board;
use crate::pieces::{Piece, PieceType};
use crate::game::Position;

pub struct QueenFSM {
    piece: Piece,
    pub position: Position,
}

impl QueenFSM {
    pub fn new(piece: Piece, position: Position) -> Self {
        Self { piece, position }
    }
}

impl PieceFSM for QueenFSM {
    fn update_state(&mut self) {
        dbg!("Color was: ", self.piece.color);
        self.update_color();
        dbg!("Queen color is ", self.piece.color);
    }
    fn update_kind(&mut self) {}
    fn update_color(&mut self) {
        if (self.piece.get_color() == crate::pieces::Color::White) {
            self.piece.color = crate::pieces::Color::Black;
        } else {
            self.piece.color = crate::pieces::Color::White;
        }
    }
    fn update_has_moved(&mut self) {}

    fn piece(&self) -> &Piece { &self.piece }
    fn piece_mut(&mut self) -> &mut Piece { &mut self.piece }

    fn check_idle(&mut self) {
        
    }
}