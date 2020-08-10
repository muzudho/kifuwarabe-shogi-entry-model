use crate::{log::LogExt, protocol::uxi::HowMuch};
use casual_logger::Log;

impl HowMuch {
    pub fn how_much(line: &str) {
        // Example: how-much 7g7f
        let bestmove = &line[9..];
        Log::print_notice(&format!("Debug   | bestmove=|{}|", bestmove));
    }
}
