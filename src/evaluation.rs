mod material;
mod positions;

use crate::board::Board;
use crate::types::side::Side;

pub struct Evaluation;

impl Evaluation {
    pub fn evaluate_position(board: &Board) -> i32 {
        let side = board.game_state.active_side;
        let material = Evaluation::evaluate_material(board);
        let positions = Evaluation::evaluate_positions(board);
        let mut value = material + positions;
        value = if side == Side::Black { -value } else { value };
        value
    }
}
