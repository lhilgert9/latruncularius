use crate::{
    movegen::{
        magics::Magic,
        movedata::{KING_MOVES, KNIGHT_MOVES, PAWN_MOVES},
        MoveGenerator,
    },
    types::{
        bitboard::Bitboard,
        nrof::NrOf,
        side::EverySide,
        square::{EverySquare, Square, BB_SQUARES},
    },
};

pub struct InitMagicsContext<'a> {
    pub get_blocker_mask_fn: fn(Square) -> Bitboard,
    pub get_attack_boards_fn: fn(Square, &[Bitboard]) -> Vec<Bitboard>,
    pub magics: &'a mut EverySquare<Magic>,
    pub attacks: &'a mut Vec<Bitboard>,
    pub magic_numbers: [u64; NrOf::SQUARES],
}

impl MoveGenerator {
    pub(super) fn init_king(king_attacks: &mut EverySquare<Bitboard>) {
        for square in Square::range() {
            let from = BB_SQUARES[square];
            let mut moves = Bitboard::empty();

            for current_move in KING_MOVES.iter() {
                let can_move = from & !current_move.impossible;
                let shift = current_move.offset.unsigned_abs() as u64;

                moves |= if current_move.offset > 0 {
                    can_move << shift
                } else {
                    can_move >> shift
                }
            }

            king_attacks[square.into()] = moves;
        }
    }

    pub(super) fn init_knight(knight_attacks: &mut EverySquare<Bitboard>) {
        for square in Square::range() {
            let from = BB_SQUARES[square];
            let mut moves = Bitboard::empty();

            for current_move in KNIGHT_MOVES.iter() {
                let can_move = from & !current_move.impossible;
                let shift = current_move.offset.unsigned_abs() as u64;

                moves |= if current_move.offset > 0 {
                    can_move << shift
                } else {
                    can_move >> shift
                }
            }

            knight_attacks[square.into()] = moves;
        }
    }

    pub(super) fn init_pawns(pawn_attacks: &mut EverySide<EverySquare<Bitboard>>) {
        for square in Square::range() {
            let from = BB_SQUARES[square];

            for side in 0..NrOf::SIDES {
                let mut moves = Bitboard::empty();

                for current_move in &PAWN_MOVES[side] {
                    let can_move = from & !current_move.impossible;
                    let shift = current_move.offset.unsigned_abs() as u64;

                    moves |= if current_move.offset > 0 {
                        can_move << shift
                    } else {
                        can_move >> shift
                    }
                }

                pawn_attacks[side.into()][square.into()] = moves;
            }
        }
    }

    pub(super) fn init_magics(context: InitMagicsContext) {
        let mut section_start = 0;

        for square in Square::range() {
            let blocker_mask = (context.get_blocker_mask_fn)(Square::from(square));

            let relevant_bits = blocker_mask.count_ones();

            let permutations = 2u64.pow(relevant_bits);

            let section_end = section_start + permutations;

            let blocker_boards = MoveGenerator::generate_blocker_boards(blocker_mask);

            let attack_boards =
                (context.get_attack_boards_fn)(Square::from(square), &blocker_boards);

            let magic = Magic {
                mask: blocker_mask,
                shift: (64 - relevant_bits) as u64,
                offset: section_start,
                number: context.magic_numbers[square],
            };

            for (blocker_board, attack_board) in blocker_boards.iter().zip(attack_boards.iter()) {
                let index = magic.get_index(*blocker_board);

                assert!(
                    context.attacks[index] == Bitboard::empty(),
                    "Error in Magics: Attack table index collision."
                );

                assert!(
                    (section_start as usize..section_end as usize).contains(&index),
                    "Error in Magics: Index is out of bounds."
                );

                context.attacks[index] = *attack_board;
            }
            context.magics[square.into()] = magic;

            section_start += permutations;
        }
    }
}
