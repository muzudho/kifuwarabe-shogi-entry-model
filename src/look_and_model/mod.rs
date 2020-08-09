pub mod double_faced_piece;
pub mod double_faced_piece_type;
pub mod facility;
pub mod game_table;
pub mod piece;
pub mod position;
pub mod search;

use num_derive::FromPrimitive;

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
