//! 局面ハッシュ。
//!

use crate::{
    config::{GameHashSeed, HAND_MAX},
    cosmic::smart::square::{BOARD_MEMORY_AREA, FILE10U8, FILE1U8, RANK10U8, RANK1U8, SQUARE_NONE},
    law::speed_of_light::HandAddresses,
    log::LogExt,
    look_and_model::{
        recording::{FireAddress, History, Movement, Phase, PHASE_LEN, PHASE_SECOND},
        AbsoluteAddress2D, DoubleFacedPiece, GameTable, Position, DOUBLE_FACED_PIECES_LEN,
        PIECE_LEN,
    },
};
use casual_logger::Log;
use rand::Rng;

impl Default for GameHashSeed {
    fn default() -> Self {
        GameHashSeed {
            // 盤上の駒
            piece: [[0; PIECE_LEN]; BOARD_MEMORY_AREA as usize],
            // 持ち駒
            hands: [[0; HAND_MAX]; DOUBLE_FACED_PIECES_LEN],
            // 先後
            phase: [0; PHASE_LEN],
        }
    }
}
impl GameHashSeed {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_square in SQUARE_NONE..BOARD_MEMORY_AREA {
            for i_piece in 0..PIECE_LEN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.piece[i_square as usize][i_piece] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_piece in 0..DOUBLE_FACED_PIECES_LEN {
            for i_count in 0..HAND_MAX {
                self.hands[i_piece][i_count] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_phase in 0..PHASE_LEN {
            self.phase[i_phase] = rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
        }
    }

    /// TODO 指し手を使って差分更新
    /// 駒を動かしたあとに使う。
    /// TODO 持ち駒の枚数がトグルになってないぜ☆（＾～＾）？
    pub fn update_by_do_move(&self, history: &mut History, table: &GameTable, move_: &Movement) {
        // TODO １つ前の局面のハッシュ。
        let mut prev_hash = if history.length_from_the_middle() == 0 {
            history.starting_position_hash
        } else {
            history.position_hashs[history.length_from_the_middle() as usize - 1]
        };
        // TODO 指し手 で差分を適用
        // 移動する駒。
        match move_.source {
            FireAddress::Board(src_sq) => {
                let src_piece_hash_index =
                    if let Some(hash_index) = table.get_piece_board_hash_index(&move_.source) {
                        hash_index
                    } else {
                        panic!(Log::print_fatal("(Err.88) Invalid captured_move."));
                    };
                // 移動前マスに、動かしたい駒があるときのハッシュ。
                prev_hash ^= self.piece[src_sq.serial_number() as usize][src_piece_hash_index];
                // 移動後マスに、動かしたい駒があるときのハッシュ。
                match move_.destination {
                    FireAddress::Board(dst_sq) => {
                        prev_hash ^=
                            self.piece[dst_sq.serial_number() as usize][src_piece_hash_index];
                    }
                    FireAddress::Hand(_dst_drop) => {
                        panic!(Log::print_fatal("(Err.90) 未対応☆（＾～＾）"))
                    }
                }
            }
            FireAddress::Hand(src_drop) => {
                let src_drop = DoubleFacedPiece::from_phase_and_type(
                    history.get_turn(),
                    src_drop.piece.type_(),
                );
                let count = table.count_hand(src_drop);
                // 打つ前の駒の枚数のハッシュ。
                prev_hash ^= self.hands[src_drop as usize][count as usize];
                // 移動後マスに、打った駒があるときのハッシュ。
                match move_.destination {
                    FireAddress::Board(dst_sq) => {
                        prev_hash ^= self.piece[dst_sq.serial_number() as usize]
                            [src_drop.nonpromoted_piece_hash_index()];
                    }
                    FireAddress::Hand(_dst_drop) => {
                        panic!(Log::print_fatal("(Err.90) 未対応☆（＾～＾）"))
                    }
                }
            }
        }
        // 移動先に駒があれば、自分の持ち駒になります。
        if let Some(dst_piece_num) = table.piece_num_at(&move_.destination) {
            if let Some(dst_piece_hash_index) = table.get_piece_board_hash_index(&move_.destination)
            {
                match move_.destination {
                    FireAddress::Board(dst_sq) => {
                        // 移動先に駒があるケースの消去
                        prev_hash ^=
                            self.piece[dst_sq.serial_number() as usize][dst_piece_hash_index];
                        // 自分の持ち駒になるケースの追加
                        let double_faced_piece = table.get_double_faced_piece(dst_piece_num);
                        let drop = DoubleFacedPiece::from_phase_and_type(
                            history.get_turn(),
                            double_faced_piece.type_(),
                        );
                        let count = table.count_hand(drop);
                        // 打つ前の駒の枚数のハッシュ。
                        prev_hash ^= self.hands[double_faced_piece as usize][count as usize + 1];
                    }
                    FireAddress::Hand(_dst_drop) => {
                        panic!(Log::print_fatal("(Err.90) 未対応☆（＾～＾）"))
                    }
                }
            }
        }

        // TODO ハッシュ更新
        history.position_hashs[history.length_from_the_middle() as usize] = prev_hash;
    }

    /*
    /// 局面ハッシュを作り直す
    pub fn current_position(&self, game: &Position) -> u64 {
        let mut hash = self.table(&game.table);

        // 手番ハッシュ
        use crate::cosmic::recording::Phase::*;
        match game.history.get_turn() {
            First => hash ^= self.phase[PHASE_FIRST],
            Second => hash ^= self.phase[PHASE_SECOND],
        }

        hash
    }
    */

    /// 初期局面ハッシュを作り直す
    pub fn starting_position(&self, game: &Position) -> u64 {
        let mut hash = self.from_table(&game.starting_table);

        // 手番ハッシュ（後手固定）
        hash ^= self.phase[PHASE_SECOND];

        hash
    }

    /// 盤面からハッシュ作成
    fn from_table(&self, table: &GameTable) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for rank in RANK1U8..RANK10U8 {
            for file in (FILE1U8..FILE10U8).rev() {
                let sq = AbsoluteAddress2D::new(file, rank);
                if let Some(piece_hash_index) =
                    table.get_piece_board_hash_index(&FireAddress::Board(sq))
                {
                    hash ^= self.piece[sq.serial_number() as usize][piece_hash_index];
                }
            }
        }

        // 持ち駒ハッシュ
        HandAddresses::for_all(
            &mut |turn: &Phase, fire_hand: &FireAddress| match fire_hand {
                FireAddress::Board(_sq) => panic!(Log::print_fatal("(Err.175) 未対応☆（＾～＾）")),
                FireAddress::Hand(drop) => {
                    let drop = DoubleFacedPiece::from_phase_and_type(*turn, drop.piece.type_());
                    let count = table.count_hand(drop);
                    /*
                    debug_assert!(
                        count <= HAND_MAX,
                        "持ち駒 {:?} の枚数 {} <= {}",
                        drop,
                        count,
                        HAND_MAX
                    );
                    */
                    hash ^= self.hands[drop as usize][count as usize];
                }
            },
        );

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }
}
