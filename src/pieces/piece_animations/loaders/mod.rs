pub mod bishop_loader;
pub mod pawn_loader;
pub mod knight_loader;
pub mod rook_loader;
pub mod queen_loader;
pub mod king_loader;

pub use bishop_loader::load_bishop_frames;
pub use pawn_loader::load_pawn_frames;
pub use knight_loader::load_knight_frames;
pub use rook_loader::load_rook_frames;
pub use queen_loader::load_queen_frames;
pub use king_loader::load_king_frames;
