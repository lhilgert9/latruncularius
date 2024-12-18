use std::sync::{mpsc, Arc};

use crate::engine::defs::ErrFatal;

use super::{defs::Information, Engine};

impl Engine {
    pub fn main_loop(&mut self) {
        let (info_tx, info_rx) = mpsc::channel::<Information>();
        self.info_rx = Some(info_rx);
        self.uci.start(info_tx, Arc::clone(&self.options));
        while !self.quit {
            let information = &self.info_rx();
            match information {
                Information::Uci(report) => self.uci_reports(report),
            }
        }
        self.uci.wait_for_shutdown();
    }

    fn info_rx(&mut self) -> Information {
        match &self.info_rx {
            Some(i) => i.recv().expect(ErrFatal::CHANNEL),
            None => panic!("{}", ErrFatal::NO_INFO_RX),
        }
    }
}
