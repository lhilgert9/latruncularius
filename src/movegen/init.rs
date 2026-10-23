use crate::{
    movegen::MoveGenerator,
    types::{
        bitboard::Bitboard,
        file::{File, BB_FILES},
        nrof::NrOf,
        rank::{Rank, BB_RANKS},
        side::EverySide,
        square::{EverySquare, Square, BB_SQUARES},
    },
};

struct PieceMove {
    offset: i8,
    impossible: Bitboard,
}

const KING_MOVES: [PieceMove; 8] = [
    PieceMove {
        offset: 8,
        impossible: BB_RANKS[Rank::R8.idx()],
    },
    PieceMove {
        offset: -8,
        impossible: BB_RANKS[Rank::R1.idx()],
    },
    PieceMove {
        offset: 1,
        impossible: BB_FILES[File::H.idx()],
    },
    PieceMove {
        offset: -1,
        impossible: BB_FILES[File::A.idx()],
    },
    PieceMove {
        offset: 7,
        impossible: BB_FILES[File::A.idx()].bitor(BB_RANKS[Rank::R8.idx()]),
    },
    PieceMove {
        offset: -7,
        impossible: BB_FILES[File::H.idx()].bitor(BB_RANKS[Rank::R1.idx()]),
    },
    PieceMove {
        offset: 9,
        impossible: BB_FILES[File::H.idx()].bitor(BB_RANKS[Rank::R8.idx()]),
    },
    PieceMove {
        offset: -9,
        impossible: BB_FILES[File::A.idx()].bitor(BB_RANKS[Rank::R1.idx()]),
    },
];

const KNIGHT_MOVES: [PieceMove; 8] = [
    PieceMove {
        offset: 15,
        impossible: BB_RANKS[Rank::R8.idx()]
            .bitor(BB_RANKS[Rank::R7.idx()])
            .bitor(BB_FILES[File::A.idx()]),
    },
    PieceMove {
        offset: 17,
        impossible: BB_RANKS[Rank::R8.idx()]
            .bitor(BB_RANKS[Rank::R7.idx()])
            .bitor(BB_FILES[File::H.idx()]),
    },
    PieceMove {
        offset: 10,
        impossible: BB_FILES[File::H.idx()]
            .bitor(BB_FILES[File::G.idx()])
            .bitor(BB_RANKS[Rank::R8.idx()]),
    },
    PieceMove {
        offset: -6,
        impossible: BB_FILES[File::H.idx()]
            .bitor(BB_FILES[File::G.idx()])
            .bitor(BB_RANKS[Rank::R1.idx()]),
    },
    PieceMove {
        offset: -17,
        impossible: BB_RANKS[Rank::R1.idx()]
            .bitor(BB_RANKS[Rank::R2.idx()])
            .bitor(BB_FILES[File::A.idx()]),
    },
    PieceMove {
        offset: -15,
        impossible: BB_RANKS[Rank::R1.idx()]
            .bitor(BB_RANKS[Rank::R2.idx()])
            .bitor(BB_FILES[File::H.idx()]),
    },
    PieceMove {
        offset: 6,
        impossible: BB_FILES[File::A.idx()]
            .bitor(BB_FILES[File::B.idx()])
            .bitor(BB_RANKS[Rank::R8.idx()]),
    },
    PieceMove {
        offset: -10,
        impossible: BB_FILES[File::A.idx()]
            .bitor(BB_FILES[File::B.idx()])
            .bitor(BB_RANKS[Rank::R1.idx()]),
    },
];

const PAWN_MOVES: [[PieceMove; 2]; NrOf::SIDES] = [
    [
        PieceMove {
            offset: 7,
            impossible: BB_FILES[File::A.idx()].bitor(BB_RANKS[Rank::R8.idx()]),
        },
        PieceMove {
            offset: 9,
            impossible: BB_FILES[File::H.idx()].bitor(BB_RANKS[Rank::R8.idx()]),
        },
    ],
    [
        PieceMove {
            offset: -9,
            impossible: BB_FILES[File::A.idx()].bitor(BB_RANKS[Rank::R1.idx()]),
        },
        PieceMove {
            offset: -7,
            impossible: BB_FILES[File::H.idx()].bitor(BB_RANKS[Rank::R1.idx()]),
        },
    ],
];

impl MoveGenerator {
    pub(super) fn init_king(king_attacks: &mut EverySquare<Bitboard>) {
        for square in (Square::A1 as usize)..=(Square::H8 as usize) {
            let from = BB_SQUARES[square];
            let mut moves = Bitboard::empty();

            for current_move in KING_MOVES.iter() {
                let can_move = from & !current_move.impossible;
                let shift = current_move.offset.unsigned_abs() as u64;

                moves |= if current_move.offset > 0 {
                    can_move << shift
                } else {
                    can_move >> shift
                }
            }

            king_attacks[square.into()] = moves;
        }
    }

    pub(super) fn init_knight(knight_attacks: &mut EverySquare<Bitboard>) {
        for square in (Square::A1 as usize)..=(Square::H8 as usize) {
            let from = BB_SQUARES[square];
            let mut moves = Bitboard::empty();

            for current_move in KNIGHT_MOVES.iter() {
                let can_move = from & !current_move.impossible;
                let shift = current_move.offset.unsigned_abs() as u64;

                moves |= if current_move.offset > 0 {
                    can_move << shift
                } else {
                    can_move >> shift
                }
            }

            knight_attacks[square.into()] = moves;
        }
    }

    pub(super) fn init_pawns(pawn_attacks: &mut EverySide<EverySquare<Bitboard>>) {
        for square in (Square::A1 as usize)..=(Square::H8 as usize) {
            let from = BB_SQUARES[square];

            for side in 0..NrOf::SIDES {
                let mut moves = Bitboard::empty();

                for current_move in &PAWN_MOVES[side] {
                    let can_move = from & !current_move.impossible;
                    let shift = current_move.offset.unsigned_abs() as u64;

                    moves |= if current_move.offset > 0 {
                        can_move << shift
                    } else {
                        can_move >> shift
                    }
                }

                pawn_attacks[side.into()][square.into()] = moves;
            }
        }
    }
}
