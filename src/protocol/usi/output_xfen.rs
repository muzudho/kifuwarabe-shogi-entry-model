//! Converts a position into a string or restores a string into a position.  
//! 局面を文字列に変換したり、文字列を局面に復元します。  
// use crate::look_and_model::{GameResult, Piece};

use crate::{
    cosmic::smart::square::{FILE10U8, FILE1U8, RANK10U8, RANK1U8},
    look_and_model::{recording::Phase, AbsoluteAddress2D, GameTable, Position},
};

/// A record of the game used to suspend or resume it.
/// ゲームを中断したり、再開したりするときに使うゲームの記録です。
impl Position {
    /// Converts the current position to sfen.
    /// 現局面を sfen に変換します。
    ///
    /// # Arguments
    ///
    /// * `sfen_enable` - 真にすると、将棋所の仕様に合わせます。
    pub fn to_xfen(&self, sfen_enable: bool) -> String {
        let mut xfen = String::default();
        if sfen_enable {
            xfen.push_str("sfen ");
        } else {
            xfen.push_str("xfen ");
        }

        // Starting board.
        // 現在図を、途中図として出力します。
        xfen.push_str(&self.table.to_sfen());

        // Next stone at the start.
        // 途中盤面で、次に置く石。 先手は b、後手は w と決められています。
        match self.history.starting_turn {
            Phase::First => match self.history.get_turn() {
                Phase::First => xfen.push_str(" b"),
                Phase::Second => xfen.push_str(" w"),
            },
            Phase::Second => match self.history.get_turn() {
                Phase::First => xfen.push_str(" w"),
                Phase::Second => xfen.push_str(" b"),
            },
        }

        // 途中盤面の次は何手目か
        if sfen_enable {
            // sfen では 1固定。
            xfen.push_str(" 1");
        } else {
            xfen.push_str(&format!(
                " {}",
                // 手数なんで 1 を足す。
                self.history.total_length_from_the_beginning() + 1
            ));
        }

        // A game record.
        // 棋譜。
        if 0 < self.history.length_from_the_middle() {
            xfen.push_str(" moves");
            for i in 0..self.history.length_from_the_middle() {
                xfen.push_str(&format!(" {}", self.history.movements[i as usize]));
            }
        }

        xfen.to_string()
    }
}

impl GameTable {
    pub fn to_sfen(&self) -> String {
        let mut sfen = String::default();
        let mut spaces = 0;
        for rank in RANK1U8..RANK10U8 {
            for file in (FILE1U8..FILE10U8).rev() {
                let sq = AbsoluteAddress2D::new(file, rank);
                if let Some(piece) = self.piece_num_on_board_at(&sq) {
                    if 0 < spaces {
                        sfen.push_str(&spaces.to_string());
                        spaces = 0;
                    }
                    sfen.push_str(&format!("{}", self.get_piece_string(piece)));
                } else {
                    spaces += 1;
                }
            }

            // Flush the remaining space.
            // 残っているスペースを flush します。
            if 0 < spaces {
                sfen.push_str(&spaces.to_string());
                spaces = 0;
            }
            sfen.push('/');
        }

        // 最後の '/' を省きます。
        sfen.pop();

        sfen.to_string()
    }
}
