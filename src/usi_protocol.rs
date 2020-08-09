//! Converts a position into a string or restores a string into a position.  
//! 局面を文字列に変換したり、文字列を局面に復元します。  
// use crate::look_and_model::{GameResult, Piece};

/*
use crate::look_and_model::game_table::GameTable;
use crate::position::Position;

/// A record of the game used to suspend or resume it.
/// ゲームを中断したり、再開したりするときに使うゲームの記録です。
impl Position {
    /// Converts the current position to sfen.
    /// 現局面を sfen に変換します。
    pub fn to_sfen(&self) -> String {
        let mut sfen = String::default();
        sfen.push_str("sfen ");

        // Starting board.
        // 開始盤面。
        sfen.push_str(self.starting_table.to_sfen());

        // Next stone at the start.
        // 開始局面で、次に置く石。
        match self.starting_turn {
            Piece::Nought => {
                sfen.push_str(" o");
            }
            Piece::Cross => {
                sfen.push_str(" x");
            }
        }

        // A game record.
        // 棋譜。
        if 0 < self.pieces_num - self.starting_pieces_num {
            sfen.push_str(" moves");
            for i in self.starting_pieces_num..self.pieces_num {
                sfen.push_str(&format!(" {}", self.history[i].to_string()));
            }
        }

        sfen.to_string()
    }
}

impl GameTable {
    pub fn to_sfen(&self) -> String {
        let mut sfen = String::default();
        sfen.push_str("sfen ");
        let mut spaces = 0;
        for sq in [7, 8, 9, 4, 5, 6, 1, 2, 3].iter() {
            if let Some(piece) = self.board[*sq as usize] {
                if 0 < spaces {
                    sfen.push_str(&spaces.to_string());
                    spaces = 0;
                }
                sfen.push(match piece {
                    Piece::Nought => 'o',
                    Piece::Cross => 'x',
                });
            } else {
                spaces += 1;
            }

            if *sq == 9 || *sq == 6 {
                if 0 < spaces {
                    sfen.push_str(&spaces.to_string());
                    spaces = 0;
                }
                sfen.push('/');
            }
        }

        // Flush the remaining space.
        // 残っているスペースを flush します。
        if 0 < spaces {
            sfen.push_str(&spaces.to_string());
        }

        sfen.to_string()
    }
}
*/
