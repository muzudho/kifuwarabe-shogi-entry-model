use crate::{engine::Engine, log::LogExt, protocol::uxi::List40};
use casual_logger::Log;

impl List40 {
    /// 表示するだけ☆（＾～＾）
    pub fn list40(engine: &Engine) {
        Log::print_notice("----駒リスト40表示 ここから----");
        engine
            .position
            .table
            .for_all_pieces_on_table(&mut |i, adr, piece| {
                Log::print_notice(&format!(
                    "[{}]{}{}",
                    i,
                    if let Some(adr_val) = adr {
                        format!(" {:?}", adr_val)
                    } else {
                        " --".to_string()
                    },
                    if let Some(piece_val) = piece {
                        format!(" {} {:?}", piece_val.text1, piece_val.num)
                    } else {
                        " --".to_string()
                    }
                ));
            });
        Log::print_notice("----駒リスト40表示 ここまで----");
    }
}
