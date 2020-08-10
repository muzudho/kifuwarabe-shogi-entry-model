use crate::{log::LogExt, protocol::usi::Usi, ENGINE_AUTHOR, ENGINE_NAME};
use casual_logger::Log;

/// 船長：きふわらべ
///
/// 対局で許されている命令だけをするぜ☆（＾～＾）
impl Usi {
    pub fn usi() {
        Log::print_notice(&format!("id name {}", ENGINE_NAME));
        Log::print_notice(&format!("id author {}", ENGINE_AUTHOR));
        /*
        IO::writeln("option name BookFile type string default public.bin");
        IO::writeln("option name UseBook type check default true");
        IO::writeln("option name Selectivity type spin default 2 min 0 max 4");
        IO::writeln(
            "option name Style type combo default Normal var Solid var Normal var Risky",
        );
        IO::writeln("option name ResetLearning type button");
        IO::writeln("option name LearningFile type filename default <empty>");
        */
        // アルファベット順ではなく、将棋所のダイアログボックスが見やすくなるように並べろだぜ☆（＾～＾）
        // 大会ルール関連☆（＾～＾）
        Log::print_notice("option name MaxPly type spin default 320 min 1 max 10000");
        // 読みの深さ関連☆（＾～＾）
        Log::print_notice("option name DepthNotToGiveUp type spin default 4 min 1 max 8");
        Log::print_notice("option name MaxDepth type spin default 7 min 1 max 15");
        // 思考時間関連☆（＾～＾）
        Log::print_notice("option name MinThinkMsec type spin default 5000 min 0 max 599000");
        Log::print_notice("option name MaxThinkMsec type spin default 17000 min 1000 max 600000");
        Log::print_notice("usiok");
    }
}
