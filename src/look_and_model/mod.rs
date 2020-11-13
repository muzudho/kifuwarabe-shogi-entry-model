pub mod absolute_address_2d;
pub mod control_board;
pub mod double_faced_piece;
pub mod double_faced_piece_type;
pub mod facility;
pub mod game_table;
pub mod piece;
pub mod piece_info;
pub mod piece_num;
pub mod piece_type;
pub mod position;
pub mod recording;
pub mod search;
pub mod searching;
pub mod title;

use crate::{
    computer_player::evaluator::Evaluation,
    cosmic::square::BOARD_MEMORY_AREA,
    law::generate_move::Area,
    look_and_model::{
        recording::{FireAddress, History},
        searching::MovegenPhase,
    },
};
use num_derive::FromPrimitive;
use std::time::{Duration, Instant};

/// このオブジェクトは大量に生成されるから、容量を小さく抑えた方がいいんだろうか☆（＾～＾）？
/// 1～9 は 4 byte.
/// 11～99 は 8 byte.
///
/// 絶対番地☆（＾～＾）相対番地と同じだが、回転の操作は座標 55 が中心になるぜ☆（＾～＾）
/// きふわらべでは 辞書象限 を採用している☆（＾～＾）
/// これは、file, rank は別々に持ち、しかも軸毎にプラス・マイナスを持つぜ☆（＾～＾）
///
/// Copy: 配列の要素の初期化時に使う☆（＾～＾）
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AbsoluteAddress2D {
    /// Square is shogi coordinate. file*10+rank.
    ///
    ///           North
    ///   91 81 71 61 51 41 31 21 11
    ///   92 82 72 62 52 42 32 22 12
    /// W 93 83 73 63 53 43 33 23 13 E
    /// E 94 84 74 64 54 44 34 24 14 A
    /// S 95 85 75 65 55 45 35 25 15 S
    /// T 96 86 76 66 56 46 36 26 16 T
    ///   97 87 77 67 57 47 37 27 17
    ///   98 88 78 68 58 48 38 28 18
    ///   99 89 79 69 59 49 39 29 19
    ///           Source
    serial: u8,
}

// 利きボード☆（＾～＾）
#[derive(Clone, Copy)]
pub struct ControlBoard {}

// Note: 持ち駒には玉も含むぜ☆（＾～＾）
pub const DOUBLE_FACED_PIECES_LEN: usize = 16;

#[derive(Clone, Copy, Debug)]
/// 表面と裏面の組み合わせで１つとしたときの種類。先後区別。玉を除けば、持ち駒の種類。
pub enum DoubleFacedPiece {
    // ▲ 玉と印字無し
    King1,
    // ▲ 飛と竜
    Rook1,
    // ▲ 角と馬
    Bishop1,
    // ▲ 金と印字無し
    Gold1,
    // ▲ 銀と全
    Silver1,
    // ▲ 桂と圭
    Knight1,
    // ▲ 香と杏
    Lance1,
    // ▲ 歩とと
    Pawn1,
    // △ 玉と印字無し
    King2,
    // △ 飛と竜
    Rook2,
    // △ 角と馬
    Bishop2,
    // △ 金と印字無し
    Gold2,
    // △ 銀と全
    Silver2,
    // △ 桂と圭
    Knight2,
    // △ 香と杏
    Lance2,
    // △ 歩とと
    Pawn2,
}

pub const DOUBLE_FACED_PIECE_TYPE_LEN: usize = 8;

#[derive(Clone, Copy, Debug, FromPrimitive)]
/// 物理的な駒の種類。玉を除けば、持ち駒の種類。
pub enum DoubleFacedPieceType {
    King,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
}

