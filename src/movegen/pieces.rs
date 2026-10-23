use crate::{
    board::Board,
    misc::bits,
    movegen::{defs::Move, move_list::MoveList, MoveGenerator},
    types::{bitboard::Bitboard, piece::Piece, square::Square},
};

impl MoveGenerator {
    pub fn king_moves(&self, board: &Board, move_list: &mut MoveList) {
        let mut pieces = board.bitboard_pieces[board.us()][Piece::King];
        while pieces > Bitboard::empty() {
            let from = bits::next(&mut pieces);
            let moves = self.king_attacks[from] & !board.bitboard_sides[board.us()];
            self.add_moves(board, Piece::King, from, moves, move_list);
        }
    }

    pub fn knight_moves(&self, board: &Board, move_list: &mut MoveList) {
        let mut pieces = board.bitboard_pieces[board.us()][Piece::Knight];
        while pieces > Bitboard::empty() {
            let from = bits::next(&mut pieces);
            let moves = self.knight_attacks[from] & !board.bitboard_sides[board.us()];
            self.add_moves(board, Piece::Knight, from, moves, move_list);
        }
    }
}

impl MoveGenerator {
    fn add_moves(
        &self,
        board: &Board,
        piece: Piece,
        from: Square,
        to: Bitboard,
        move_list: &mut MoveList,
    ) {
        let mut targets = to;
        while targets > Bitboard::empty() {
            let to = bits::next(&mut targets);
            let distance = (to as i8 - from as i8).abs();
            let capture = board.pieces[to];
            let castling = piece == Piece::King && distance == 2;
            move_list.push(Move::new(
                piece,
                from,
                to,
                capture,
                Piece::None,
                false,
                false,
                castling,
            ));
        }
    }
}
