use crate::types::bitboard::Bitboard;

pub struct CastlingRights;
impl CastlingRights {
    pub const WK: Bitboard = Bitboard::new(0b0001);
    pub const WQ: Bitboard = Bitboard::new(0b0010);
    pub const BK: Bitboard = Bitboard::new(0b0100);
    pub const BQ: Bitboard = Bitboard::new(0b1000);
    pub const ALL: Bitboard = Bitboard::new(0b1111);
}
