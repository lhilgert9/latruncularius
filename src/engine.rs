mod main_loop;
mod search_handler;
mod uci_handler;
mod utils;

use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::{
    board::{Board, defs::FenResult},
    defs::FEN_START_POSITION,
    movegen::MoveGenerator,
    search::{
        Search,
        defs::{SearchControl, SearchReport},
    },
    types::error::ErrFatal,
    uci::{
        Uci,
        cmd::{UciIn, UciOut},
    },
};

pub struct Engine {
    quit: bool,
    debug: bool,
    uci: Uci,
    board: Arc<Mutex<Board>>,
    mg: Arc<MoveGenerator>,
    input_rx: Option<Receiver<EngineInput>>,
    search: Search,
}

pub enum EngineInput {
    Uci(UciIn),
    Search(SearchReport),
}

impl Engine {
    pub fn new() -> Self {
        Self {
            quit: false,
            debug: false,
            uci: Uci::new(),
            board: Arc::new(Mutex::new(Board::new())),
            mg: Arc::new(MoveGenerator::new()),
            input_rx: None,
            search: Search::new(),
        }
    }

    pub fn run(&mut self) -> FenResult {
        self.board
            .lock()
            .expect(ErrFatal::LOCK)
            .setup_fen(FEN_START_POSITION)
            .expect(ErrFatal::NEW_GAME);

        self.main_loop();

        Ok(())
    }
}

impl Engine {
    fn quit(&mut self) {
        self.search.send(SearchControl::Quit);
        self.uci.send(UciOut::Quit);
        self.quit = true;
    }
}
