//! プレイ中の対局があるときに変更してはいけないデータ。

pub mod game_hash_seed;

use crate::cosmic::recording::PHASE_LEN;
use crate::cosmic::smart::{features::HAND_MAX, square::BOARD_MEMORY_AREA};
use crate::look_and_model::{piece::PIECE_LEN, DOUBLE_FACED_PIECES_LEN};
use crate::Config;

/// USI対応コンピューター将棋ソフトの名前☆（＾～＾）
pub const ENGINE_NAME: &str = "KifuwarabeEM bld49";

/// 作者の名前。姓・名の順にしたいぜ☆（＾～＾）異文化に通じる表記方法はないものか☆（＾～＾）
pub const ENGINE_AUTHOR: &str = "TAKAHASHI, Satoshi";

/// ログ
pub const LOG_FILE: &str = "kifuwarabeEM";

pub const PV_BUFFER: usize = 2048;

/// info 表示の間隔（ミリ秒）
pub const INFO_INTERVAL_MSEC: u128 = 1000;

impl Default for Config {
    fn default() -> Self {
        Config {
            hash_seed: GameHashSeed::default(),
        }
    }
}
/// 現対局ハッシュ種
/// ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
pub struct GameHashSeed {
    // 盤上の駒
    pub piece: [[u64; PIECE_LEN]; BOARD_MEMORY_AREA as usize],
    // 持ち駒
    pub hands: [[u64; HAND_MAX]; DOUBLE_FACED_PIECES_LEN],
    // 先後
    pub phase: [u64; PHASE_LEN],
}
