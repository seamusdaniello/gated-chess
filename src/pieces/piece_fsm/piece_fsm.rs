use crate::pieces::Piece;

pub trait PieceFSM {
    fn update_state(&mut self);
    fn update_kind(&mut self);
    fn update_color(&mut self);
    fn update_has_moved(&mut self);

    fn piece(&self) -> &Piece;
    fn piece_mut(&mut self) -> &mut Piece;
}