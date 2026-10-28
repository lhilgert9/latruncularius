use crate::{
    board::Board,
    types::{
        bitboard::Bitboard,
        piece::Piece,
        side::Side,
        square::{Square, BB_SQUARES},
    },
};

impl Board {
    pub fn init(&mut self) {
        let mut bb_white = Bitboard::empty();
        let mut bb_black = Bitboard::empty();

        for (bb_w, bb_b) in self.bitboard_pieces[Side::White]
            .iter()
            .zip(self.bitboard_pieces[Side::Black].iter())
        {
            bb_white |= *bb_w;
            bb_black |= *bb_b;
        }
        self.bitboard_sides[Side::White] = bb_white;
        self.bitboard_sides[Side::Black] = bb_black;

        for (p_type, (w, b)) in self.bitboard_pieces[Side::White]
            .iter()
            .zip(self.bitboard_pieces[Side::Black].iter())
            .enumerate()
        {
            let mut white_pieces = *w;
            let mut black_pieces = *b;
            let piece_type = Piece::from(p_type);

            while white_pieces > Bitboard::empty() {
                let square = Square::from(white_pieces.trailing_zeros());
                white_pieces ^= BB_SQUARES[square];
                self.pieces[square] = piece_type;
            }
            while black_pieces > Bitboard::empty() {
                let square = Square::from(black_pieces.trailing_zeros());
                black_pieces ^= BB_SQUARES[square];
                self.pieces[square] = piece_type;
            }
        }
    }
}
