// engine errors
pub type EngineRunResult = Result<(), u8>;
pub const ENGINE_RUN_ERRORS: [&str; 7] = [
    "FEN: Must have six parts",
    "FEN: Pieces and squares incorrect",
    "FEN: Color selection incorrect",
    "FEN: Castling permissions incorrect",
    "FEN: En-passant square incorrect",
    "FEN: Half-move clock incorrect",
    "FEN: Full-move number incorrect",
];

pub struct About;
impl About {
    pub const ENGINE: &'static str = "Latruncularius";
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const AUTHOR: &'static str = "Lucas Hilgert";
}
