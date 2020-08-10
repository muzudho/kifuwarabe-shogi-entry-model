use crate::engine::Engine;
use crate::log::LogExt;
use crate::protocol::uxi::Hash;
use casual_logger::Log;

impl Hash {
    pub fn hash(engine: &Engine) {
        Log::print_notice("局面ハッシュ表示");
        let s = engine.position.get_positions_hash_text();
        Log::print_notice(&s);
    }
}
