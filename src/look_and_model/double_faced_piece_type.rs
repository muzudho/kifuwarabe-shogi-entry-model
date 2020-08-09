use crate::look_and_model::DoubleFacedPieceType;
use std::fmt;

/// USIの Drop に合わせるぜ☆（＾～＾） 先後を区別しないぜ☆（＾～＾）
impl fmt::Display for DoubleFacedPieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::DoubleFacedPieceType::*;
        match *self {
            King => write!(f, "?*"),
            Rook => write!(f, "R*"),
            Bishop => write!(f, "B*"),
            Gold => write!(f, "G*"),
            Silver => write!(f, "S*"),
            Knight => write!(f, "N*"),
            Lance => write!(f, "L*"),
            Pawn => write!(f, "P*"),
        }
    }
}
impl DoubleFacedPieceType {
    /// 最大要素数。
    pub fn hand_max_elements(&self) -> usize {
        match self {
            DoubleFacedPieceType::King => 2,
            DoubleFacedPieceType::Rook => 2,
            DoubleFacedPieceType::Bishop => 2,
            DoubleFacedPieceType::Gold => 4,
            DoubleFacedPieceType::Silver => 4,
            DoubleFacedPieceType::Knight => 4,
            DoubleFacedPieceType::Lance => 4,
            DoubleFacedPieceType::Pawn => 18,
        }
    }
}
