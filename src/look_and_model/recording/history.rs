//!
//! * History (棋譜)
//! * Movement (指し手)
//! * Phase (先後。手番,相手番)
//! * Person (先手,後手)
//!
use crate::look_and_model::recording::{History, Movement, Phase, PLY_SIZE};

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
