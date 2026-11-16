use std::ops::{Index, IndexMut};

use crate::types::nrof::NrOf;

#[derive(Clone, Copy, Debug)]
pub struct EveryPiece<T>([T; NrOf::PIECES]);

impl<T> EveryPiece<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self([value; NrOf::PIECES])
    }
}

impl<T> EveryPiece<T> {
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> Index<Piece> for EveryPiece<T> {
    type Output = T;
    fn index(&self, piece: Piece) -> &Self::Output {
        &self.0[piece as usize]
    }
}

impl<T> IndexMut<Piece> for EveryPiece<T> {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6,
}

impl Piece {
    pub fn capital_char(self) -> char {
        const CHARS: [char; 7] = ['K', 'Q', 'R', 'B', 'N', 'I', '-'];
        CHARS[self as usize]
    }

    pub fn small_char(self) -> char {
        const CHARS: [char; 7] = ['k', 'q', 'r', 'b', 'n', 'i', '-'];
        CHARS[self as usize]
    }

    pub fn promo_str(self) -> &'static str {
        match self {
            Self::Queen => "q",
            Self::Rook => "r",
            Self::Bishop => "b",
            Self::Knight => "n",
            _ => "",
        }
    }

    #[inline(always)]
    pub const fn idx(self) -> usize {
        self as usize
    }
}

impl From<usize> for Piece {
    fn from(value: usize) -> Self {
        match value {
            0 => Piece::King,
            1 => Piece::Queen,
            2 => Piece::Rook,
            3 => Piece::Bishop,
            4 => Piece::Knight,
            5 => Piece::Pawn,
            6 => Piece::None,
            _ => panic!("Invalid usize for Piece: {}", value),
        }
    }
}
