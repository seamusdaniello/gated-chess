use crate::board;

pub struct GameStateManager {
    pub board: [[Square; 8]; 8],
    pub piece_fsms: HashMap<PieceId, PieceFSM>,
    pub piece_positions: HashMap<PieceId, Position>,
}

impl GameStateManager {
    pub fn new() -> Self {
        Self::from_board(board::create_board());
    }

    pub fn from_board(board: [[Square; 8]; 8]) -> Self {
        let mut manager = Self {
            board,
            piece_fsms: HashMap::new(),
            piece_positions: HashMap::new(),
            active_gates: HashMap::new(),
            current_turn: Color::White,
            turn_number: 0,
            phase: GamePhase::AwaitingInput,
        };
    }

    fn register_all_pieces(&mut self) {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row][col].piece {
                    self.register_piece(piece, Position { row, col });
                }
            }
        }
    }
}