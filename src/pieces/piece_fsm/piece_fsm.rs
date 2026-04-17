use crate::pieces::Piece;

pub trait PieceFSM {
    fn update_state(&mut self);
    fn update_color(&mut self);

    fn piece(&self) -> &Piece;
}
