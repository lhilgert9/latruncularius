use crate::types::bitboard::Bitboard;
use crate::types::castling::CastlingRights;
use crate::types::file::File;
use crate::types::file::BB_FILES;
use crate::types::nrof::NrOf;
use crate::types::rank::Rank;
use crate::types::rank::BB_RANKS;
use crate::types::side::Side;
use crate::types::square::Square;

pub(super) struct PieceMove {
    pub offset: i8,
    pub impossible: Bitboard,
}

pub(super) const KING_MOVES: [PieceMove; 8] = [
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

pub(super) const KNIGHT_MOVES: [PieceMove; 8] = [
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

pub(super) const PAWN_MOVES: [[PieceMove; 2]; NrOf::SIDES] = [
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

pub(super) struct CastlingOption<'a> {
    pub side: Side,
    pub flag: Bitboard,
    pub blockers: &'a [Square],
    pub dangerous: &'a [Square],
    pub kingside: bool,
}

pub(super) const CASTLING_OPTIONS: [CastlingOption; 4] = [
    CastlingOption {
        side: Side::White,
        flag: CastlingRights::WK,
        blockers: &[Square::F1, Square::G1],
        dangerous: &[Square::E1, Square::F1],
        kingside: true,
    },
    CastlingOption {
        side: Side::White,
        flag: CastlingRights::WQ,
        blockers: &[Square::D1, Square::C1, Square::B1],
        dangerous: &[Square::E1, Square::D1],
        kingside: false,
    },
    CastlingOption {
        side: Side::Black,
        flag: CastlingRights::BK,
        blockers: &[Square::F8, Square::G8],
        dangerous: &[Square::E8, Square::F8],
        kingside: true,
    },
    CastlingOption {
        side: Side::Black,
        flag: CastlingRights::BQ,
        blockers: &[Square::D8, Square::C8, Square::B8],
        dangerous: &[Square::E8, Square::D8],
        kingside: false,
    },
];
