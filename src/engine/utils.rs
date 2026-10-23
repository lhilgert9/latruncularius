use crate::{
    engine::Engine,
    movegen::{defs::Move, move_list::MoveList},
    types::error::ErrFatal,
};

impl Engine {
    pub fn execute_move(&mut self, _m: &str) -> bool {
        if let Some(m) = self.is_pseudo_legal_move() {
            self.board.lock().expect(ErrFatal::LOCK).make_move(m)
        } else {
            false
        }
    }

    pub fn is_pseudo_legal_move(&self) -> Option<Move> {
        let mut move_list = MoveList::new();
        self.mg
            .generate_moves(&self.board.lock().expect(ErrFatal::LOCK), &mut move_list);
        for i in 0..move_list.len() {
            let _m = move_list.get_move(i);
        }
        None
    }
}
