use crate::{
    defs::FEN_START_POSITION,
    engine::Engine,
    search::defs::{SearchControl, SearchMode, SearchParams},
    types::error::ErrFatal,
    uci::cmd::{UciIn, UciOut},
};

impl Engine {
    pub fn uci_handler(&mut self, cmd: UciIn) {
        match cmd {
            UciIn::Uci => self.uci.send(UciOut::Id),
            UciIn::IsReady => self.uci.send(UciOut::ReadyOk),
            UciIn::DebugOn => self.debug = true,
            UciIn::DebugOff => self.debug = false,
            UciIn::Stop => self.search.send(SearchControl::Stop),
            UciIn::Quit => self.quit(),
            UciIn::UciNewGame => {
                self.board
                    .lock()
                    .expect(ErrFatal::LOCK)
                    .setup_fen(FEN_START_POSITION)
                    .expect(ErrFatal::NEW_GAME);
            }
            UciIn::Position(fen, moves) => {
                let fen_result = self.board.lock().expect(ErrFatal::LOCK).setup_fen(&fen);
                if fen_result.is_ok() {
                    for m in moves.iter() {
                        let result = self.execute_move(m);
                        if !result && self.debug {
                            let fail = format!("Not legal move: {}", m);
                            self.uci.send(UciOut::InfoString(fail));
                            break;
                        }
                    }
                } else if self.debug {
                    self.uci
                        .send(UciOut::InfoString(String::from("FEN setup failed")));
                }
            }
            UciIn::GoInfinite => self.search.send(SearchControl::Start(SearchParams {
                depth: 0,
                mode: SearchMode::Infinite,
            })),
            UciIn::GoDepth(depth) => self.search.send(SearchControl::Start(SearchParams {
                depth,
                mode: SearchMode::Infinite,
            })),
            UciIn::GoNodes(_nodes) => self.search.send(SearchControl::Start(SearchParams {
                depth: 0,
                mode: SearchMode::Infinite,
            })),
            UciIn::GoMoveTime(_time) => self.search.send(SearchControl::Start(SearchParams {
                depth: 0,
                mode: SearchMode::Infinite,
            })),
            UciIn::GoPerft(depth) => self.search.send(SearchControl::Start(SearchParams {
                depth,
                mode: SearchMode::Perft,
            })),
            UciIn::Board => self.uci.send(UciOut::Custom(format!(
                "{}",
                self.board.lock().expect(ErrFatal::LOCK)
            ))),
            UciIn::Unknown(cmd) => self
                .uci
                .send(UciOut::InfoString(format!("Unkown command: '{}'", cmd))),
        };
    }
}
