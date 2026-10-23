use crate::{board::game_state::GameState, defs::MAX_GAME_MOVES};

#[derive(Clone, Copy, Debug)]
pub struct History {
    list: [GameState; MAX_GAME_MOVES],
    count: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            list: [GameState::new(); MAX_GAME_MOVES],
            count: 0,
        }
    }

    pub fn push(&mut self, g: GameState) {
        self.list[self.count] = g;
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<GameState> {
        if self.count > 0 {
            self.count -= 1;
            Some(self.list[self.count])
        } else {
            None
        }
    }

    pub fn _len(&self) -> usize {
        self.count
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }
}
