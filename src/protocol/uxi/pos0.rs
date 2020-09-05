use crate::{log::LogExt, look_and_model::PosNums, protocol::uxi::Pos0, Engine};
use casual_logger::Log;

impl Pos0 {
    pub fn pos0(engine: &Engine) {
        // 初期局面表示
        let s = engine.position.pretty1(PosNums::Start);
        Log::print_notice(&s);
    }
}
