use std::fmt::Display;

use crate::{
    board::Board,
    types::{
        bitboard::Bitboard,
        castling::CastlingRights,
        piece::Piece,
        side::Side,
        square::{Square, BB_SQUARES},
    },
};

const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";

#[derive(Debug)]
pub enum FenError {
    Length,
    Part1,
    Part2,
    Part3,
    Part4,
    Part5,
    Part6,
}

impl Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            Self::Length => "Error in FEN string: Must be 6 parts",
            Self::Part1 => "Error in FEN Part 1: Pieces or squares",
            Self::Part2 => "Error in FEN Part 2: Colors",
            Self::Part3 => "Error in FEN Part 3: Castling rights",
            Self::Part4 => "Error in FEN Part 4: En passant field",
            Self::Part5 => "Error in FEN Part 5: Half-move clock",
            Self::Part6 => "Error in FEN Part 6: Full-move number",
        };
        write!(f, "{error}")
    }
}

pub type FenResult = Result<(), FenError>;

impl Board {
    pub fn setup_fen(&mut self, fen: &str) -> FenResult {
        let mut fen: Vec<String> = fen.split_whitespace().map(String::from).collect();

        if fen.len() == 4 {
            fen.append(&mut vec![String::from("0"), String::from("1")]);
        }

        if fen.len() != 6 {
            return Err(FenError::Length);
        }

        let mut temp_board = self.clone();
        temp_board.reset();

        pieces(&mut temp_board, &fen[0])?;
        color(&mut temp_board, &fen[1])?;
        castling(&mut temp_board, &fen[2])?;
        en_passant(&mut temp_board, &fen[3])?;
        half_move_clock(&mut temp_board, &fen[4])?;
        full_move_number(&mut temp_board, &fen[5])?;

        temp_board.init();
        *self = temp_board;

        Ok(())
    }
}

fn pieces(board: &mut Board, part: &str) -> FenResult {
    let mut rank = 7 as u8;
    let mut file = 0 as u8;
    for c in part.chars() {
        let square = (rank * 8 + file) as usize;
        match c {
            'k' => board.bitboard_pieces[Side::Black][Piece::King] |= BB_SQUARES[square],
            'q' => board.bitboard_pieces[Side::Black][Piece::Queen] |= BB_SQUARES[square],
            'r' => board.bitboard_pieces[Side::Black][Piece::Rook] |= BB_SQUARES[square],
            'b' => board.bitboard_pieces[Side::Black][Piece::Bishop] |= BB_SQUARES[square],
            'n' => board.bitboard_pieces[Side::Black][Piece::Knight] |= BB_SQUARES[square],
            'p' => board.bitboard_pieces[Side::Black][Piece::Pawn] |= BB_SQUARES[square],
            'K' => board.bitboard_pieces[Side::White][Piece::King] |= BB_SQUARES[square],
            'Q' => board.bitboard_pieces[Side::White][Piece::Queen] |= BB_SQUARES[square],
            'R' => board.bitboard_pieces[Side::White][Piece::Rook] |= BB_SQUARES[square],
            'B' => board.bitboard_pieces[Side::White][Piece::Bishop] |= BB_SQUARES[square],
            'N' => board.bitboard_pieces[Side::White][Piece::Knight] |= BB_SQUARES[square],
            'P' => board.bitboard_pieces[Side::White][Piece::Pawn] |= BB_SQUARES[square],
            '1'..='8' => {
                if let Some(x) = c.to_digit(10) {
                    file += x as u8;
                }
            }
            '/' => {
                if file != 8 {
                    return Err(FenError::Part1);
                }
                file = 0;
                rank -= 1;
            }
            _ => return Err(FenError::Part1),
        }
        if LIST_OF_PIECES.contains(c) {
            file += 1;
        }
    }
    Ok(())
}

fn color(board: &mut Board, part: &str) -> FenResult {
    match part {
        c if c == "w" => board.game_state.active_side = Side::White,
        c if c == "b" => board.game_state.active_side = Side::Black,
        _ => return Err(FenError::Part2),
    }
    Ok(())
}

fn castling(board: &mut Board, part: &str) -> FenResult {
    if (1..=4).contains(&part.len()) {
        for c in part.chars() {
            match c {
                'K' => board.game_state.castling |= CastlingRights::WK,
                'Q' => board.game_state.castling |= CastlingRights::WQ,
                'k' => board.game_state.castling |= CastlingRights::BK,
                'q' => board.game_state.castling |= CastlingRights::BQ,
                '-' => board.game_state.castling |= Bitboard::empty(),
                _ => return Err(FenError::Part3),
            }
        }
        return Ok(());
    }

    Err(FenError::Part3)
}

fn en_passant(board: &mut Board, part: &str) -> FenResult {
    if part.len() == 2 {
        let mut square: usize = 0;
        let (file, rank) = part.split_at(1);
        match "abcdefgh".find(file) {
            Some(idx) => square += idx * 8,
            None => return Err(FenError::Part4),
        }
        match rank.parse::<usize>() {
            Ok(idx) => square += idx,
            Err(_) => return Err(FenError::Part4),
        }
        board.game_state.en_passant = Some(Square::from(square));
    } else if part.len() == 1 && part == "-" {
        board.game_state.en_passant = None;
        return Ok(());
    }
    Err(FenError::Part4)
}

fn half_move_clock(board: &mut Board, part: &str) -> FenResult {
    match part.parse::<u8>() {
        Ok(half_move_clock) => board.game_state.half_move_clock = half_move_clock,
        Err(_) => return Err(FenError::Part5),
    }
    Ok(())
}

fn full_move_number(board: &mut Board, part: &str) -> FenResult {
    match part.parse::<u16>() {
        Ok(full_move_number) => board.game_state.full_move_number = full_move_number,
        Err(_) => return Err(FenError::Part6),
    }
    Ok(())
}
