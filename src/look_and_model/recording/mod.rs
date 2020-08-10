pub mod captured_move;
pub mod fire_address;
pub mod hand_address;
pub mod history;
pub mod movement;
pub mod phase;

use crate::look_and_model::{AbsoluteAddress2D, DoubleFacedPiece, PieceNum};

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

/// 持ち駒の番地。
#[derive(Copy, Clone, Debug)]
pub struct HandAddress {
    /// USI出力に必要。 'R*' とか。 指し手生成で 歩、香、桂、その他の区別にも利用。
    /// 利用するとき 先手／後手 情報はよく使うんで、めんとくさいんで 先手／後手 情報も持たせておきます。
    pub piece: DoubleFacedPiece,
    /// TODO 未使用☆（＾～＾）？
    pub sq: AbsoluteAddress2D,
}

/// 手数☆（＾～＾） 大会ルールとは別で、このプログラムが対応できる上限値☆（＾～＾）
/// 主要大会では、一番大きくても　電竜戦の 512 だろ☆（＾～＾）
pub const PLY_SIZE: usize = 1024;

/// 同一局面何回で千日手
pub const SENNTITE_NUM: isize = 4;

pub struct History {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    length_from_the_middle: isize,
    /// 途中局面の次の一手は何手目か。
    pub length_from_the_beginning: isize,
    /// 棋譜
    /// TODO 0手目を初期局面にしたいので、最初にパスを入れてほしい☆（＾～＾）
    pub movements: [Movement; PLY_SIZE],
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hashs: [u64; PLY_SIZE],
    /// 初期局面ハッシュ
    pub starting_position_hash: u64,
    /// 開始局面で次に指す方。
    pub starting_turn: Phase,
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
