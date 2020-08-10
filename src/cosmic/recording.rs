//!
//! * History (棋譜)
//! * Movement (指し手)
//! * Phase (先後。手番,相手番)
//! * Person (先手,後手)
//!
use crate::look_and_model::{
    recording::{Movement, Phase},
    AbsoluteAddress2D, DoubleFacedPiece,
};

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
impl Default for History {
    fn default() -> History {
        History {
            length_from_the_middle: 0,
            length_from_the_beginning: 0,
            movements: [Movement::default(); PLY_SIZE],
            position_hashs: [0; PLY_SIZE],
            starting_position_hash: 0,
            starting_turn: Phase::First,
        }
    }
}
impl History {
    /// 途中図から何手目か☆（＾～＾）１開始ではなく、０開始☆（＾～＾）
    pub fn length_from_the_middle(&self) -> isize {
        self.length_from_the_middle
    }
    /// 途中図から何手目か☆（＾～＾）１開始ではなく、０開始☆（＾～＾）
    pub fn total_length_from_the_beginning(&self) -> isize {
        self.length_from_the_beginning + self.length_from_the_middle
    }
    /// 手数を足す☆（＾～＾）
    pub fn add_moves(&mut self, offset: isize) {
        self.length_from_the_middle += offset;
    }
    /// 手番
    pub fn get_turn(&self) -> Phase {
        match self.starting_turn {
            Phase::First => {
                if self.length_from_the_middle % 2 == 0 {
                    Phase::First
                } else {
                    Phase::Second
                }
            }
            Phase::Second => {
                if self.length_from_the_middle % 2 == 0 {
                    Phase::Second
                } else {
                    Phase::First
                }
            }
        }
    }
    /// 現在の指し手
    pub fn get_move(&self) -> &Movement {
        &self.movements[self.length_from_the_middle as usize]
    }
    /*
    /// 局面ハッシュを更新
    pub fn set_position_hash(&mut self, hash: u64) {
        self.position_hashs[self.ply as usize] = hash;
    }
    */
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
impl HandAddress {
    pub fn new(piece: DoubleFacedPiece, sq: AbsoluteAddress2D) -> Self {
        HandAddress {
            piece: piece,
            sq: sq,
        }
    }
}
