use crate::types::{bitboard::Bitboard, nrof::NrOf};

pub const BB_RANKS: [Bitboard; NrOf::RANKS] = init_bb_ranks();

pub enum Rank {
    R1 = 0,
    R2,
    _R3,
    R4,
    R5,
    _R6,
    R7,
    R8,
}

impl Rank {
    pub const fn idx(self) -> usize {
        self as usize
    }
}

const fn init_bb_ranks() -> [Bitboard; NrOf::RANKS] {
    let r1 = Bitboard::new(0xFF);
    let mut ranks = [Bitboard::empty(); NrOf::RANKS];
    let mut i = 0;
    while i < NrOf::RANKS {
        ranks[i] = r1.shl(i as u64 * 8);
        i += 1;
    }
    ranks
}
