use std::error::Error;
use std::fmt;

use crate::types::piece::Piece;
use crate::types::square::Square;

#[derive(Debug)]
pub enum ParsedMoveError {
    WrongMoveLength,
    CannotConvertSquare,
    CannotConvertPromotionPiece,
}

impl fmt::Display for ParsedMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::WrongMoveLength => "Move must be of length 4 or 5",
            Self::CannotConvertSquare => "Cannot convert square to number",
            Self::CannotConvertPromotionPiece => "Cannot convert promotion piece",
        };
        write!(f, "{error}")
    }
}

impl Error for ParsedMoveError {}

pub type ParsedMove = (Square, Square, Piece);

pub fn uci_move(m: &str) -> Result<ParsedMove, ParsedMoveError> {
    let lower_case = m.to_ascii_lowercase();
    let mut parsed_move: ParsedMove = (Square::from(0u8), Square::from(0u8), Piece::None);

    if m.len() != 4 && m.len() != 5 {
        return Err(ParsedMoveError::WrongMoveLength);
    }

    if let Some(from) = parse_square(&lower_case[0..=1])
        && let Some(to) = parse_square(&lower_case[2..=3])
    {
        parsed_move.0 = Square::from(from);
        parsed_move.1 = Square::from(to);
    } else {
        return Err(ParsedMoveError::CannotConvertSquare);
    }

    if m.len() == 5 {
        let c = lower_case.chars().last().unwrap_or('-');

        if let Some(piece) = parse_promotion(c) {
            parsed_move.2 = piece;
        } else {
            return Err(ParsedMoveError::CannotConvertPromotionPiece);
        }
    }

    Ok(parsed_move)
}

fn parse_square(square: &str) -> Option<usize> {
    Square::name_list().iter().position(|&sq| sq == square)
}

fn parse_promotion(promotion: char) -> Option<Piece> {
    let mut piece = None;

    if let Some(p) = promotion.to_lowercase().next() {
        match p {
            'q' => piece = Some(Piece::Queen),
            'r' => piece = Some(Piece::Rook),
            'b' => piece = Some(Piece::Bishop),
            'n' => piece = Some(Piece::Knight),
            _ => (),
        }
    }

    piece
}
