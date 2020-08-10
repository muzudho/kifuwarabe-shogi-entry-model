use crate::log::LogExt;
use crate::usi_protocol::IsReady;
use casual_logger::Log;

impl IsReady {
    pub fn isready() {
        Log::print_notice("readyok");
    }
}
