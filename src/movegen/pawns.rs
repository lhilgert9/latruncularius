use crate::{
    board::Board,
    misc::bits,
    movegen::{defs::Move, move_list::MoveList, MoveGenerator},
    types::{
        bitboard::Bitboard,
        piece::Piece,
        rank::{Rank, BB_RANKS},
        side::Side,
        square::{Square, BB_SQUARES},
    },
};

const PROMOTION_OPTIONS: [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

impl MoveGenerator {
    pub fn pawn_pushes(&self, board: &Board, move_list: &mut MoveList) {
        let us = board.us();
        let empty = board.empty();
        let direction = if us == Side::White { 8 } else { -8 } as i8;
        let shift = direction.abs() as u64;
        let fourth_rank = if us == Side::White {
            BB_RANKS[Rank::R4.idx()]
        } else {
            BB_RANKS[Rank::R5.idx()]
        };
        let promotion_rank = if us == Side::White {
            BB_RANKS[Rank::R8.idx()]
        } else {
            BB_RANKS[Rank::R1.idx()]
        };
        let mut pieces = board.bitboard_pieces[board.us()][Piece::Pawn];
        while pieces > Bitboard::empty() {
            let from = bits::next(&mut pieces);
            let to = (from as i8 + direction) as usize;
            let one_step = BB_SQUARES[to] & empty;
            let two_step = if direction > 0 {
                (one_step << shift) & fourth_rank & empty
            } else {
                (one_step >> shift) & fourth_rank & empty
            };
            self.add_pawn_pushes(from, (one_step & !promotion_rank) | two_step, move_list);
            self.add_pawn_promotions(board, from, one_step & promotion_rank, move_list);
        }
    }

    pub fn pawn_captures(&self, board: &Board, move_list: &mut MoveList) {
        let us = board.us();
        let opponent_pieces = board.bitboard_sides[board.opponent()];
        let promotion_rank = if us == Side::White {
            BB_RANKS[Rank::R8.idx()]
        } else {
            BB_RANKS[Rank::R1.idx()]
        };

        let en_passant = board
            .game_state
            .en_passant
            .map_or(Bitboard::empty(), |ep| BB_SQUARES[ep.idx()]);

        let mut pieces = board.bitboard_pieces[board.us()][Piece::Pawn];

        while pieces > Bitboard::empty() {
            let from = bits::next(&mut pieces);
            let attacks = self.pawn_attacks[us][from];

            let captures = attacks & (opponent_pieces | en_passant);

            self.add_pawn_captures(board, from, captures & !promotion_rank, move_list);
            self.add_pawn_promotions(board, from, captures & promotion_rank, move_list);
        }
    }
}

impl MoveGenerator {
    fn add_pawn_pushes(&self, from: Square, mut targets: Bitboard, move_list: &mut MoveList) {
        while targets > Bitboard::empty() {
            let to = bits::next(&mut targets);
            let distance = (to as i8 - from as i8).abs();
            let double_step = distance == 16;
            move_list.push(Move::new(
                Piece::Pawn,
                from,
                to,
                Piece::None,
                Piece::None,
                false,
                double_step,
                false,
            ));
        }
    }

    fn add_pawn_captures(
        &self,
        board: &Board,
        from: Square,
        mut targets: Bitboard,
        move_list: &mut MoveList,
    ) {
        while targets > Bitboard::empty() {
            let to = bits::next(&mut targets);
            let captured = board.pieces[to];
            let en_passant = board.game_state.en_passant.map_or(false, |ep| ep == to);
            move_list.push(Move::new(
                Piece::Pawn,
                from,
                to,
                captured,
                Piece::None,
                en_passant,
                false,
                false,
            ));
        }
    }

    fn add_pawn_promotions(
        &self,
        board: &Board,
        from: Square,
        mut targets: Bitboard,
        move_list: &mut MoveList,
    ) {
        while targets > Bitboard::empty() {
            let to = bits::next(&mut targets);
            let captured = board.pieces[to];
            for promotion in PROMOTION_OPTIONS {
                move_list.push(Move::new(
                    Piece::Pawn,
                    from,
                    to,
                    captured,
                    promotion,
                    false,
                    false,
                    false,
                ));
            }
        }
    }
}
