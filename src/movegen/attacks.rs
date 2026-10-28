use crate::movegen::MoveGenerator;
use crate::types::bitboard::Bitboard;
use crate::types::side::Side;
use crate::types::square::Square;

impl MoveGenerator {
    pub(super) fn get_king_attacks(&self, square: Square) -> Bitboard {
        self.king_attacks[square]
    }

    pub(super) fn get_queen_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        self.get_rook_attacks(square, occupancy) ^ self.get_bishop_attacks(square, occupancy)
    }

    pub(super) fn get_rook_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let index = self.rook_magics[square].get_index(occupancy);
        self.rook_attacks[index]
    }

    pub(super) fn get_bishop_attacks(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let index = self.bishop_magics[square].get_index(occupancy);
        self.bishop_attacks[index]
    }

    pub(super) fn get_knight_attacks(&self, square: Square) -> Bitboard {
        self.knight_attacks[square]
    }

    pub(super) fn get_pawn_attacks(&self, side: Side, square: Square) -> Bitboard {
        self.pawn_attacks[side][square]
    }
}
