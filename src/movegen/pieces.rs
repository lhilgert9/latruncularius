use crate::{
    board::Board,
    misc::bits,
    movegen::{defs::Move, move_list::MoveList, movedata::CASTLING_OPTIONS, MoveGenerator},
    types::{
        bitboard::Bitboard,
        piece::Piece,
        square::{Square, BB_SQUARES},
    },
};

impl MoveGenerator {
    pub fn king_moves(&self, board: &Board, move_list: &mut MoveList, mask: Bitboard) {
        let mut pieces = board.bitboard_pieces[board.us()][Piece::King];
        while pieces > Bitboard::empty() {
            let from = bits::next(&mut pieces);
            let moves = self.get_king_attacks(from) & mask;
            self.add_moves(board, Piece::King, from, moves, move_list);
        }
    }

    pub fn queen_moves(&self, board: &Board, move_list: &mut MoveList, mask: Bitboard) {
        let mut pieces = board.bitboard_pieces[board.us()][Piece::Queen];
        let occupancy = board.occupancy();
        while pieces > Bitboard::empty() {
            let from = bits::next(&mut pieces);
            let moves = self.get_queen_attacks(from, occupancy) & mask;
            self.add_moves(board, Piece::Queen, from, moves, move_list);
        }
    }

    pub fn rook_moves(&self, board: &Board, move_list: &mut MoveList, mask: Bitboard) {
        let mut pieces = board.bitboard_pieces[board.us()][Piece::Rook];
        let occupancy = board.occupancy();
        while pieces > Bitboard::empty() {
            let from = bits::next(&mut pieces);
            let moves = self.get_rook_attacks(from, occupancy) & mask;
            self.add_moves(board, Piece::Rook, from, moves, move_list);
        }
    }

    pub fn bishop_moves(&self, board: &Board, move_list: &mut MoveList, mask: Bitboard) {
        let mut pieces = board.bitboard_pieces[board.us()][Piece::Bishop];
        let occupancy = board.occupancy();
        while pieces > Bitboard::empty() {
            let from = bits::next(&mut pieces);
            let moves = self.get_bishop_attacks(from, occupancy) & mask;
            self.add_moves(board, Piece::Bishop, from, moves, move_list);
        }
    }

    pub fn knight_moves(&self, board: &Board, move_list: &mut MoveList, mask: Bitboard) {
        let mut pieces = board.bitboard_pieces[board.us()][Piece::Knight];
        while pieces > Bitboard::empty() {
            let from = bits::next(&mut pieces);
            let moves = self.get_knight_attacks(from) & mask;
            self.add_moves(board, Piece::Knight, from, moves, move_list);
        }
    }

    pub fn castling(&self, board: &Board, move_list: &mut MoveList) {
        let us = board.us();
        let occupancy = board.occupancy();
        let king_square = board.king_square(us);
        let from = BB_SQUARES[king_square];
        let mut to = Bitboard::empty();

        for option in CASTLING_OPTIONS {
            if option.side != us {
                continue;
            }
            if option.flag & board.game_state.castling == Bitboard::empty() {
                continue;
            }
            if option
                .blockers
                .iter()
                .any(|&square| BB_SQUARES[square] & occupancy > Bitboard::empty())
            {
                continue;
            }
            if option
                .dangerous
                .iter()
                .any(|&square| self.is_square_attacked(board, board.opponent(), square))
            {
                continue;
            }

            to |= if option.kingside {
                from << 2
            } else {
                from >> 2
            }
        }

        self.add_moves(board, Piece::King, king_square, to, move_list);
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
