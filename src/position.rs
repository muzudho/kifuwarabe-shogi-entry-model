//! GameTable. A record of the game used to suspend or resume it.  
//! 局面。 ゲームを中断したり、再開したりするときに使うゲームの記録です。  
use crate::config::PV_BUFFER;
use crate::cosmic::playing::{MovegenPhase, PosNums};
use crate::cosmic::pos_hash::pos_hash::*;
use crate::cosmic::recording::{FireAddress, HandAddress, History, Movement, Phase};
use crate::log::LogExt;
use crate::look_and_model::game_table::GameTable;
use casual_logger::Log;

/// Position. A record of the game used to suspend or resume it.  
/// 局面。 ゲームを中断したり、再開したりするときに使うゲームの記録です。  
pub struct Position {
    /// 棋譜
    pub history: History,
    /// 初期の卓
    pub starting_table: GameTable,
    /// TODO 開始局面で次に指す方。棋譜の方も要対応。
    pub starting_turn: Phase,
    /// 現対局ハッシュ種☆（＾～＾）
    pub hash_seed: GameHashSeed,
    /// 現在の卓
    pub table: GameTable,
    pub movegen_phase: MovegenPhase,

    // Principal variation(読み筋)☆（＾～＾）
    pv_text: String,
    pv_len: usize,
}
impl Default for Position {
    fn default() -> Position {
        Position {
            history: History::default(),
            starting_table: GameTable::default(),
            starting_turn: Phase::First,
            hash_seed: GameHashSeed::default(),
            table: GameTable::default(),
            movegen_phase: MovegenPhase::default(),
            pv_text: String::with_capacity(PV_BUFFER),
            pv_len: 0,
        }
    }
}
impl Position {
    /// 初期局面、現局面ともにクリアーします。
    /// 手目も 0 に戻します。
    pub fn clear(&mut self) {
        self.starting_table.clear();
        self.starting_turn = Phase::First;
        self.table.clear();
        self.history.ply = 0;
        self.pv_text = String::with_capacity(PV_BUFFER);
        self.pv_len = 0;
    }
    pub fn pv_text(&self) -> &str {
        &self.pv_text
    }
    pub fn pv_len(&self) -> usize {
        self.pv_len
    }
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット
        self.hash_seed.big_bang();
    }

    /// 棋譜の作成
    pub fn set_move(&mut self, move_: &Movement) {
        self.history.movements[self.history.ply as usize] = *move_; // クローンが入る☆（＾～＾）？
    }
    /// テスト用に棋譜表示☆（＾～＾）
    pub fn get_moves_history_text(&self) -> String {
        let mut s = String::new();
        for ply in 0..self.history.ply {
            let movement = &self.history.movements[ply as usize];
            s.push_str(&format!("[{}] {}", ply, movement));
        }
        s
    }

    pub fn get_table(&self, num: PosNums) -> &GameTable {
        match num {
            PosNums::Current => &self.table,
            PosNums::Start => &self.starting_table,
        }
    }
    pub fn mut_starting(&mut self) -> &mut GameTable {
        &mut self.starting_table
    }

    /// テスト用に局面ハッシュ☆（＾～＾）
    pub fn get_positions_hash_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "[ini] {:20}\n",
            &self.history.starting_position_hash
        ));

        for ply in 0..self.history.ply {
            let hash = &self.history.position_hashs[ply as usize];
            // 64bitは10進数20桁。改行する
            s.push_str(&format!("[{:3}] {:20}\n", ply, hash));
        }
        s
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    /// TODO 初期局面を何に使ってるのか☆（＾～＾）？
    pub fn count_same_position(&self) -> isize {
        if self.history.ply < 1 {
            return 0;
        }

        let mut count = 0;
        let last_ply = self.history.ply - 1;
        let new_ply = self.history.ply;
        for i_ply in 0..new_ply {
            let t = last_ply - i_ply;
            if self.history.position_hashs[t as usize]
                == self.history.position_hashs[last_ply as usize]
            {
                count += 1;
            }
        }

        // 初期局面のハッシュ
        if self.history.starting_position_hash == self.history.position_hashs[last_ply as usize] {
            count += 1;
        }

        count
    }

    /// Place the stone.  
    /// １手指します。  
    pub fn do_move(&mut self, turn: Phase, move_: &Movement) {
        // Principal variation.
        if self.pv_text.is_empty() {
            self.pv_text.push_str(&move_.to_string());
        } else {
            self.pv_text.push_str(&format!(" {}", move_));
        }
        self.pv_len += 1;

        self.set_move(&move_);
        self.redo_move(turn, move_);
    }

    /// Place the stone.  
    /// Do not write to the pv.  
    /// １手指します。  
    /// 読み筋への書き込みを行いません。  
    pub fn redo_move(&mut self, turn: Phase, move_: &Movement) {
        // 局面ハッシュを作り直す
        self.hash_seed
            .update_by_do_move(&mut self.history, &self.table, move_);

        // 移動元のマスにある駒をポップすることは確定。
        let src_piece_num = self.table.pop_piece(turn, &move_.source);

        // 持ち駒は成ることは無いので、成るなら盤上の駒であることが確定。
        if move_.promote {
            // 成ったのなら、元のマスの駒を成らすぜ☆（＾～＾）
            if let Some(piece_num) = src_piece_num {
                self.table.promote(piece_num);
            } else {
                panic!(Log::print_fatal(
                    "(Err.248) 成ったのに、元の升に駒がなかった☆（＾～＾）"
                ));
            }
        }
        // 移動先升に駒があるかどうか
        // あれば　盤の相手の駒を先後反転して、自分の駒台に置きます。
        self.table.rotate_piece_board_to_hand_on_do(turn, &move_);

        // 移動先升に駒を置く
        self.table.push_piece(&move_.destination, src_piece_num);

        // // 局面ハッシュを作り直す
        // let ky_hash = self.hash_seed.current_position(&self);
        // self.history.set_position_hash(ky_hash);

        self.history.ply += 1;
    }

    /// 逆順に指します。
    pub fn undo_move(&mut self) -> bool {
        // Principal variation.
        // None か スペースが出てくるまで削除しようぜ☆（＾～＾）
        loop {
            if let Some(ch) = self.pv_text.pop() {
                if ch == ' ' {
                    break;
                }
            } else {
                break;
            }
        }

        if 0 < self.pv_len {
            self.pv_len -= 1;
        }
        if 0 < self.history.ply {
            // 棋譜から読取、手目も減る
            self.history.ply -= 1;
            let move_ = &self.history.get_move();
            // 移動先にある駒をポップするのは確定。
            let moveing_piece_num = self
                .table
                .pop_piece(self.history.get_turn(), &move_.destination);
            match move_.source {
                FireAddress::Board(_src_sq) => {
                    // 盤上の移動なら
                    if move_.promote {
                        // 成ったなら、成る前へ
                        if let Some(source_piece_num) = moveing_piece_num {
                            self.table.demote(source_piece_num);
                        } else {
                            panic!(Log::print_fatal(
                                "(Err.305) 成ったのに移動先に駒が無いぜ☆（＾～＾）！"
                            ))
                        }
                    }

                    // 打でなければ、移動元升に、動かした駒を置く☆（＾～＾）打なら何もしないぜ☆（＾～＾）
                    self.table.push_piece(&move_.source, moveing_piece_num);
                }
                FireAddress::Hand(src_drop) => {
                    // 打なら
                    // 打った場所に駒があるはずだぜ☆（＾～＾）
                    let piece_num = if let Some(piece_num) = moveing_piece_num {
                        piece_num
                    } else {
                        panic!(Log::print_fatal("(Err.250) Invalid moveing_piece_num"));
                    };
                    // 置いた駒を、駒台に戻すだけだぜ☆（＾～＾）
                    // TODO この駒を置くことになる場所は☆（＾～＾）？
                    self.table.push_piece(
                        &FireAddress::Hand(HandAddress::new(
                            self.table.get_double_faced_piece(piece_num),
                            src_drop.sq,
                        )),
                        moveing_piece_num,
                    );
                }
            }

            // 取った駒が有ったか。
            // あれば、指し手で取った駒の先後をひっくり返せば、自分の駒台にある駒を取り出せるので取り出して、盤の上に指し手の取った駒のまま駒を置きます。
            self.table
                .rotate_piece_hand_to_board_on_undo(self.history.get_turn(), &move_);

            // TODO 局面ハッシュを作り直したいぜ☆（＾～＾）

            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }
}
