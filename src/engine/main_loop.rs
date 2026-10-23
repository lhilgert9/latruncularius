use std::sync::mpsc::channel;

use crate::{
    engine::{Engine, EngineInput},
    types::error::ErrFatal,
};

impl Engine {
    pub fn main_loop(&mut self) {
        let (input_tx, input_rx) = channel::<EngineInput>();
        self.uci.init(input_tx);

        self.input_rx = Some(input_rx);

        while !self.quit {
            let incoming = match &self.input_rx {
                Some(i) => i.recv(),
                None => panic!("{}", ErrFatal::NO_INFO_RX),
            };
            if let Ok(i) = incoming {
                match i {
                    EngineInput::Uci(cmd) => self.uci_handler(cmd),
                }
            }
        }
        self.uci.shutdown();
    }
}
