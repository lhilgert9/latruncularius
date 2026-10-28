use crate::{
    board::Board,
    movegen::{move_list::MoveList, MoveGenerator},
    types::{bitboard::Bitboard, piece::Piece, side::Side, square::Square},
};

impl MoveGenerator {
    pub fn generate_moves(&self, board: &Board, move_list: &mut MoveList) {
        let mask = !board.bitboard_sides[board.us()];
        self.king_moves(board, move_list, mask);
        self.queen_moves(board, move_list, mask);
        self.rook_moves(board, move_list, mask);
        self.bishop_moves(board, move_list, mask);
        self.knight_moves(board, move_list, mask);
        self.pawn_pushes(board, move_list);
        self.pawn_captures(board, move_list);
        self.castling(board, move_list);
    }

    pub fn is_square_attacked(&self, board: &Board, attacker: Side, square: Square) -> bool {
        let attackers = board.bitboard_pieces[attacker];

        let occupancy = board.occupancy();
        let king = self.get_king_attacks(square);
        let rook = self.get_rook_attacks(square, occupancy);
        let bishop = self.get_bishop_attacks(square, occupancy);
        let queen = rook | bishop;
        let knight = self.get_knight_attacks(square);
        let pawn = self.get_pawn_attacks(attacker.other(), square);
        let empty = Bitboard::empty();

        (king & attackers[Piece::King]) > empty
            || (queen & attackers[Piece::Queen]) > empty
            || (rook & attackers[Piece::Rook]) > empty
            || (bishop & attackers[Piece::Bishop]) > empty
            || (knight & attackers[Piece::Knight]) > empty
            || (pawn & attackers[Piece::Pawn]) > empty
    }
}
