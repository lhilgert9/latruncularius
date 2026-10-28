use crate::{
    board::{defs::Location, Board},
    movegen::MoveGenerator,
    types::{
        bitboard::Bitboard,
        file::{File, BB_FILES},
        rank::{Rank, BB_RANKS},
        square::{Square, BB_SQUARES},
    },
};

impl MoveGenerator {
    pub(super) fn get_rook_blocker_mask(square: Square) -> Bitboard {
        let location = Board::file_and_rank(square);
        let rook_square = BB_SQUARES[square];
        let edges = MoveGenerator::board_edges_without_pieces(location);
        let mask = BB_FILES[location.file] | BB_RANKS[location.rank];
        mask & !edges & !rook_square
    }

    pub(super) fn get_bishop_blocker_mask(square: Square) -> Bitboard {
        let location = Board::file_and_rank(square);
        let bishop_square = BB_SQUARES[square];
        let edges = MoveGenerator::board_edges_without_pieces(location);

        let empty = Bitboard::empty();
        let up_right = MoveGenerator::bitboard_ray(square, Direction::UpRight, empty);
        let down_right = MoveGenerator::bitboard_ray(square, Direction::DownRight, empty);
        let down_left = MoveGenerator::bitboard_ray(square, Direction::DownLeft, empty);
        let up_left = MoveGenerator::bitboard_ray(square, Direction::UpLeft, empty);

        (up_right | down_right | down_left | up_left) & !edges & !bishop_square
    }

    pub(super) fn generate_blocker_boards(blocker_mask: Bitboard) -> Vec<Bitboard> {
        let mut blocker_boards = Vec::new();
        let mut n = Bitboard::empty();

        loop {
            blocker_boards.push(n);
            n = n.wrapping_sub(blocker_mask) & blocker_mask;
            if n == Bitboard::empty() {
                break;
            }
        }

        blocker_boards
    }

    pub(super) fn get_rook_attack_boards(square: Square, blockers: &[Bitboard]) -> Vec<Bitboard> {
        let mut attacks = Vec::new();

        for &blocker in blockers {
            let attack = MoveGenerator::bitboard_ray(square, Direction::Up, blocker)
                | MoveGenerator::bitboard_ray(square, Direction::Right, blocker)
                | MoveGenerator::bitboard_ray(square, Direction::Down, blocker)
                | MoveGenerator::bitboard_ray(square, Direction::Left, blocker);
            attacks.push(attack);
        }

        attacks
    }

    pub(super) fn get_bishop_attack_boards(square: Square, blockers: &[Bitboard]) -> Vec<Bitboard> {
        let mut attacks = Vec::new();
        for &blocker in blockers {
            let attack = MoveGenerator::bitboard_ray(square, Direction::UpRight, blocker)
                | MoveGenerator::bitboard_ray(square, Direction::DownRight, blocker)
                | MoveGenerator::bitboard_ray(square, Direction::DownLeft, blocker)
                | MoveGenerator::bitboard_ray(square, Direction::UpLeft, blocker);
            attacks.push(attack);
        }
        attacks
    }
}

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

struct DirectionContext {
    file_step: i8,
    rank_step: i8,
    shift: u64,
    shift_direction: ShiftDirection,
}

enum ShiftDirection {
    Left,
    Right,
}

impl MoveGenerator {
    fn board_edges_without_pieces(location: Location) -> Bitboard {
        let piece_file = BB_FILES[location.file];
        let piece_rank = BB_RANKS[location.rank];
        (BB_FILES[File::A] & !piece_file)
            | (BB_FILES[File::H] & !piece_file)
            | (BB_RANKS[Rank::R1] & !piece_rank)
            | (BB_RANKS[Rank::R8] & !piece_rank)
    }

    fn bitboard_ray(square: Square, direction: Direction, blocker: Bitboard) -> Bitboard {
        let location = Board::file_and_rank(square);
        let direction_context = MoveGenerator::direction_context(direction);
        let mut file = location.file as i8;
        let mut rank = location.rank as i8;
        let mut bitboard_square = BB_SQUARES[square];
        let mut ray = Bitboard::empty();

        loop {
            file += direction_context.file_step;
            rank += direction_context.rank_step;

            if file < File::A as i8
                || file > File::H as i8
                || rank < Rank::R1 as i8
                || rank > Rank::R8 as i8
            {
                break;
            }

            bitboard_square = match direction_context.shift_direction {
                ShiftDirection::Left => bitboard_square << direction_context.shift,
                ShiftDirection::Right => bitboard_square >> direction_context.shift,
            };

            ray |= bitboard_square;

            if (bitboard_square & blocker) != Bitboard::empty() {
                break;
            }
        }

        ray
    }

    fn direction_context(direction: Direction) -> DirectionContext {
        match direction {
            Direction::Up => DirectionContext {
                file_step: 0,
                rank_step: 1,
                shift: 8,
                shift_direction: ShiftDirection::Left,
            },
            Direction::UpRight => DirectionContext {
                file_step: 1,
                rank_step: 1,
                shift: 9,
                shift_direction: ShiftDirection::Left,
            },
            Direction::Right => DirectionContext {
                file_step: 1,
                rank_step: 0,
                shift: 1,
                shift_direction: ShiftDirection::Left,
            },
            Direction::DownRight => DirectionContext {
                file_step: 1,
                rank_step: -1,
                shift: 7,
                shift_direction: ShiftDirection::Right,
            },
            Direction::Down => DirectionContext {
                file_step: 0,
                rank_step: -1,
                shift: 8,
                shift_direction: ShiftDirection::Right,
            },
            Direction::DownLeft => DirectionContext {
                file_step: -1,
                rank_step: -1,
                shift: 9,
                shift_direction: ShiftDirection::Right,
            },
            Direction::Left => DirectionContext {
                file_step: -1,
                rank_step: 0,
                shift: 1,
                shift_direction: ShiftDirection::Right,
            },
            Direction::UpLeft => DirectionContext {
                file_step: -1,
                rank_step: 1,
                shift: 7,
                shift_direction: ShiftDirection::Left,
            },
        }
    }
}
