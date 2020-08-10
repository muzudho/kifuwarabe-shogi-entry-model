pub mod captured_move;
pub mod fire_address;
pub mod movement;
pub mod phase;

use crate::{
    cosmic::recording::*,
    look_and_model::{AbsoluteAddress2D, PieceNum},
};

/// 取ることになる駒の移動。
#[derive(Clone, Copy)]
pub struct CapturedMove {
    /// 元あった所。
    pub source: FireAddress,
    /// 移動先。
    pub destination: FireAddress,
}

/// 盤上と、駒台で　共通しないものを並列にします。
#[derive(Copy, Clone, Debug)]
pub enum FireAddress {
    Board(AbsoluteAddress2D),
    Hand(HandAddress),
}

/// 駒の背番号も欲しいか☆（＾～＾）？
/// 棋譜にも使うので、取った駒の情報を記憶しておくんだぜ☆（＾～＾）
/// 投了なら これを使わず、None にしろだぜ☆（＾～＾）
///
/// 移動していないことを表すには、移動元と移動先を同じにすればいいんだぜ☆（＾～＾）
///
/// Copy: 配列の要素の初期化時に使う☆（＾～＾）
#[derive(Clone, Copy)]
pub struct Movement {
    /// 動かす駒の背番号
    pub piece_num: PieceNum,
    /// 移動元マス。
    pub source: FireAddress,
    /// 移動先マス。リバーシブルに作りたいので、駒台にも指せる。
    pub destination: FireAddress,
    /// 移動後に成るなら真
    pub promote: bool,
    /// 取ることになる駒。先後がひっくり返るのは駒を取られた時だけだぜ☆（＾～＾）
    pub captured: Option<CapturedMove>,
}

/// 局面ハッシュを作るときに、フェーズ用に配列があって、それのサイズに使ってるぜ☆（＾～＾）
// pub const PHASE_FIRST: usize = 0;
pub const PHASE_SECOND: usize = 1;
pub const PHASE_LEN: usize = 2;

/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    First,
    Second,
}
