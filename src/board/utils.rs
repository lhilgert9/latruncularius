use crate::{
    board::Board,
    types::{
        bitboard::Bitboard,
        piece::{EveryPiece, Piece},
        side::{EverySide, Side},
        square::EverySquare,
    },
};

impl Board {
    pub fn reset(&mut self) {
        self.bitboard_pieces = EverySide::new(EveryPiece::new(Bitboard::empty()));
        self.bitboard_sides = EverySide::new(Bitboard::empty());
        self.game_state.clear();
        self.history.clear();
        self.pieces = EverySquare::new(Piece::None);
    }

    pub fn us(&self) -> Side {
        self.game_state.active_side
    }

    pub fn opponent(&self) -> Side {
        self.game_state.active_side.other()
    }

    pub fn occupancy(&self) -> Bitboard {
        self.bitboard_sides[Side::White] | self.bitboard_sides[Side::Black]
    }

    pub fn empty(&self) -> Bitboard {
        !self.occupancy()
    }
}
