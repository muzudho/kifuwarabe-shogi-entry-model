use crate::{
    law::cryptographic::num_to_lower_case, look_and_model::recording::FireAddress,
    look_and_model::AbsoluteAddress2D,
};
use std::fmt;

/// USI向け。
impl fmt::Display for FireAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FireAddress::Board(sq) => {
                    let (file, rank) = sq.to_file_rank();
                    format!("{}{}", file, num_to_lower_case(rank as usize))
                }
                FireAddress::Hand(drop) => {
                    format!("{}", drop.piece.type_())
                }
            },
        )
    }
}
impl Default for FireAddress {
    /// ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        FireAddress::Board(AbsoluteAddress2D::default())
    }
}
