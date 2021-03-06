use crate::{log::LogExt, look_and_model::PosNums, protocol::uxi::Pos, Engine};
use casual_logger::Log;

impl Pos {
    pub fn pos(engine: &Engine) {
        // 現局面表示
        let s = engine.position.pretty1(PosNums::Current);
        Log::print_notice(&s);
    }
}
