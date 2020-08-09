pub mod double_faced_piece_type;
pub mod facility;
pub mod game_table;
pub mod piece;
pub mod position;
pub mod search;

use num_derive::FromPrimitive;

pub const PHYSICAL_PIECE_TYPE_LEN: usize = 8;

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
