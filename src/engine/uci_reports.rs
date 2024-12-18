use crate::uci::{UciControl, UciReport};

use super::Engine;

impl Engine {
    pub fn uci_reports(&mut self, report: &UciReport) {
        match report {
            UciReport::Uci => self.uci.send(UciControl::Identify),
            UciReport::UciNewGame => (),
            UciReport::IsReady => self.uci.send(UciControl::Ready),
            UciReport::Position(fen, moves) => (),
            UciReport::GoInfinite => (),
            UciReport::GoDepth(depth) => (),
            UciReport::GoMoveTime(millis) => (),
            UciReport::GoNodes(nodes) => (),
            UciReport::Stop => (),
            UciReport::Quit => self.quit(),
            UciReport::Unknown => (),
        }
    }
}