/// 卓☆（＾～＾）
/// でかいのでコピーもクローンも不可☆（＾～＾）！
/// 10の位を筋、1の位を段とする。
/// 0筋、0段は未使用
pub struct GameTable {
    /// 盤に、駒が紐づくぜ☆（＾～＾）
    board: [Option<PieceNum>; BOARD_MEMORY_AREA as usize],
    /// 持ち駒を次に置く番地。
    hand_next: [isize; DOUBLE_FACED_PIECES_LEN],
    /// 背番号付きの駒に、番地が紐づいているぜ☆（＾～＾）
    address_list: [FireAddress; PIECE_NUM_LEN],
    /// 駒の背番号に、駒が紐づくぜ☆（＾～＾）
    piece_list: [Piece; PIECE_NUM_LEN],
    /// 駒の背番号を付けるのに使うぜ☆（＾～＾）
    double_faced_piece_type_index: [usize; DOUBLE_FACED_PIECE_TYPE_LEN],
    /// 指し手生成に利用☆（＾～＾）
    pub area: Area,
}

/// 行き先表示案内板だぜ☆（＾～＾）
/// 読み筋とか表示されてるぜ☆（＾～＾）
pub struct InfoDisplay {
    /// 情報表示。現在の経過時間。
    info_stopwatch: Instant,
    /// 情報表示。前回表示したタイム。
    info_previous: Duration,
    /// 情報表示。初回は必ず表示。
    info_first: bool,
}

pub const PIECE_LEN: usize = 28;
pub static PIECE_WHITE_SPACE: &str = "    ";

/// toy_boxの中にカプセル化するぜ☆（＾～＾）
/// 先後付きの駒と空白。
/// 接尾辞の 1 は先手、 2 は後手。
///
/// Copy: 配列の要素の初期化のために利用。
#[derive(Copy, Clone, PartialEq, FromPrimitive)]
pub enum Piece {
    // ▲玉
    King1,
    // ▲きりん
    Rook1,
    // ▲ぞう
    Bishop1,
    // ▲いぬ
    Gold1,
    // ▲ねこ
    Silver1,
    // ▲うさぎ
    Knight1,
    // ▲いのしし
    Lance1,
    // ▲ひよこ
    Pawn1,
    // ▲ぱわーあっぷきりん
    Dragon1,
    // ▲ぱわーあっぷぞう
    Horse1,
    // ▲ぱわーあっぷねこ
    PromotedSilver1,
    // ▲ぱわーあっぷうさぎ
    PromotedKnight1,
    // ▲ぱわーあっぷいのしし
    PromotedLance1,
    // ▲ぱわーあっぷひよこ
    PromotedPawn1,
    // ▽ライオン
    King2,
    // ▽キリン
    Rook2,
    // ▽ゾウ
    Bishop2,
    // ▽イヌ
    Gold2,
    // ▽ネコ
    Silver2,
    // ▽ウサギ
    Knight2,
    // ▽イノシシ
    Lance2,
    // ▽ヒヨコ
    Pawn2,
    // ▽パワーアップキリン
    Dragon2,
    // ▽パワーアップゾウ
    Horse2,
    // ▽パワーアップネコ
    PromotedSilver2,
    // ▽パワーアップウサギ
    PromotedKnight2,
    // ▽パワーアップイノシシ
    PromotedLance2,
    // ▽パワーアップヒヨコ
    PromotedPawn2,
}

/// ちゆり「駒そのものではなく、駒の情報が欲しいだけなら、これだぜ☆」
/// きふわらべ「USIでは使わないから、独自の表記でOKだぜ☆」
/// 夢美「new()で引数２つ設定する必要があるけど、必ずしもそれを利用する必要はないのね」
pub struct PieceInfo {
    pub text1: String,
    pub num: String,
}

/// 背番号(名前)付きの駒の数。
pub const PIECE_NUM_LEN: usize = 40;

