use crate::types::{bitboard::Bitboard, side::EverySide, square::EverySquare};

pub mod defs;
mod init;
pub mod magic_gen;
mod main;
pub mod move_list;
mod pawns;
mod pieces;

#[derive(Clone, Copy, Debug)]
pub struct MoveGenerator {
    king_attacks: EverySquare<Bitboard>,
    knight_attacks: EverySquare<Bitboard>,
    pawn_attacks: EverySide<EverySquare<Bitboard>>,
}

impl MoveGenerator {
    pub fn new() -> Self {
        let mut mg = Self {
            king_attacks: EverySquare::new(Bitboard::empty()),
            knight_attacks: EverySquare::new(Bitboard::empty()),
            pawn_attacks: EverySide::new(EverySquare::new(Bitboard::empty())),
        };

        MoveGenerator::init_king(&mut mg.king_attacks);
        MoveGenerator::init_knight(&mut mg.knight_attacks);
        MoveGenerator::init_pawns(&mut mg.pawn_attacks);

        mg
    }
}
