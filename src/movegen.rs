use crate::{
    movegen::{
        init::InitMagicsContext,
        magics::{
            Magic, BISHOP_ATTACK_TABLE_SIZE, BISHOP_MAGIC_NRS, ROOK_ATTACK_TABLE_SIZE,
            ROOK_MAGIC_NRS,
        },
    },
    types::{bitboard::Bitboard, side::EverySide, square::EverySquare},
};

mod attacks;
pub mod defs;
mod init;
mod magic_helper;
mod magics;
mod main;
pub mod move_list;
mod movedata;
mod pawns;
mod pieces;
#[cfg(feature = "wizardry")]
pub mod wizardry;

pub struct MoveGenerator {
    king_attacks: EverySquare<Bitboard>,
    knight_attacks: EverySquare<Bitboard>,
    pawn_attacks: EverySide<EverySquare<Bitboard>>,
    rook_attacks: Vec<Bitboard>,
    bishop_attacks: Vec<Bitboard>,
    rook_magics: EverySquare<Magic>,
    bishop_magics: EverySquare<Magic>,
}

impl MoveGenerator {
    pub fn new() -> Self {
        let mut mg = Self {
            king_attacks: EverySquare::new(Bitboard::empty()),
            knight_attacks: EverySquare::new(Bitboard::empty()),
            pawn_attacks: EverySide::new(EverySquare::new(Bitboard::empty())),
            rook_attacks: vec![Bitboard::empty(); ROOK_ATTACK_TABLE_SIZE],
            bishop_attacks: vec![Bitboard::empty(); BISHOP_ATTACK_TABLE_SIZE],
            rook_magics: EverySquare::new(Default::default()),
            bishop_magics: EverySquare::new(Default::default()),
        };

        let rook = InitMagicsContext {
            get_blocker_mask_fn: MoveGenerator::get_rook_blocker_mask,
            get_attack_boards_fn: MoveGenerator::get_rook_attack_boards,
            magics: &mut mg.rook_magics,
            attacks: &mut mg.rook_attacks,
            magic_numbers: ROOK_MAGIC_NRS,
        };
        let bishop = InitMagicsContext {
            get_blocker_mask_fn: MoveGenerator::get_bishop_blocker_mask,
            get_attack_boards_fn: MoveGenerator::get_bishop_attack_boards,
            magics: &mut mg.bishop_magics,
            attacks: &mut mg.bishop_attacks,
            magic_numbers: BISHOP_MAGIC_NRS,
        };

        MoveGenerator::init_king(&mut mg.king_attacks);
        MoveGenerator::init_knight(&mut mg.knight_attacks);
        MoveGenerator::init_pawns(&mut mg.pawn_attacks);
        MoveGenerator::init_magics(rook);
        MoveGenerator::init_magics(bishop);

        mg
    }
}
