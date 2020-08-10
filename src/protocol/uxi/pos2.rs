use crate::{
    cosmic::playing::PosNums,
    engine::Engine,
    log::LogExt,
    look_and_model::position::{PositionLook2a, PositionLook2b},
    protocol::uxi::Pos2,
};
use casual_logger::Log;

impl Pos2 {
    pub fn pos2(engine: &Engine) {
        // 現局面表示
        let s = format!(
            "{}{}{}{}",
            PositionLook2a::to_string(&engine.position, PosNums::Current),
            PositionLook2b::to_string(&engine.position, PosNums::Current),
            engine.position.table.pretty2c(),
            engine.position.table.pretty2d()
        );
        Log::print_notice(&s);
    }
}
