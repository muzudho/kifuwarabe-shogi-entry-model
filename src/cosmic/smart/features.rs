//!
//! 駒種類
//!
//! 先後なしの駒と空白
//!

use std::fmt;

// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const HAND_MAX: usize = 18;

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
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::PieceType::*;
        match *self {
            King => write!(f, "ら"),
            Rook => write!(f, "き"),
            Bishop => write!(f, "ぞ"),
            Gold => write!(f, "い"),
            Silver => write!(f, "ね"),
            Knight => write!(f, "う"),
            Lance => write!(f, "い"),
            Pawn => write!(f, "ひ"),
            Dragon => write!(f, "PK"),
            Horse => write!(f, "PZ"),
            PromotedSilver => write!(f, "PN"),
            PromotedKnight => write!(f, "PU"),
            PromotedLance => write!(f, "PS"),
            PromotedPawn => write!(f, "PH"),
        }
    }
}

// 利きボード☆（＾～＾）
#[derive(Clone, Copy)]
pub struct ControlBoard {}
impl Default for ControlBoard {
    fn default() -> Self {
        ControlBoard {}
    }
}
