use crate::{engine::Engine, log::LogExt, look_and_model::PosNums, protocol::uxi::Pos2};
use casual_logger::Log;

impl Pos2 {
    pub fn pos2(engine: &Engine) {
        // 現局面表示
        let s = format!(
            "{}{}{}{}",
            engine.position.pretty2a(PosNums::Current),
            engine.position.pretty2b(PosNums::Current),
            engine.position.table.pretty2c(),
            engine.position.table.pretty2d()
        );
        Log::print_notice(&s);
    }
}
