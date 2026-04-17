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
    fn update_state(&mut self) {
        dbg!("Color was: ", self.piece.color);
        self.update_color();
        dbg!("Queen color is ", self.piece.color);
    }
    fn update_color(&mut self) {
        if self.piece.get_color() == crate::pieces::Color::White  {
            self.piece.color = crate::pieces::Color::Black;
        } else {
            self.piece.color = crate::pieces::Color::White;
        }
    }

    fn piece(&self) -> &Piece { &self.piece }
}
