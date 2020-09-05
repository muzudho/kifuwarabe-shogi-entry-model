use crate::{log::LogExt, protocol::uxi::Undo, Engine};
use casual_logger::Log;

impl Undo {
    pub fn undo(engine: &mut Engine) {
        if !engine.position.undo_move() {
            Log::print_notice(&format!(
                "info string ply={} を、これより戻せません",
                engine.position.history.length_from_the_middle()
            ));
        }
    }
}
