#[derive(Eq, PartialEq)]
pub enum UciIn {
    Uci,
    IsReady,
    UciNewGame,
    Position(String, Vec<String>),
    // SetOption(String, Option<String>),
    // GoInfinite,
    // GoDepth(i8),
    // GoMoveTime(u128),
    // GoNodes(usize),
    // GoGameTime(GameTime),
    DebugOff,
    DebugOn,
    Stop,
    Quit,
    Board,
    Unknown(String),
}

#[derive(Eq, PartialEq)]
pub enum UciOut {
    Id,
    ReadyOk,
    // SearchCurrMove(SearchCurrentMove),
    // SearchSummary(SearchSummary),
    // SearchStats(SearchStats),
    // BestMove(Move),
    InfoString(String),
    Quit,
    Custom(String),
}
