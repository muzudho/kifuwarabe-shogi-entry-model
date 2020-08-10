//!
//! 駒 と 盤
//!
use crate::look_and_model::{PieceInfo, PieceNum};
use std::*;

impl PieceInfo {
    pub fn new(piece_display: &str, num: PieceNum) -> Self {
        PieceInfo {
            text1: piece_display.to_string(),
            num: format!("{:?}", num),
        }
    }
}
