/// 指令室はこちらだぜ☆（＾～＾）！
use crate::{log::LogExt, look_and_model::Title};
use casual_logger::Log;

impl Title {
    // 対話モードのタイトル画面
    pub fn pretty() {
        // 横幅は 半角79文字使えるぜ☆（＾～＾）
        // 80文字目を使うと、次の行が改行で空行になってしまう☆（＾～＾）
        Log::print_notice(
            &"\
 Kifuwarabe's Shogi Entry model
 きふわらべの将棋 入門モデル
\
"
            .to_string(),
        );
    }
}
