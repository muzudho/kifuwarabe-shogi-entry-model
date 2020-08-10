use crate::cosmic::playing::PosNums;
use crate::engine::Engine;
use crate::log::LogExt;
use crate::look_and_model::position::PositionLook;
use crate::protocol::uxi::Pos;
use casual_logger::Log;

impl Pos {
    pub fn pos(engine: &Engine) {
        // 現局面表示
        let s = PositionLook::to_string(&engine.position, PosNums::Current);
        Log::print_notice(&s);
    }
}
