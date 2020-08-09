//! Converts a position into a string or restores a string into a position.  
//! 局面を文字列に変換したり、文字列を局面に復元します。  
// use crate::look_and_model::{GameResult, Piece};

use crate::cosmic::recording::Phase;
use crate::cosmic::smart::square::{AbsoluteAddress2D, FILE10U8, FILE1U8, RANK10U8, RANK1U8};
use crate::look_and_model::game_table::GameTable;
use crate::position::Position;

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
        let mut sfen = String::default();

        // Starting board.
        // 途中盤面。
        sfen.push_str(&self.starting_table.to_sfen());

        // Next stone at the start.
        // 途中盤面で、次に置く石。 先手は b、後手は w と決められています。
        match self.history.starting_turn {
            Phase::First => match self.history.get_turn() {
                Phase::First => sfen.push_str(" b"),
                Phase::Second => sfen.push_str(" w"),
            },
            Phase::Second => match self.history.get_turn() {
                Phase::First => sfen.push_str(" w"),
                Phase::Second => sfen.push_str(" b"),
            },
        }

        // 途中盤面の次は何手目か
        if sfen_enable {
            // sfen では 1固定。
            sfen.push_str(" 1");
        } else {
            sfen.push_str(&format!(
                " {}",
                // 手数なんで 1 を足す。
                self.history.total_length_from_the_beginning() + 1
            ));
        }
        /* TODO

                // A game record.
                // 棋譜。
                if 0 < self.pieces_num - self.starting_pieces_num {
                    sfen.push_str(" moves");
                    for i in self.starting_pieces_num..self.pieces_num {
                        sfen.push_str(&format!(" {}", self.history[i].to_string()));
                    }
                }
        */

        sfen.to_string()
    }
}

impl GameTable {
    pub fn to_sfen(&self) -> String {
        let mut sfen = String::default();
        sfen.push_str("sfen ");
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
