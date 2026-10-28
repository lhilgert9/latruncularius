use crate::{
    board::Board,
    movegen::{defs::Move, MoveGenerator},
    types::{
        bitboard::Bitboard,
        castling::CastlingRights,
        piece::Piece,
        side::Side,
        square::{Square, BB_SQUARES},
    },
};

impl Board {
    pub fn make_move(&mut self, m: Move, mg: &MoveGenerator) -> bool {
        let mut current_game_state = self.game_state;
        current_game_state.next_move = m;
        self.history.push(current_game_state);

        let us = self.us();
        let opponent = self.opponent();

        let piece = m.piece();
        let from = m.from();
        let to = m.to();
        let captured = m.captured();
        let promotion = m.promotion();
        let castling = m.castling();
        let double_step = m.double_step();
        let en_passant = m.en_passant();

        let is_promotion = promotion != Piece::None;
        let is_capture = captured != Piece::None;

        self.game_state.half_move_clock += 1;
        self.game_state.en_passant = None;

        if is_capture {
            self.remove_piece(opponent, captured, to);
            self.game_state.half_move_clock = 0;
            if (piece == Piece::Rook) && self.game_state.castling > Bitboard::empty() {
                match from {
                    Square::A1 => {
                        self.game_state.castling &= CastlingRights::ALL & !CastlingRights::WQ
                    }
                    Square::H1 => {
                        self.game_state.castling &= CastlingRights::ALL & !CastlingRights::WK
                    }
                    Square::A8 => {
                        self.game_state.castling &= CastlingRights::ALL & !CastlingRights::BQ
                    }
                    Square::H8 => {
                        self.game_state.castling &= CastlingRights::ALL & !CastlingRights::BK
                    }
                    _ => (),
                }
            }
        }

        if piece != Piece::Pawn {
            self.move_piece(us, piece, from, to);
        } else {
            self.remove_piece(us, piece, from);
            self.put_piece(us, if !is_promotion { piece } else { promotion }, to);

            self.game_state.half_move_clock = 0;

            if en_passant {
                self.remove_piece(opponent, Piece::Pawn, Square::from(to as u8 ^ 8));
            }

            if double_step {
                self.game_state.en_passant = Some(Square::from(to as u8 ^ 8))
            }
        }

        if (piece == Piece::King || piece == Piece::Rook)
            && self.game_state.castling > Bitboard::empty()
        {
            match from {
                Square::A1 => self.game_state.castling &= CastlingRights::ALL & !CastlingRights::WQ,
                Square::E1 => {
                    self.game_state.castling &=
                        CastlingRights::ALL & !CastlingRights::WQ & !CastlingRights::WK
                }
                Square::H1 => self.game_state.castling &= CastlingRights::ALL & !CastlingRights::WK,
                Square::A8 => self.game_state.castling &= CastlingRights::ALL & !CastlingRights::BQ,
                Square::E8 => {
                    self.game_state.castling &=
                        CastlingRights::ALL & !CastlingRights::BQ & !CastlingRights::BK
                }
                Square::H8 => self.game_state.castling &= CastlingRights::ALL & !CastlingRights::BK,
                _ => (),
            }
        }

        if castling {
            match to {
                Square::G1 => self.move_piece(us, Piece::Rook, Square::H1, Square::F1),
                Square::C1 => self.move_piece(us, Piece::Rook, Square::A1, Square::D1),
                Square::G8 => self.move_piece(us, Piece::Rook, Square::H8, Square::F8),
                Square::C8 => self.move_piece(us, Piece::Rook, Square::A8, Square::D8),
                _ => panic!("Error moving rook during castling"),
            }
        }

        self.swap_side();

        if us == Side::Black {
            self.game_state.full_move_number += 1;
        }

        let is_legal = !mg.is_square_attacked(self, opponent, self.king_square(us));
        if !is_legal {
            self.unmake_last_move();
        }

        is_legal
    }

    pub fn unmake_last_move(&mut self) {
        if let Some(game_state) = self.history.pop() {
            self.game_state = game_state;
        } else {
            return;
        }

        let us = self.us();
        let opponent = self.opponent();

        let m = self.game_state.next_move;
        let piece = m.piece();
        let from = m.from();
        let to = m.to();
        let captured = m.captured();
        let promotion = m.promotion();
        let castling = m.castling();
        let en_passant = m.en_passant();

        if promotion != Piece::None {
            self.remove_piece(us, promotion, to);
            self.put_piece(us, Piece::Pawn, from);
        } else {
            self.move_piece(us, piece, to, from);
        }

        if castling {
            match to {
                Square::G1 => self.move_piece(us, Piece::Rook, Square::F1, Square::H1),
                Square::C1 => self.move_piece(us, Piece::Rook, Square::D1, Square::A1),
                Square::G8 => self.move_piece(us, Piece::Rook, Square::F8, Square::H8),
                Square::C8 => self.move_piece(us, Piece::Rook, Square::D8, Square::A8),
                _ => panic!("Error reversing rook castling move"),
            }
        }

        if captured != Piece::None {
            self.put_piece(opponent, captured, to);
        }

        if en_passant {
            self.put_piece(opponent, Piece::Pawn, Square::from(to as u8 ^ 8));
        }
    }
}

impl Board {
    fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.bitboard_pieces[side][piece] ^= BB_SQUARES[square];
        self.bitboard_sides[side] ^= BB_SQUARES[square];
        self.pieces[square] = Piece::None;
    }

    fn put_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.bitboard_pieces[side][piece] |= BB_SQUARES[square];
        self.bitboard_sides[side] |= BB_SQUARES[square];
        self.pieces[square] = piece;
    }

    fn move_piece(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.put_piece(side, piece, to);
    }

    fn swap_side(&mut self) {
        self.game_state.active_side.flip_mut();
    }
}
