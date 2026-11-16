pub use crate::search::search_data::SearchData;

use std::fmt::Display;

use crate::movegen::defs::Move;

pub enum SearchControl {
    Quit,
    Stop,
    Start(SearchParams),
}

pub enum SearchReport {
    Finished(Move),
}

pub struct SearchParams {
    pub depth: u8,
    pub mode: SearchMode,
}

#[derive(PartialEq, Eq, Debug)]
pub enum SearchMode {
    Infinite,
    Perft,
}

impl Display for SearchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Infinite => "infinite",
                Self::Perft => "perft",
            }
        )
    }
}
