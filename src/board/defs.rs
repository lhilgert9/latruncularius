use crate::types::{file::File, rank::Rank};

#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub file: File,
    pub rank: Rank,
}
