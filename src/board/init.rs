use crate::{
    board::{Board, zobrist::ZobristKey},
    types::{
        bitboard::Bitboard,
        piece::Piece,
        side::Side,
        square::{BB_SQUARES, EverySquare, Square},
    },
};

impl Board {
    pub fn init(&mut self) {
        let (bb_white, bb_black) = self.init_bitboards();
        self.bitboard_sides[Side::White] = bb_white;
        self.bitboard_sides[Side::Black] = bb_black;

        self.pieces = self.init_pieces();
        self.game_state.zobrist_key = self.init_zobrist_key();
    }

    fn init_bitboards(&self) -> (Bitboard, Bitboard) {
        let mut bb_white = Bitboard::empty();
        let mut bb_black = Bitboard::empty();

        for (bb_w, bb_b) in self.bitboard_pieces[Side::White]
            .iter()
            .zip(self.bitboard_pieces[Side::Black].iter())
        {
            bb_white |= *bb_w;
            bb_black |= *bb_b;
        }
        (bb_white, bb_black)
    }

    fn init_pieces(&self) -> EverySquare<Piece> {
        let bb_white = self.bitboard_pieces[Side::White];
        let bb_black = self.bitboard_pieces[Side::Black];
        let mut pieces = EverySquare::new(Piece::None);

        for (piece_type, (w, b)) in bb_white.iter().zip(bb_black.iter()).enumerate() {
            let mut white_pieces = *w;
            let mut black_pieces = *b;
            let piece_type = Piece::from(piece_type);

            while white_pieces > Bitboard::empty() {
                let square = Square::from(white_pieces.trailing_zeros());
                white_pieces ^= BB_SQUARES[square];
                pieces[square] = piece_type;
            }
            while black_pieces > Bitboard::empty() {
                let square = Square::from(black_pieces.trailing_zeros());
                black_pieces ^= BB_SQUARES[square];
                pieces[square] = piece_type;
            }
        }
        pieces
    }

    fn init_zobrist_key(&self) -> ZobristKey {
        let bb_white = self.bitboard_pieces[Side::White];
        let bb_black = self.bitboard_pieces[Side::Black];
        let mut key = 0;

        for (piece_type, (w, b)) in bb_white.iter().zip(bb_black.iter()).enumerate() {
            let mut white_pieces = *w;
            let mut black_pieces = *b;
            let piece = Piece::from(piece_type);

            while white_pieces > Bitboard::empty() {
                let square = Square::from(white_pieces.trailing_zeros());
                white_pieces ^= BB_SQUARES[square];
                key ^= self.zobrist_randoms.piece(Side::White, piece, square);
            }
            while black_pieces > Bitboard::empty() {
                let square = Square::from(black_pieces.trailing_zeros());
                black_pieces ^= BB_SQUARES[square];
                key ^= self.zobrist_randoms.piece(Side::Black, piece, square);
            }
        }

        if self.game_state.active_side == Side::Black {
            key ^= self.zobrist_randoms.black();
        }
        key ^= self.zobrist_randoms.castling(self.game_state.castling);
        key ^= self.zobrist_randoms.en_passant(self.game_state.en_passant);
        key
    }
}
