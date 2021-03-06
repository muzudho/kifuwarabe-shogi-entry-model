use crate::{log::LogExt, protocol::uxi::Kifu, Engine};
use casual_logger::Log;

impl Kifu {
    pub fn kifu(engine: &Engine) {
        Log::print_notice("棋譜表示");
        let s = engine.position.get_moves_history_text();
        Log::print_notice(&s);
    }
}
