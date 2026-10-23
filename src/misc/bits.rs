use crate::types::{bitboard::Bitboard, square::Square};

pub fn next(bitboard: &mut Bitboard) -> Square {
    let square = Square::from(bitboard.trailing_zeros());
    *bitboard ^= Bitboard::new(1) << square as u64;
    square
}
