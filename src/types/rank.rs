use std::ops::{Index, RangeInclusive};

use crate::types::{bitboard::Bitboard, nrof::NrOf};

pub const BB_RANKS: [Bitboard; NrOf::RANKS] = init_bb_ranks();

#[derive(Clone, Copy, Debug)]
pub enum Rank {
    R1 = 0,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Rank {
    #[inline(always)]
    pub const fn range() -> RangeInclusive<u8> {
        RANGE
    }

    #[inline(always)]
    pub const fn idx(self) -> usize {
        self as usize
    }
}

const RANGE: RangeInclusive<u8> = (Rank::R1 as u8)..=(Rank::R8 as u8);

impl<T> Index<Rank> for [T; NrOf::RANKS] {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Rank) -> &Self::Output {
        &self[index as usize]
    }
}

impl From<usize> for Rank {
    #[inline(always)]
    fn from(value: usize) -> Self {
        match value {
            0 => Self::R1,
            1 => Self::R2,
            2 => Self::R3,
            3 => Self::R4,
            4 => Self::R5,
            5 => Self::R6,
            6 => Self::R7,
            7 => Self::R8,
            _ => panic!("Invalid usize for Rank: {}", value),
        }
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
