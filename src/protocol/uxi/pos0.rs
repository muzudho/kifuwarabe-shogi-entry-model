use crate::cosmic::playing::PosNums;
use crate::engine::Engine;
use crate::log::LogExt;
use crate::look_and_model::position::PositionLook;
use crate::protocol::uxi::Pos0;
use casual_logger::Log;

impl Pos0 {
    pub fn pos0(engine: &Engine) {
        // 初期局面表示
        let s = PositionLook::to_string(&engine.position, PosNums::Start);
        Log::print_notice(&s);
    }
}