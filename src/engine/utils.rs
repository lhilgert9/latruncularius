use crate::{
    engine::Engine,
    misc::parse::{self, ParsedMove},
    movegen::{defs::Move, move_list::MoveList},
    types::{error::ErrFatal, piece::Piece, square::Square},
};

impl Engine {
    pub fn execute_move(&mut self, m: &str) -> bool {
        let parsed_move =
            parse::uci_move(m).unwrap_or((Square::from(0u8), Square::from(0u8), Piece::None));
        if let Some(m) = self.is_pseudo_legal_move(parsed_move) {
            self.board
                .lock()
                .expect(ErrFatal::LOCK)
                .make_move(m, &self.mg)
        } else {
            false
        }
    }

    pub fn is_pseudo_legal_move(&self, parsed_move: ParsedMove) -> Option<Move> {
        let mut move_list = MoveList::new();
        self.mg
            .generate_moves(&self.board.lock().expect(ErrFatal::LOCK), &mut move_list);
        for i in 0..move_list.len() {
            let m = move_list.get_move(i);
            if parsed_move.0 == m.from()
                && parsed_move.1 == m.to()
                && parsed_move.2 == m.promotion()
            {
                return Some(m);
            }
        }
        None
    }
}
