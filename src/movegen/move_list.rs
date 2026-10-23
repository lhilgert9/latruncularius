use crate::movegen::defs::Move;

pub struct MoveList {
    list: [Move; 255],
    count: u8,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            list: [Move::empty(); 255],
            count: 0,
        }
    }

    pub fn len(&self) -> u8 {
        self.count
    }

    pub fn get_move(&self, index: u8) -> Move {
        self.list[index as usize]
    }

    pub fn push(&mut self, m: Move) {
        self.list[self.count as usize] = m;
        self.count += 1;
    }
}
