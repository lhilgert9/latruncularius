mod alpha_beta;
pub mod defs;
mod search_data;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;

use crate::board::Board;
use crate::engine::EngineInput;
use crate::misc::perft;
use crate::movegen::MoveGenerator;
use crate::search::defs::SearchControl;
use crate::search::defs::SearchData;
use crate::search::defs::SearchMode;
use crate::search::defs::SearchReport;
use crate::transposition::defs::Transposition;
use crate::types::error::ErrFatal;

pub struct Search {
    search_thread: Option<JoinHandle<()>>,
    control_tx: Option<Sender<SearchControl>>,
    transposition: Arc<Mutex<Transposition<SearchData>>>,
}

impl Search {
    pub fn new(tt_size: usize) -> Self {
        Self {
            search_thread: None,
            control_tx: None,
            transposition: Arc::new(Mutex::new(Transposition::<SearchData>::new(tt_size))),
        }
    }

    pub fn init(
        &mut self,
        report_tx: Sender<EngineInput>,
        board: Arc<Mutex<Board>>,
        mg: Arc<MoveGenerator>,
    ) {
        let (control_tx, control_rx) = channel::<SearchControl>();

        let arc_board = Arc::clone(&board);

        let h = thread::spawn(move || {
            let mut quit = false;
            while !quit {
                let cmd = control_rx.recv().expect(ErrFatal::CHANNEL);
                match cmd {
                    SearchControl::Quit => quit = true,
                    SearchControl::Start(sp) => {
                        let mtx_board = arc_board.lock().expect(ErrFatal::LOCK);
                        let mut board = mtx_board.clone();
                        std::mem::drop(mtx_board);
                        if sp.mode == SearchMode::Perft {
                            perft::run(sp.depth, &mut board, Arc::clone(&mg), 1024);
                        } else {
                            let best_move =
                                Search::alpha_beta(8, i32::MIN, i32::MAX, &mut board, &mg).1;
                            report_tx
                                .send(EngineInput::Search(SearchReport::Finished(best_move)))
                                .expect(ErrFatal::CHANNEL);
                        }
                    }
                    _ => (),
                }
            }
        });
        self.search_thread = Some(h);
        self.control_tx = Some(control_tx);
    }

    pub fn transposition_clear(&mut self) {
        self.transposition.lock().expect(ErrFatal::LOCK).clear();
    }

    pub fn transposition_resize(&mut self, megabytes: usize) {
        self.transposition
            .lock()
            .expect(ErrFatal::LOCK)
            .resize(megabytes);
    }

    pub fn send(&mut self, cmd: SearchControl) {
        if let Some(control_tx) = &self.control_tx {
            control_tx.send(cmd).expect(ErrFatal::CHANNEL);
        }
    }

    pub fn shutdown(&mut self) {
        if let Some(h) = self.search_thread.take() {
            h.join().expect(ErrFatal::THREAD)
        }
    }
}
