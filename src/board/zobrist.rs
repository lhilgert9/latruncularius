use rand::RngExt;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

use crate::types::bitboard::Bitboard;
use crate::types::nrof::NrOf;
use crate::types::piece::Piece;
use crate::types::side::Side;
use crate::types::square::Square;

const RNG_SEED: [u8; 32] = [125; 32];
pub type ZobristKey = u64;

pub struct ZobristRandoms {
    pieces: [[[ZobristKey; NrOf::SQUARES]; NrOf::PIECES]; NrOf::SIDES],
    castling: [ZobristKey; 16],
    black: ZobristKey,
    en_passant: [ZobristKey; NrOf::SQUARES],
}

impl ZobristRandoms {
    pub fn new() -> Self {
        let mut random = ChaChaRng::from_seed(RNG_SEED);
        let mut zobrist_randoms = Self {
            pieces: [[[0; NrOf::SQUARES]; NrOf::PIECES]; NrOf::SIDES],
            castling: [0; 16],
            black: 0,
            en_passant: [0; NrOf::SQUARES],
        };

        zobrist_randoms.pieces.iter_mut().for_each(|side| {
            side.iter_mut().for_each(|piece| {
                piece
                    .iter_mut()
                    .for_each(|square| *square = random.random::<u64>());
            });
        });

        zobrist_randoms
            .castling
            .iter_mut()
            .for_each(|right| *right = random.random::<u64>());

        zobrist_randoms
            .en_passant
            .iter_mut()
            .for_each(|right| *right = random.random::<u64>());

        zobrist_randoms.black = random.random::<u64>();

        zobrist_randoms
    }

    pub fn piece(&self, side: Side, piece: Piece, square: Square) -> ZobristKey {
        self.pieces[side.idx()][piece.idx()][square.idx()]
    }

    pub fn castling(&self, rights: Bitboard) -> ZobristKey {
        self.castling[rights.idx()]
    }

    pub fn black(&self) -> ZobristKey {
        self.black
    }

    pub fn en_passant(&self, en_passant: Option<Square>) -> ZobristKey {
        match en_passant {
            Some(square) => self.en_passant[square.idx()],
            None => 0,
        }
    }
}
