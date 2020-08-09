use crate::look_and_model::DoubleFacedPiece;

impl DoubleFacedPiece {
    /// 開始地点。
    pub fn hand_start(&self) -> isize {
        match self {
            DoubleFacedPiece::King1 => 2,
            DoubleFacedPiece::Rook1 => 103,
            DoubleFacedPiece::Bishop1 => 101,
            DoubleFacedPiece::Gold1 => 6,
            DoubleFacedPiece::Silver1 => 10,
            DoubleFacedPiece::Knight1 => 50,
            DoubleFacedPiece::Lance1 => 90,
            DoubleFacedPiece::Pawn1 => 121,
            DoubleFacedPiece::King2 => 1,
            DoubleFacedPiece::Rook2 => 102,
            DoubleFacedPiece::Bishop2 => 100,
            DoubleFacedPiece::Gold2 => 3,
            DoubleFacedPiece::Silver2 => 7,
            DoubleFacedPiece::Knight2 => 20,
            DoubleFacedPiece::Lance2 => 60,
            DoubleFacedPiece::Pawn2 => 104,
        }
    }
    /// 向き。
    pub fn hand_direction(&self) -> isize {
        match self {
            DoubleFacedPiece::King1 => -1,
            DoubleFacedPiece::Rook1 => -1,
            DoubleFacedPiece::Bishop1 => -1,
            DoubleFacedPiece::Gold1 => -1,
            DoubleFacedPiece::Silver1 => -1,
            DoubleFacedPiece::Knight1 => -10,
            DoubleFacedPiece::Lance1 => -10,
            DoubleFacedPiece::Pawn1 => -1,
            DoubleFacedPiece::King2 => 1,
            DoubleFacedPiece::Rook2 => 1,
            DoubleFacedPiece::Bishop2 => 1,
            DoubleFacedPiece::Gold2 => 1,
            DoubleFacedPiece::Silver2 => 1,
            DoubleFacedPiece::Knight2 => 10,
            DoubleFacedPiece::Lance2 => 10,
            DoubleFacedPiece::Pawn2 => 1,
        }
    }
}
