use crate::log::LogExt;
use crate::protocol::usi::IsReady;
use casual_logger::Log;

impl IsReady {
    pub fn isready() {
        Log::print_notice("readyok");
    }
}
