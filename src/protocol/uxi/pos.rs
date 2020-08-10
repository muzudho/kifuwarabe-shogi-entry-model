use crate::{cosmic::playing::PosNums, engine::Engine, log::LogExt, protocol::uxi::Pos};
use casual_logger::Log;

impl Pos {
    pub fn pos(engine: &Engine) {
        // 現局面表示
        let s = engine.position.pretty1(PosNums::Current);
        Log::print_notice(&s);
    }
}
