use crate::engine::Engine;
use crate::search::defs::SearchReport;
use crate::uci::cmd::UciOut;

impl Engine {
    pub fn search_handler(&mut self, cmd: SearchReport) {
        match cmd {
            SearchReport::Finished(best_move) => self.uci.send(UciOut::BestMove(best_move)),
        }
    }
}
