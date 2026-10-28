use crate::{
    board::{defs::Location, Board},
    types::{
        bitboard::Bitboard,
        file::File,
        piece::{EveryPiece, Piece},
        rank::Rank,
        side::{EverySide, Side},
        square::{EverySquare, Square},
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

    #[inline(always)]
    pub fn us(&self) -> Side {
        self.game_state.active_side
    }

    #[inline(always)]
    pub fn opponent(&self) -> Side {
        self.game_state.active_side.other()
    }

    #[inline(always)]
    pub fn occupancy(&self) -> Bitboard {
        self.bitboard_sides[Side::White] | self.bitboard_sides[Side::Black]
    }

    #[inline(always)]
    pub fn empty(&self) -> Bitboard {
        !self.occupancy()
    }

    #[inline(always)]
    pub fn king_square(&self, side: Side) -> Square {
        let sq = self.bitboard_pieces[side][Piece::King].trailing_zeros();
        Square::from(sq)
    }

    #[inline(always)]
    pub fn file_and_rank(square: Square) -> Location {
        let file = File::from(square.idx() % 8);
        let rank = Rank::from(square.idx() / 8);
        Location { file, rank }
    }

    #[inline(always)]
    pub fn fourth_rank(side: Side) -> Rank {
        if side == Side::White {
            Rank::R4
        } else {
            Rank::R5
        }
    }

    #[inline(always)]
    pub fn promotion_rank(side: Side) -> Rank {
        if side == Side::White {
            Rank::R8
        } else {
            Rank::R1
        }
    }

    #[inline(always)]
    pub fn pawn_direction(side: Side) -> i8 {
        if side == Side::White {
            8
        } else {
            -8
        }
    }
}
