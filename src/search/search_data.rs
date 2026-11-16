use crate::movegen::defs::ShortMove;
use crate::transposition::defs::HashData;

#[derive(Clone, Copy, Default)]
pub struct SearchData {
    depth: u8,
    value: i32,
    best_move: ShortMove,
}

impl HashData for SearchData {
    fn depth(&self) -> u8 {
        self.depth
    }
    fn empty() -> Self {
        SearchData::default()
    }
}

impl SearchData {
    pub fn get(&self) -> (Option<i32>, ShortMove) {
        (Some(self.value), self.best_move)
    }
}
