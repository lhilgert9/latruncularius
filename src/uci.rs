pub mod cmd;
mod input_thread;
mod output_thread;

use std::{sync::mpsc::Sender, thread::JoinHandle};

use crate::{engine::EngineInput, types::error::ErrFatal, uci::cmd::UciOut};

pub struct Uci {
    input_thread: Option<JoinHandle<()>>,
    output_thread: Option<JoinHandle<()>>,
    output_tx: Option<Sender<UciOut>>,
}

impl Uci {
    pub fn new() -> Self {
        Self {
            input_thread: None,
            output_thread: None,
            output_tx: None,
        }
    }

    pub fn init(&mut self, info_tx: Sender<EngineInput>) {
        self.input_thread(info_tx);
        self.output_thread();
    }

    pub fn send(&mut self, cmd: UciOut) {
        if let Some(output_tx) = &self.output_tx {
            output_tx.send(cmd).expect(ErrFatal::CHANNEL);
        }
    }

    pub fn shutdown(&mut self) {
        if let Some(h) = self.input_thread.take() {
            h.join().expect(ErrFatal::THREAD)
        }

        if let Some(h) = self.output_thread.take() {
            h.join().expect(ErrFatal::THREAD)
        }
    }
}
