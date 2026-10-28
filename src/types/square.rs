use std::{
    fmt::Display,
    ops::{Index, IndexMut, RangeInclusive},
};

use crate::types::{bitboard::Bitboard, nrof::NrOf};

pub const BB_SQUARES: [Bitboard; NrOf::SQUARES] = init_bb_squares();
const fn init_bb_squares() -> [Bitboard; NrOf::SQUARES] {
    let mut bb_squares = [Bitboard::empty(); NrOf::SQUARES];
    let mut i = 0;
    while i < NrOf::SQUARES {
        bb_squares[i] = Bitboard::new(1).shl(i as u64);
        i += 1;
    }

    bb_squares
}

#[derive(Clone, Copy, Debug)]
pub struct EverySquare<T>([T; NrOf::SQUARES]);

impl<T> EverySquare<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self([value; NrOf::SQUARES])
    }
}

impl<T> Index<Square> for EverySquare<T> {
    type Output = T;
    fn index(&self, square: Square) -> &Self::Output {
        &self.0[square as usize]
    }
}

impl<T> IndexMut<Square> for EverySquare<T> {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Square {
  A1 = 0, B1, C1, D1, E1, F1, G1, H1,
      A2, B2, C2, D2, E2, F2, G2, H2,
      A3, B3, C3, D3, E3, F3, G3, H3,
      A4, B4, C4, D4, E4, F4, G4, H4,
      A5, B5, C5, D5, E5, F5, G5, H5,
      A6, B6, C6, D6, E6, F6, G6, H6,
      A7, B7, C7, D7, E7, F7, G7, H7,
      A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    #[inline(always)]
    pub fn idx(self) -> usize {
        self as usize
    }

    #[inline(always)]
    pub fn range() -> RangeInclusive<usize> {
        RANGE
    }
}

const RANGE: RangeInclusive<usize> = (Square::A1 as usize)..=(Square::H8 as usize);

impl<T> Index<Square> for [T; NrOf::SQUARES] {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Square) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Square> for [T; NrOf::SQUARES] {
    #[inline(always)]
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl From<usize> for Square {
    #[rustfmt::skip]
    fn from(value: usize) -> Self {
        match value {
            0 => Square::A1, 1 => Square::B1, 2 => Square::C1, 3 => Square::D1,
            4 => Square::E1, 5 => Square::F1, 6 => Square::G1, 7 => Square::H1,
            8 => Square::A2, 9 => Square::B2, 10 => Square::C2, 11 => Square::D2,
            12 => Square::E2, 13 => Square::F2, 14 => Square::G2, 15 => Square::H2,
            16 => Square::A3, 17 => Square::B3, 18 => Square::C3, 19 => Square::D3,
            20 => Square::E3, 21 => Square::F3, 22 => Square::G3, 23 => Square::H3,
            24 => Square::A4, 25 => Square::B4, 26 => Square::C4, 27 => Square::D4,
            28 => Square::E4, 29 => Square::F4, 30 => Square::G4, 31 => Square::H4,
            32 => Square::A5, 33 => Square::B5, 34 => Square::C5, 35 => Square::D5,
            36 => Square::E5, 37 => Square::F5, 38 => Square::G5, 39 => Square::H5,
            40 => Square::A6, 41 => Square::B6, 42 => Square::C6, 43 => Square::D6,
            44 => Square::E6, 45 => Square::F6, 46 => Square::G6, 47 => Square::H6,
            48 => Square::A7, 49 => Square::B7, 50 => Square::C7, 51 => Square::D7,
            52 => Square::E7, 53 => Square::F7, 54 => Square::G7, 55 => Square::H7,
            56 => Square::A8, 57 => Square::B8, 58 => Square::C8, 59 => Square::D8,
            60 => Square::E8, 61 => Square::F8, 62 => Square::G8, 63 => Square::H8,
            _ => panic!("Invalid usize for Square: {}", value),
        }
    }
}

impl From<u32> for Square {
    fn from(value: u32) -> Self {
        Self::from(value as usize)
    }
}

impl From<u8> for Square {
    fn from(value: u8) -> Self {
        Self::from(value as usize)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let idx = *self as u8;
        let file = (b'A' + (idx % 8)) as char;
        let rank = (b'1' + (idx / 8)) as char;
        write!(f, "{}{}", file, rank)
    }
}
