pub mod defs;
mod display;
pub mod fen;
mod game_state;
mod history;
mod init;
mod moving;
mod utils;

use crate::{
    board::{game_state::GameState, history::History},
    types::{
        bitboard::Bitboard,
        piece::{EveryPiece, Piece},
        side::EverySide,
        square::EverySquare,
    },
};

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub bitboard_pieces: EverySide<EveryPiece<Bitboard>>,
    pub bitboard_sides: EverySide<Bitboard>,
    pub pieces: EverySquare<Piece>,
    pub game_state: GameState,
    history: History,
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitboard_pieces: EverySide::new(EveryPiece::new(Bitboard::empty())),
            bitboard_sides: EverySide::new(Bitboard::empty()),
            pieces: EverySquare::new(Piece::None),
            game_state: GameState::new(),
            history: History::new(),
        }
    }
}
