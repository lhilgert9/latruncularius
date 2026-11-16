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
                self.search.transposition_clear();
            }
            UciIn::Position(fen, moves) => self.position(fen, &moves),
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
            UciIn::SetOption(name, value) => {
                if !name.is_empty() {
                    self.setoption(name, value)
                } else if self.debug {
                    self.uci
                        .send(UciOut::InfoString(String::from("Option must have a name")));
                }
            }
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

impl Engine {
    fn position(&mut self, fen: String, moves: &[String]) {
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

    fn setoption(&mut self, option: String, value: Option<String>) {
        match option {
            option if option == "hash" => {
                if let Some(value) = value {
                    if let Ok(size) = value.parse::<usize>() {
                        self.settings.tt_size = size;
                        self.search.transposition_resize(size);
                    } else if self.debug {
                        self.uci.send(UciOut::InfoString(String::from(
                            "Value must be integer for option",
                        )));
                    }
                } else if self.debug {
                    self.uci.send(UciOut::InfoString(String::from(
                        "Value is required for option",
                    )));
                }
            }
            option if option == "clear hash" => self.search.transposition_clear(),
            _ => {
                if self.debug {
                    self.uci
                        .send(UciOut::InfoString(String::from("Unknown option name")));
                }
            }
        }
    }
}