/// 駒の背番号（名前）だぜ☆（＾～＾）大橋流で触る駒の順だぜ☆（＾～＾）
#[derive(Clone, Copy, FromPrimitive, Debug, PartialEq)]
pub enum PieceNum {
    /// 1 先手玉
    King1,
    /// 2 後手玉
    King2,
    /// 3 金
    Gold3,
    /// 4 金
    Gold4,
    /// 5 金
    Gold5,
    /// 6 金
    Gold6,
    /// 7 銀
    Silver7,
    /// 8 銀
    Silver8,
    /// 9 銀
    Silver9,
    /// 10 銀
    Silver10,
    /// 11 桂
    Knight11,
    /// 12 桂
    Knight12,
    /// 13 桂
    Knight13,
    /// 14 桂
    Knight14,
    /// 15 香
    Lance15,
    /// 16 香
    Lance16,
    /// 17 香
    Lance17,
    /// 18 香
    Lance18,
    /// 19 角
    Bishop19,
    /// 20 角
    Bishop20,
    /// 21 飛
    Rook21,
    /// 22 飛
    Rook22,
    /// 23 歩
    Pawn23,
    /// 24 歩
    Pawn24,
    /// 25 歩
    Pawn25,
    /// 26 歩
    Pawn26,
    /// 27 歩
    Pawn27,
    /// 28 歩
    Pawn28,
    /// 29 歩
    Pawn29,
    /// 30 歩
    Pawn30,
    /// 31 歩
    Pawn31,
    /// 32 歩
    Pawn32,
    /// 33 歩
    Pawn33,
    /// 34 歩
    Pawn34,
    /// 35 歩
    Pawn35,
    /// 36 歩
    Pawn36,
    /// 37 歩
    Pawn37,
    /// 38 歩
    Pawn38,
    /// 39 歩
    Pawn39,
    /// 40 歩
    Pawn40,
}

pub const PIECE_TYPE_LEN: usize = 14;

/// USIでCopyするので、Copyが要る。
#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    // 玉
    King,
    // 飛
    Rook,
    // 角
    Bishop,
    // 金
    Gold,
    // 銀
    Silver,
    // 桂
    Knight,
    // 香
    Lance,
    // 歩
    Pawn,
    // 竜
    Dragon,
    // 馬
    Horse,
    // 全
    PromotedSilver,
    // 圭
    PromotedKnight,
    // 杏
    PromotedLance,
    // ぱわーあっぷひよこ
    PromotedPawn,
}

/// Position. A record of the game used to suspend or resume it.  
/// 局面。 ゲームを中断したり、再開したりするときに使うゲームの記録です。  
pub struct Position {
    /// 棋譜
    pub history: History,
    /// 初期の卓。これは SFEN を持てばよくて、オブジェクトは持たなくていいんじゃないか☆（＾～＾）？
    pub starting_table: GameTable,
    /// 現在の卓
    pub table: GameTable,
    pub movegen_phase: MovegenPhase,

    // Principal variation(読み筋)☆（＾～＾）
    pv_text: String,
    pv_len: usize,
}

/// 局面
pub enum PosNums {
    // 現局面
    Current,
    // 初期局面
    Start,
}

/*
/// PV表示、または 文字列表示だぜ☆（＾～＾）
pub enum PvString {
    /// 思考を開始してからのミリ秒と、読み筋。
    PV(u128, String),
    String(String),
}
*/

/// ツリーは探索中に１つしか生成しないぜ☆（＾～＾）
pub struct Search {
    /// Number of state nodes searched.  
    /// 探索した状態ノード数。  
    pub nodes: u64,
    /// Start the stopwatch when this structure is created.  
    /// この構造体を生成した時点からストップ・ウォッチを開始します。  
    pub stopwatch: Instant,

    // 思考時間（ミリ秒）をランダムにすることで、指し手を変えるぜ☆（＾～＾）
    pub think_msec: u128,

    pub evaluation: Evaluation,

    // 反復深化探索の１回目だけ真☆（＾～＾）
    pub depth_not_to_give_up: usize,
    // 読みの深さの上限☆（＾～＾）１手を読み切るなら 0 を指定しろだぜ☆（＾～＾）
    pub max_depth0: usize,
    /// 情報表示担当
    pub info: InfoDisplay,
}

/// タイトル画面。
pub struct Title {}
