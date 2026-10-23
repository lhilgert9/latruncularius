use crate::{
    board::Board,
    movegen::{move_list::MoveList, MoveGenerator},
    types::{side::Side, square::Square},
};

impl MoveGenerator {
    pub fn generate_moves(&self, board: &Board, move_list: &mut MoveList) {
        self.king_moves(board, move_list);
        self.knight_moves(board, move_list);
        self.pawn_pushes(board, move_list);
        self.pawn_captures(board, move_list);
    }

    pub fn _is_square_attacked(&self, _board: &Board, _attacker: Side, _square: Square) -> bool {
        false
    }
}
