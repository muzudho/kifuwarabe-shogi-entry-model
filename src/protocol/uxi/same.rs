use crate::{engine::Engine, log::LogExt, protocol::uxi::Same};
use casual_logger::Log;

impl Same {
    pub fn same(engine: &Engine) {
        let count = engine.position.count_same_position();
        Log::print_notice(&format!("同一局面調べ count={}", count));
    }
}
