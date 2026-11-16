use crate::board::Board;
use crate::evaluation::Evaluation;
use crate::misc::bits;
use crate::types::bitboard::Bitboard;
use crate::types::nrof::NrOf;
use crate::types::side::Side;
use crate::types::square::Square;

impl Evaluation {
    pub(super) fn evaluate_positions(board: &Board) -> i32 {
        let mut white = 0;
        let mut black = 0;
        for (piece, (w, b)) in board.bitboard_pieces[Side::White]
            .iter()
            .zip(board.bitboard_pieces[Side::Black].iter())
            .enumerate()
        {
            let mut white_pieces = *w;
            while white_pieces > Bitboard::empty() {
                let square = flip_square(bits::next(&mut white_pieces));
                white += SQUARE_VALUES[piece][square];
            }
            let mut black_pieces = *b;
            while black_pieces > Bitboard::empty() {
                let square = bits::next(&mut black_pieces);
                black += SQUARE_VALUES[piece][square];
            }
        }
        white - black
    }
}

fn flip_square(square: Square) -> Square {
    let sq = square.idx();
    Square::from(56 - sq + (sq % 8) * 2)
}

#[rustfmt::skip]
const SQUARE_VALUES: [[i32; NrOf::SQUARES]; NrOf::PIECES] = [
    [
         0,  0,  0,   0,   0,   0,  0,  0,
         0,  0,  0,   0,   0,   0,  0,  0,
         0,  0,  0,   0,   0,   0,  0,  0,
         0,  0,  0,   0,   0,   0,  0,  0,
         0,  0,  0,   0,   0,   0,  0,  0,
         0,  0,  0,   0,   0,   0,  0,  0,
         0,  0,  0,   0,   0,   0,  0,  0,
        20, 25, 20, -20, -20, -20, 20, 15,
    ],
    [
        -30, -20, -10, -10, -10, -10, -20, -30,
        -20, -10,  -5,  -5,  -5,  -5, -10, -20,
        -10,  -5,  10,  10,  10,  10,  -5, -10,
        -10,  -5,  10,  20,  20,  10,  -5, -10,
        -10,  -5,  10,  20,  20,  10,  -5, -10,
        -10,  -5,  -5,  -5,  -5,  -5,  -5, -10,
        -20, -10,  -5,  -5,  -5,  -5, -10, -20,
        -30, -20, -10, -10, -10, -10, -20, -30,
    ],
    [
          0,  0,  0,  0,  0,  0,  0,  0,
         15, 15, 15, 20, 20, 15, 15, 15,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
        -10,  0,  0, 10, 10, 10, 0, -10,
    ],
    [
        -25,  -5,  -5,  -5,  -5,  -5,  -5, -25,
        -20,  -5,  -5,  -5,  -5,  -5,  -5, -20,
        -15,  -5,  -5,   0,   0,  -5,  -5, -15,
        -15,   5,   5,  25,  25,   5,   5, -15,
          0,   0,   5,  20,  20,   5,   0,   0,
          0,   0,   0,   5,   5,   0,   0,   0,
        -15,   0,   0,   5,   5,   0,   0, -15,
        -25, -15, -15, -15, -15, -15, -15, -25,
    ],
    [
        -10,  0, 0,   0,  0,  0,  0, -10,
          0,  5,  5,  5,  5,  5,  5,   0,
          0,  5, 25, 25, 25, 25,  5,   0,
          0,  5, 25, 25, 25, 25,  5,   0,
          0,  5, 25, 25, 25, 25,  5,   0,
          0,  5, 20, 25, 25, 25,  5,   0,
          0,  5,  5,  5,  5,  5,  5,   0,
        -10, 10,  0,  0,  0,  0, 10, -10,
    ],
    [
        0,  0,  0,  0,  0,  0,  0,  0,
       60, 60, 60, 60, 70, 60, 60, 60,
       40, 40, 40, 50, 60, 40, 40, 40,
       20, 20, 20, 40, 50, 20, 20, 20,
        5,  5, 15, 30, 40, 10,  5,  5,
        5,  5, 10, 20, 30,  5,  5,  5,
        5,  5,  5,-30,-30,  5,  5,  5,
        0,  0,  0,  0,  0,  0,  0,  0,
    ]
];
