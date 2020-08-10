pub mod double_faced_piece;
pub mod double_faced_piece_type;
pub mod facility;
pub mod game_table;
pub mod piece;
pub mod position;
pub mod search;
pub mod title;

use crate::computer_player::evaluator::Evaluation;
use crate::cosmic::{
    recording::FireAddress,
    smart::square::BOARD_MEMORY_AREA,
    toy_box::{PieceNum, NAMED_PIECES_LEN},
};
use crate::law::generate_move::Area;
use crate::look_and_model::piece::Piece;
use num_derive::FromPrimitive;
use std::time::{Duration, Instant};

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
    address_list: [FireAddress; NAMED_PIECES_LEN],
    /// 駒の背番号に、駒が紐づくぜ☆（＾～＾）
    piece_list: [Piece; NAMED_PIECES_LEN],
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

/// PV表示、または 文字列表示だぜ☆（＾～＾）
pub enum PvString {
    /// 思考を開始してからのミリ秒と、読み筋。
    PV(u128, String),
    String(String),
}

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
