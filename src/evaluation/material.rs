use crate::board::Board;
use crate::evaluation::Evaluation;
use crate::types::nrof::NrOf;
use crate::types::side::Side;

const VALUES: [i32; NrOf::PIECES] = [0, 900, 500, 325, 300, 100];

impl Evaluation {
    pub(super) fn evaluate_material(board: &Board) -> i32 {
        let mut score = 0;
        if let Some(white) = board.bitboard_pieces[Side::White]
            .iter()
            .enumerate()
            .map(|(idx, pieces)| pieces.count_ones() as i32 * VALUES[idx])
            .reduce(|a, b| a + b)
        {
            score += white;
        };
        if let Some(black) = board.bitboard_pieces[Side::Black]
            .iter()
            .enumerate()
            .map(|(idx, pieces)| pieces.count_ones() as i32 * VALUES[idx])
            .reduce(|a, b| a + b)
        {
            score -= black;
        };

        score
    }
}
