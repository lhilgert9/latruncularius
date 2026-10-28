use crate::{
    movegen::{
        magics::{Magic, BISHOP_ATTACK_TABLE_SIZE, ROOK_ATTACK_TABLE_SIZE},
        MoveGenerator,
    },
    types::{bitboard::Bitboard, square::Square},
};

pub fn generate_magics() {
    let mut rook = FindMagicContext {
        attack_table_size: ROOK_ATTACK_TABLE_SIZE,
        get_blocker_mask_fn: MoveGenerator::get_rook_blocker_mask,
        get_attack_boards_fn: MoveGenerator::get_rook_attacks,
        successful_magics: Vec::new(),
    };
    println!("Finding magic numbers for rooks...\n");
    find_magic(&mut rook);
    rook.successful_magics
        .iter()
        .for_each(|magic| print_successful_magic(magic));
    println!();

    let mut bishop = FindMagicContext {
        attack_table_size: BISHOP_ATTACK_TABLE_SIZE,
        get_blocker_mask_fn: MoveGenerator::get_bishop_blocker_mask,
        get_attack_boards_fn: MoveGenerator::get_bishop_attacks,
        successful_magics: Vec::new(),
    };
    println!("Finding magic numbers for bishops...\n");
    find_magic(&mut bishop);
    bishop
        .successful_magics
        .iter()
        .for_each(|magic| print_successful_magic(magic));
}

struct SuccessfulMagic {
    square: Square,
    magic_number: u64,
    section_start: u64,
    section_end: u64,
    attempts: u64,
}

struct FindMagicContext {
    pub get_blocker_mask_fn: fn(Square) -> Bitboard,
    pub get_attack_boards_fn: fn(Square, &[Bitboard]) -> Vec<Bitboard>,
    pub attack_table_size: usize,
    pub successful_magics: Vec<SuccessfulMagic>,
}

struct MagicSearchContext<'a> {
    pub attack_table: Vec<Bitboard>,
    pub trial_magic: Magic,
    pub blocker_boards: &'a [Bitboard],
    pub attack_boards: &'a [Bitboard],
    pub permutations: u64,
    pub section_start: u64,
    pub section_end: u64,
}

fn find_magic(context: &mut FindMagicContext) {
    let mut section_start = 0;

    for square in Square::range() {
        let blocker_mask = (context.get_blocker_mask_fn)(Square::from(square));

        let relevant_bits = blocker_mask.count_ones();

        let permutations = 2u64.pow(relevant_bits);

        let section_end = section_start + permutations - 1;

        let blocker_boards = MoveGenerator::generate_blocker_boards(blocker_mask);

        let attack_boards = (context.get_attack_boards_fn)(Square::from(square), &blocker_boards);

        let mut magic_search_context = MagicSearchContext {
            attack_table: vec![Bitboard::empty(); context.attack_table_size],
            trial_magic: Magic {
                mask: blocker_mask,
                shift: (64 - relevant_bits) as u64,
                offset: section_start,
                number: 0,
            },
            attack_boards: &attack_boards,
            blocker_boards: &blocker_boards,
            permutations,
            section_end,
            section_start,
        };

        let attempts = find_working_magic(&mut magic_search_context);

        context.successful_magics.push(SuccessfulMagic {
            square: Square::from(square),
            magic_number: magic_search_context.trial_magic.number,
            section_start,
            section_end,
            attempts,
        });

        section_start += permutations;
    }

    assert!(
        section_start == context.attack_table_size as u64,
        "Creating magics failed. Permutations were skipped."
    );
}

fn find_working_magic(context: &mut MagicSearchContext) -> u64 {
    let mut attempts = 0;
    loop {
        attempts += 1;
        context.trial_magic.number = create_new_magic();

        match validate_magic(context) {
            Ok(()) => return attempts,
            Err(()) => continue,
        }
    }
}

fn validate_magic(context: &mut MagicSearchContext) -> Result<(), ()> {
    for i in 0..context.permutations as usize {
        let index = context.trial_magic.get_index(context.blocker_boards[i]);

        assert!(
            (context.section_start as usize..=context.section_end as usize).contains(&index),
            "Error: Index is out of bounds."
        );

        if context.attack_table[index] != Bitboard::empty() {
            for j in context.section_start..=context.section_end {
                context.attack_table[j as usize] = Bitboard::empty();
            }
            return Err(());
        }
        context.attack_table[index] = context.attack_boards[i];
    }
    Ok(())
}

fn create_new_magic() -> u64 {
    rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>()
}

fn print_successful_magic(magic: &SuccessfulMagic) {
    println!(
        "{}: {:24}u64 (offset: {:6}, end: {:6}, attempts: {})",
        magic.square, magic.magic_number, magic.section_start, magic.section_end, magic.attempts
    );
    // println!("{}u64,", magic.magic_number);
}
