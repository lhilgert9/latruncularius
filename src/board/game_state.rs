use crate::{
    board::zobrist::ZobristKey,
    movegen::defs::Move,
    types::{bitboard::Bitboard, side::Side, square::Square},
};

#[derive(Clone, Copy, Debug)]
pub struct GameState {
    pub active_side: Side,
    pub castling: Bitboard,
    pub half_move_clock: u8,
    pub en_passant: Option<Square>,
    pub full_move_number: u16,
    pub next_move: Move,
    pub zobrist_key: ZobristKey,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            active_side: Side::White,
            castling: Bitboard::empty(),
            half_move_clock: 0,
            en_passant: None,
            full_move_number: 0,
            next_move: Move::empty(),
            zobrist_key: 0,
        }
    }

    pub fn clear(&mut self) {
        self.active_side = Side::White;
        self.castling = Bitboard::empty();
        self.half_move_clock = 0;
        self.en_passant = None;
        self.full_move_number = 0;
        self.zobrist_key = 0;
    }
}
