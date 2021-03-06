//!
//! きふわらべＷＣＳＣ３０
//!
//! これは、最初に実行されるファイルだぜ☆（＾～＾）
//!

// extern crate は、 main.rs か lib.rs の冒頭にまとめろだぜ☆（＾～＾）
extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate atoi;
extern crate num_derive;
extern crate num_traits;
extern crate regex;

// Rust言語の mod や ソース置き場の説明
//     「Rust のモジュールシステム」
//      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
//
// 使いたい ディレクトリー名を pub mod しろだぜ☆（＾～＾）
// 別のアプリにも見えるようにしたけりゃ pub mod にしろだぜ☆（＾～＾）
mod command_line_seek;
mod computer_player;
mod config;
mod cosmic;
mod engine;
mod law;
mod log;
mod look_and_model;
mod performance_measurement;
mod protocol;
mod test;

use crate::{config::GameHashSeed, engine::Response, log::LogExt, look_and_model::Position};
use casual_logger::{Log, Opt};
use test::test;

/// USI対応コンピューター将棋ソフトの名前☆（＾～＾）
pub const ENGINE_NAME: &str = "KifuwarabeEM bld69";

/// 作者の名前。姓・名の順にしたいぜ☆（＾～＾）異文化に通じる表記方法はないものか☆（＾～＾）
pub const ENGINE_AUTHOR: &str = "TAKAHASHI, Satoshi";

/// ログ
pub const LOG_FILE: &str = "kifuwarabeEM";

pub const PV_BUFFER: usize = 2048;

/// info 表示の間隔（ミリ秒）
/// 1 秒では短いんだろうか☆（＾～＾） 3 にしてみるか……☆（＾～＾）
pub const INFO_INTERVAL_MSEC: u128 = 3000;

fn main() {
    Log::set_file_name(LOG_FILE);
    Log::set_opt(Opt::Release);
    // Log::set_level(Level::Notice);
    Log::remove_old_logs();
    // 宇宙☆（＾～＾）変化するぜ☆（＾～＾）
    let mut engine = Engine::default();

    // ビッグバン
    engine.big_bang();

    test();

    // End the loop with 'quit'. Forced termination with [Ctrl]+[C].
    // 'quit' でループを終了。 [Ctrl]+[C] で強制終了。
    loop {
        let mut line: String = String::new();
        // Wait for command line input from standard input.
        // 標準入力からのコマンドライン入力を待機します。
        match std::io::stdin().read_line(&mut line) {
            Ok(_n) => {}
            // Tips. You can separate error numbers by simply specifying the line number.
            // テクニック。 エラー番号は行番号を振っておくだけで少しはばらけます。
            Err(e) => panic!(Log::print_fatal(&format!(
                "(Err.373) Failed to read line. / {}",
                e
            ))),
        };

        if let Some(response) = engine.enter(&line) {
            match response {
                Response::Quit => {
                    break;
                }
            }
        }
    }

    Log::flush();
}

/// プレイ中の対局があるときに変更してはいけないデータ。
pub struct Config {
    /// ハッシュ種☆（＾～＾）ゲームの途中でクリアしてはいけないぜ☆（＾～＾）
    pub hash_seed: GameHashSeed,
}

/// アプリケーション開始時に決め終えておくものだぜ☆（＾～＾）
pub struct Engine {
    /// 対局
    pub position: Position,
    /// 設定。プレイ中の対局があるときに変更してはいけないデータ。
    pub config: Config,
    /// 大会ルールの最大手数☆（＾～＾）
    pub option_max_ply: usize,
    /// 読みの最大深さ。
    pub option_max_depth: usize,
    /// 思考時間の最小秒☆（＾～＾）
    pub option_min_think_msec: u64,
    /// 思考時間の最大秒☆（＾～＾）
    pub option_max_think_msec: u64,
    /// 諦めない深さ☆（＾～＾）読み終わるまで、思考時間を無視するぜ☆（＾～＾）max_depth - 1 より小さくしろだぜ☆（＾～＾）
    pub option_depth_not_to_give_up: usize,
}
