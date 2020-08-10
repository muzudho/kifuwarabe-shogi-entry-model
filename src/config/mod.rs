//! プレイ中の対局があるときに変更してはいけないデータ。

pub mod game_hash_seed;

use crate::{
    cosmic::square::BOARD_MEMORY_AREA,
    look_and_model::{recording::PHASE_LEN, DOUBLE_FACED_PIECES_LEN, PIECE_LEN},
    Config,
};

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

// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const HAND_MAX: usize = 18;
