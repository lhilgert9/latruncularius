use std::fmt::Display;

use crate::types::{piece::Piece, square::Square};

struct Shift;
impl Shift {
    pub const PIECE: usize = 0;
    pub const FROM_SQ: usize = 3;
    pub const TO_SQ: usize = 9;
    pub const CAPTURE: usize = 15;
    pub const PROMOTION: usize = 18;
    pub const EN_PASSANT: usize = 21;
    pub const DOUBLE_STEP: usize = 22;
    pub const CASTLING: usize = 23;
    pub const SORTSCORE: usize = 24;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    data: usize,
}

impl Move {
    #[inline(always)]
    pub fn empty() -> Self {
        Self { data: 0 }
    }

    #[inline(always)]
    pub fn new(
        piece: Piece,
        from: Square,
        to: Square,
        captured: Piece,
        promotion: Piece,
        en_passant: bool,
        double_step: bool,
        castling: bool,
    ) -> Self {
        Self {
            data: (castling as usize) << Shift::CASTLING
                | (double_step as usize) << Shift::DOUBLE_STEP
                | (en_passant as usize) << Shift::EN_PASSANT
                | (promotion as usize) << Shift::PROMOTION
                | (captured as usize) << Shift::CAPTURE
                | (to as usize) << Shift::TO_SQ
                | (from as usize) << Shift::FROM_SQ
                | (piece as usize) << Shift::PIECE,
        }
    }

    #[inline(always)]
    pub fn piece(&self) -> Piece {
        Piece::from((self.data >> Shift::PIECE as u64) & 0x7)
    }

    #[inline(always)]
    pub fn from(&self) -> Square {
        Square::from((self.data >> Shift::FROM_SQ as u64) & 0x3F)
    }

    #[inline(always)]
    pub fn to(&self) -> Square {
        Square::from((self.data >> Shift::TO_SQ as u64) & 0x3F)
    }

    #[inline(always)]
    pub fn captured(&self) -> Piece {
        Piece::from((self.data >> Shift::CAPTURE as u64) & 0x7)
    }

    #[inline(always)]
    pub fn promotion(&self) -> Piece {
        Piece::from((self.data >> Shift::PROMOTION as u64) & 0x7)
    }

    #[inline(always)]
    pub fn en_passant(&self) -> bool {
        ((self.data >> Shift::EN_PASSANT as u64) & 0x1) as u8 == 1
    }

    #[inline(always)]
    pub fn double_step(&self) -> bool {
        ((self.data >> Shift::DOUBLE_STEP as u64) & 0x1) as u8 == 1
    }

    #[inline(always)]
    pub fn castling(&self) -> bool {
        ((self.data >> Shift::CASTLING as u64) & 0x1) as u8 == 1
    }

    #[inline(always)]
    pub fn _to_short_move(&self) -> ShortMove {
        ShortMove::new((self.data & 0xFF_FF_FF) as u32)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.from().name(),
            self.to().name(),
            self.promotion().promo_str()
        )
    }
}

#[derive(Clone, Copy, Default)]
pub struct ShortMove {
    data: u32,
}

impl ShortMove {
    pub fn new(data: u32) -> Self {
        Self { data }
    }
}
