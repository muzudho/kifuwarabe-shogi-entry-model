use crate::{
    law::cryptographic::num_to_lower_case,
    look_and_model::{
        recording::{CapturedMove, FireAddress, Movement},
        PieceNum,
    },
};
use std::fmt;

impl Default for Movement {
    /// ゴミの値を作るぜ☆（＾～＾）
    fn default() -> Self {
        Movement {
            piece_num: PieceNum::King1,
            source: FireAddress::default(),
            destination: FireAddress::default(),
            promote: false,
            captured: None,
        }
    }
}
impl Movement {
    pub fn new(
        piece_num: PieceNum,
        source: FireAddress,
        destination: FireAddress,
        promote: bool,
        captured: Option<CapturedMove>,
    ) -> Self {
        Movement {
            piece_num: piece_num,
            source: source,
            destination: destination,
            promote: promote,
            captured: captured,
        }
    }
}
impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.source {
            FireAddress::Board(src_sq) => {
                let (sx, sy) = src_sq.to_file_rank();
                write!(
                    f,
                    "{}{}{}{}",
                    sx,
                    num_to_lower_case(sy as usize),
                    self.destination,
                    if self.promote { "+" } else { "" }
                )
            }
            FireAddress::Hand(_src_drop) => write!(
                f,
                "{}{}{}",
                self.source, // src_drop,
                self.destination,
                if self.promote { "+" } else { "" }
            ),
        }
    }
}
impl fmt::Debug for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Movement({:?}{:?}{})",
            self.source, self.destination, self.promote,
        )
    }
}
