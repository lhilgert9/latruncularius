use crate::board::Board;
use crate::evaluation::Evaluation;
use crate::movegen::MoveGenerator;
use crate::movegen::defs::Move;
use crate::movegen::move_list::MoveList;
use crate::search::Search;

impl Search {
    pub fn alpha_beta(
        depth: i8,
        mut alpha: i32,
        beta: i32,
        board: &mut Board,
        mg: &MoveGenerator,
    ) -> (i32, Move) {
        if depth <= 0 {
            return (Evaluation::evaluate_position(board), Move::empty());
        }

        let mut best_score = i32::MIN;
        let mut best_move = Move::empty();

        let mut move_list = MoveList::new();
        mg.generate_moves(board, &mut move_list);

        for i in 0..move_list.len() {
            let m = move_list.get_move(i);

            let is_legal = board.make_move(m, mg);

            if !is_legal {
                continue;
            }

            let score = -Search::alpha_beta(depth - 1, -beta, -alpha, board, mg).0;

            board.unmake_last_move();
            if score > best_score {
                best_score = score;
                best_move = m;
                if score >= beta {
                    break;
                }
                if score > alpha {
                    alpha = score;
                }
            }
        }

        (best_score, best_move)
    }
}
