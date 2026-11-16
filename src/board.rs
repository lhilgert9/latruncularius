pub mod defs;
mod display;
mod fen;
mod game_state;
mod history;
mod init;
mod moving;
mod utils;
mod zobrist;

use std::sync::Arc;

use crate::{
    board::{game_state::GameState, history::History, zobrist::ZobristRandoms},
    types::{
        bitboard::Bitboard,
        piece::{EveryPiece, Piece},
        side::EverySide,
        square::EverySquare,
    },
};

#[derive(Clone)]
pub struct Board {
    pub bitboard_pieces: EverySide<EveryPiece<Bitboard>>,
    pub bitboard_sides: EverySide<Bitboard>,
    pub pieces: EverySquare<Piece>,
    pub game_state: GameState,
    history: History,
    zobrist_randoms: Arc<ZobristRandoms>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitboard_pieces: EverySide::new(EveryPiece::new(Bitboard::empty())),
            bitboard_sides: EverySide::new(Bitboard::empty()),
            pieces: EverySquare::new(Piece::None),
            game_state: GameState::new(),
            history: History::new(),
            zobrist_randoms: Arc::new(ZobristRandoms::new()),
        }
    }
}
