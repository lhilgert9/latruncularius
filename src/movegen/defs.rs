use std::fmt::Display;

use crate::types::{piece::Piece, square::Square};

#[derive(Clone, Copy, Debug)]
pub struct Move {
    piece: Piece,
    from: Square,
    to: Square,
    captured: Piece,
    promotion: Piece,
    en_passant: bool,
    double_step: bool,
    castling: bool,
}

impl Move {
    pub fn empty() -> Self {
        Self {
            piece: Piece::None,
            from: Square::A1,
            to: Square::A1,
            captured: Piece::None,
            promotion: Piece::None,
            en_passant: false,
            double_step: false,
            castling: false,
        }
    }

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
            piece,
            from,
            to,
            captured,
            en_passant,
            castling,
            double_step,
            promotion,
        }
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn from(&self) -> Square {
        self.from
    }

    pub fn to(&self) -> Square {
        self.to
    }

    pub fn captured(&self) -> Piece {
        self.captured
    }

    pub fn promotion(&self) -> Piece {
        self.promotion
    }

    pub fn en_passant(&self) -> bool {
        self.en_passant
    }

    pub fn double_step(&self) -> bool {
        self.double_step
    }

    pub fn castling(&self) -> bool {
        self.castling
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.from, self.to, self.promotion.promo_str())
    }
}
