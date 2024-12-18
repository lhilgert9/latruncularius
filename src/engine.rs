pub mod defs;
mod main_loop;
mod uci_reports;

use std::sync::{mpsc::Receiver, Arc};

use defs::{EngineOption, EngineOptionDefaults, EngineOptionName, Information, UiElement};

use crate::{
    defs::EngineRunResult,
    uci::{Uci, UciControl},
};

pub struct Engine {
    quit: bool,
    options: Arc<Vec<EngineOption>>,
    uci: Uci,
    info_rx: Option<Receiver<Information>>,
}

impl Engine {
    pub fn new() -> Self {
        let is_64_bit = std::mem::size_of::<usize>() == 8;

        let tt_max = if is_64_bit {
            EngineOptionDefaults::HASH_MAX_64_BIT
        } else {
            EngineOptionDefaults::HASH_MAX_32_BIT
        };

        let options = vec![
            EngineOption::new(
                EngineOptionName::HASH,
                UiElement::Spin,
                Some(EngineOptionDefaults::HASH_DEFAULT.to_string()),
                Some(EngineOptionDefaults::HASH_MIN.to_string()),
                Some(tt_max.to_string()),
            ),
            EngineOption::new(
                EngineOptionName::CLEAR_HASH,
                UiElement::Button,
                None,
                None,
                None,
            ),
        ];
        Self {
            quit: false,
            options: Arc::new(options),
            uci: Uci::new(),
            info_rx: None,
        }
    }

    pub fn run(&mut self) -> EngineRunResult {
        self.main_loop();
        Ok(())
    }

    pub fn quit(&mut self) {
        self.uci.send(UciControl::Quit);
        self.quit = true;
    }
}
