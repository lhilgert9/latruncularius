use crate::types::{bitboard::Bitboard, nrof::NrOf};

pub const BB_FILES: [Bitboard; NrOf::FILES] = init_bb_files();

pub enum File {
    A = 0,
    B,
    _C,
    _D,
    _E,
    _F,
    G,
    H,
}

impl File {
    pub const fn idx(self) -> usize {
        self as usize
    }
}

const fn init_bb_files() -> [Bitboard; NrOf::FILES] {
    let file_a = Bitboard::new(0x0101010101010101);
    let mut files = [Bitboard::empty(); NrOf::FILES];
    let mut i = 0;
    while i < NrOf::FILES {
        files[i] = file_a.shl(i as u64);
        i += 1;
    }
    files
}
