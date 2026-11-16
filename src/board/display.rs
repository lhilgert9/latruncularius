use std::fmt::Display;

use crate::{
    board::Board,
    evaluation::Evaluation,
    types::{bitboard::Bitboard, castling::CastlingRights, nrof::NrOf, piece::Piece, side::Side},
};

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty = self.pretty_string();
        let meta = self.meta_string();
        write!(f, "{pretty}{meta}")
    }
}

impl Board {
    fn pretty_string(&self) -> String {
        let mut pretty = String::from("");
        let mut board = ['.'; NrOf::SQUARES];
        let rank_coordinates = "ABCDEFGH";

        for (piece, &bb) in self.bitboard_pieces[Side::White].iter().enumerate() {
            for (i, square) in board.iter_mut().enumerate() {
                if (bb >> i as u64) & Bitboard::new(1) == Bitboard::new(1) {
                    *square = Piece::from(piece).capital_char();
                }
            }
        }
        for (piece, &bb) in self.bitboard_pieces[Side::Black].iter().enumerate() {
            for (i, square) in board.iter_mut().enumerate() {
                if (bb >> i as u64) & Bitboard::new(1) == Bitboard::new(1) {
                    *square = Piece::from(piece).small_char();
                }
            }
        }

        pretty += "\n";

        for rank in (1..=8).rev() {
            pretty.push_str(format!("{rank}   ").as_str());
            for file in 0..8 {
                let character = board[(((rank - 1) * 8) + file) as usize];
                pretty.push_str(format!("{character} ").as_str());
            }
            pretty += "\n"
        }

        pretty += "\n    ";

        for c in rank_coordinates.chars() {
            pretty.push_str(format!("{} ", c).as_str());
        }

        pretty += "\n\n";

        pretty
    }

    fn meta_string(&self) -> String {
        let mut meta = String::from("");
        let active_color = match self.game_state.active_side {
            Side::White => "White",
            Side::Black => "Black",
        };

        let mut castling = String::from("");
        let c = self.game_state.castling;
        if c & CastlingRights::WK > Bitboard::empty() {
            castling += "K"
        };
        if c & CastlingRights::WQ > Bitboard::empty() {
            castling += "Q"
        };
        if c & CastlingRights::BK > Bitboard::empty() {
            castling += "k"
        };
        if c & CastlingRights::BQ > Bitboard::empty() {
            castling += "q"
        };
        if castling.len() == 0 {
            castling = String::from("-")
        }

        let en_passant = match self.game_state.en_passant {
            Some(_) => "",
            None => "-",
        };

        let hmc = self.game_state.half_move_clock;
        let fmn = self.game_state.full_move_number;

        meta.push_str(format!("{:<20}{}\n", "Active Color:", active_color).as_str());
        meta.push_str(format!("{:<20}{}\n", "Castling:", castling).as_str());
        meta.push_str(format!("{:<20}{}\n", "En Passant:", en_passant).as_str());
        meta.push_str(format!("{:<20}{}\n", "Half-move clock:", hmc).as_str());
        meta.push_str(format!("{:<20}{}\n", "Full-move number:", fmn).as_str());
        meta.push_str(
            format!(
                "{:<20}{}\n",
                "Evaluation:",
                Evaluation::evaluate_position(self)
            )
            .as_str(),
        );
        meta
    }
}
