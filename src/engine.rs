mod cmdline;
mod main_loop;
mod uci_handler;
mod utils;

use std::sync::{mpsc::Receiver, Arc, Mutex};

use crate::{
    board::{fen::FenResult, Board},
    defs::FEN_START_POSITION,
    engine::cmdline::cmdline,
    misc::perft,
    movegen::MoveGenerator,
    types::error::ErrFatal,
    uci::{
        cmd::{UciIn, UciOut},
        Uci,
    },
};

pub struct Engine {
    quit: bool,
    debug: bool,
    uci: Uci,
    board: Arc<Mutex<Board>>,
    mg: Arc<MoveGenerator>,
    input_rx: Option<Receiver<EngineInput>>,
}

pub enum EngineInput {
    Uci(UciIn),
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
        }
    }

    pub fn run(&mut self) -> FenResult {
        self.board
            .lock()
            .expect(ErrFatal::LOCK)
            .setup_fen(FEN_START_POSITION)
            .expect(ErrFatal::NEW_GAME);

        let args = cmdline();

        if let Some(depth) = args.perft {
            perft::run(depth, Arc::clone(&self.mg))?;
            return Ok(());
        }

        self.main_loop();

        Ok(())
    }
}

impl Engine {
    fn quit(&mut self) {
        self.uci.send(UciOut::Quit);
        self.quit = true;
    }
}
