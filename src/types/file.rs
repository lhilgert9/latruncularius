use std::ops::{Index, RangeInclusive};

use crate::types::{bitboard::Bitboard, nrof::NrOf};

pub const BB_FILES: [Bitboard; NrOf::FILES] = init_bb_files();

#[derive(Clone, Copy, Debug)]
pub enum File {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    #[inline(always)]
    pub const fn range() -> RangeInclusive<u8> {
        RANGE
    }

    #[inline(always)]
    pub const fn idx(self) -> usize {
        self as usize
    }
}

const RANGE: RangeInclusive<u8> = (File::A as u8)..=(File::H as u8);

impl<T> Index<File> for [T; NrOf::FILES] {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: File) -> &Self::Output {
        &self[index as usize]
    }
}

impl From<usize> for File {
    #[inline(always)]
    fn from(value: usize) -> Self {
        match value {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            6 => Self::G,
            7 => Self::H,
            _ => panic!("Invalid usize for File: {}", value),
        }
    }
}

const fn init_bb_files() -> [Bitboard; NrOf::FILES] {
    let file_a = Bitboard::new(0x0101010101010101);
    let mut files = [Bitboard::empty(); NrOf::FILES];
    let mut i = 0;
    while i < NrOf::FILES {
        files[i] = file_a.shl(i as u64);
        i += 1;
    }
    files
}
