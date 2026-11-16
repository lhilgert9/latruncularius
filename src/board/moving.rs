use crate::{
    board::Board,
    movegen::{MoveGenerator, defs::Move},
    types::{
        bitboard::Bitboard,
        castling::CastlingRights,
        nrof::NrOf,
        piece::Piece,
        side::Side,
        square::{BB_SQUARES, Square},
    },
};

const CASTLING_RIGHTS: [u64; NrOf::SQUARES] = init_castling_rights();
const fn init_castling_rights() -> [u64; NrOf::SQUARES] {
    let mut cr = [CastlingRights::ALL.as_u64(); NrOf::SQUARES];

    cr[Square::A1.idx()] &= !CastlingRights::WQ.as_u64();
    cr[Square::E1.idx()] &= !CastlingRights::WK.as_u64() & !CastlingRights::WQ.as_u64();
    cr[Square::H1.idx()] &= !CastlingRights::WK.as_u64();
    cr[Square::A8.idx()] &= !CastlingRights::BQ.as_u64();
    cr[Square::E8.idx()] &= !CastlingRights::BK.as_u64() & !CastlingRights::BQ.as_u64();
    cr[Square::H8.idx()] &= !CastlingRights::BK.as_u64();

    cr
}

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

        if self.game_state.en_passant.is_some() {
            self.clear_ep_square();
        }

        if is_capture {
            self.remove_piece(opponent, captured, to);
            self.game_state.half_move_clock = 0;
            if captured == Piece::Rook && self.game_state.castling > Bitboard::empty() {
                self.update_castling(self.game_state.castling & Bitboard::new(CASTLING_RIGHTS[to]));
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
                self.set_ep_square(Square::from(to as u8 ^ 8));
            }
        }

        if (piece == Piece::King || piece == Piece::Rook)
            && self.game_state.castling > Bitboard::empty()
        {
            self.update_castling(self.game_state.castling & Bitboard::new(CASTLING_RIGHTS[from]));
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
            remove_piece(self, us, promotion, to);
            put_piece(self, us, Piece::Pawn, from);
        } else {
            reverse_move(self, us, piece, from, to);
        }

        if castling {
            match to {
                Square::G1 => reverse_move(self, us, Piece::Rook, Square::H1, Square::F1),
                Square::C1 => reverse_move(self, us, Piece::Rook, Square::A1, Square::D1),
                Square::G8 => reverse_move(self, us, Piece::Rook, Square::H8, Square::F8),
                Square::C8 => reverse_move(self, us, Piece::Rook, Square::A8, Square::D8),
                _ => panic!("Error reversing rook castling move"),
            }
        }

        if captured != Piece::None {
            put_piece(self, opponent, captured, to);
        }

        if en_passant {
            put_piece(self, opponent, Piece::Pawn, Square::from(to as u8 ^ 8));
        }
    }
}

impl Board {
    fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.bitboard_pieces[side][piece] ^= BB_SQUARES[square];
        self.bitboard_sides[side] ^= BB_SQUARES[square];
        self.pieces[square] = Piece::None;
        self.game_state.zobrist_key ^= self.zobrist_randoms.piece(side, piece, square);
    }

    fn put_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.bitboard_pieces[side][piece] |= BB_SQUARES[square];
        self.bitboard_sides[side] |= BB_SQUARES[square];
        self.pieces[square] = piece;
        self.game_state.zobrist_key ^= self.zobrist_randoms.piece(side, piece, square);
    }

    fn move_piece(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.put_piece(side, piece, to);
    }

    fn set_ep_square(&mut self, square: Square) {
        self.game_state.zobrist_key ^= self.zobrist_randoms.en_passant(self.game_state.en_passant);
        self.game_state.en_passant = Some(square);
        self.game_state.zobrist_key ^= self.zobrist_randoms.en_passant(self.game_state.en_passant);
    }

    fn clear_ep_square(&mut self) {
        self.game_state.zobrist_key ^= self.zobrist_randoms.en_passant(self.game_state.en_passant);
        self.game_state.en_passant = None;
        self.game_state.zobrist_key ^= self.zobrist_randoms.en_passant(self.game_state.en_passant);
    }

    fn swap_side(&mut self) {
        self.game_state.active_side.flip_mut();
        self.game_state.zobrist_key ^= self.zobrist_randoms.black();
    }

    fn update_castling(&mut self, new_castling_rights: Bitboard) {
        self.game_state.zobrist_key ^= self.zobrist_randoms.castling(self.game_state.castling);
        self.game_state.castling = new_castling_rights;
        self.game_state.zobrist_key ^= self.zobrist_randoms.castling(self.game_state.castling);
    }
}

fn remove_piece(board: &mut Board, side: Side, piece: Piece, square: Square) {
    board.bitboard_pieces[side][piece] ^= BB_SQUARES[square];
    board.bitboard_sides[side] ^= BB_SQUARES[square];
    board.pieces[square] = Piece::None;
}

fn put_piece(board: &mut Board, side: Side, piece: Piece, square: Square) {
    board.bitboard_pieces[side][piece] |= BB_SQUARES[square];
    board.bitboard_sides[side] |= BB_SQUARES[square];
    board.pieces[square] = piece;
}

fn reverse_move(board: &mut Board, side: Side, piece: Piece, from: Square, to: Square) {
    remove_piece(board, side, piece, to);
    put_piece(board, side, piece, from);
}
