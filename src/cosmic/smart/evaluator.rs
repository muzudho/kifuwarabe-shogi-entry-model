//!
//! １手指して、何点動いたかを評価するぜ☆（＾～＾）
//!
use crate::cosmic::smart::features::PieceType;

/// TODO 千日手の価値☆（＾～＾） ENGIN OPTIONにしたいぜ☆（＾～＾）
pub const REPITITION_VALUE: isize = -300;

pub struct Evaluation {
    /// 駒割の重み☆（＾～＾）1000分率☆（＾～＾）
    komawari_weight: isize,
    // 駒割だぜ☆（＾～＾）
    piece_allocation_value: isize,
    /// 成り駒ボーナスだぜ☆（＾～＾）
    promotion_value: isize,
}
impl Evaluation {
    pub fn new(komawari_weight: isize) -> Self {
        Evaluation {
            komawari_weight: komawari_weight,
            piece_allocation_value: 0,
            promotion_value: 0,
        }
    }
    pub fn centi_pawn(&self) -> isize {
        self.komawari()
    }
    pub fn komawari(&self) -> isize {
        self.komawari_weight * self.piece_allocation_value / 1000
    }

    pub fn before_search(&mut self) {
        // ひっくり返すぜ☆（＾～＾）
        self.piece_allocation_value *= -1;
        self.promotion_value *= -1;
    }

    pub fn after_search(&mut self) {
        // ひっくり返すぜ☆（＾～＾）
        self.piece_allocation_value *= -1;
        self.promotion_value *= -1;
    }

    /// # Arguments
    ///
    /// * `promotion_value` - 成ったら加点☆（＾～＾）
    pub fn after_do_move(
        &mut self,
        captured_piece_type: Option<PieceType>,
        promotion_value: isize,
    ) -> (isize, isize) {
        // 取った駒の価値を評価するぜ☆（＾～＾）
        let delta_captured_piece = Evaluation::caputured_piece_type_value(captured_piece_type);
        self.piece_allocation_value += delta_captured_piece;

        // 成り駒を取って降格させたら、成り駒評価値追加だぜ☆（＾～＾）
        let delta_promotion = if let Some(captured_piece_type_val) = captured_piece_type {
            if captured_piece_type_val
                .promoted()
            {
                captured_piece_type_val.double_faced_piece_type().promotion_value()
            } else {
                0
            }
        } else {
            0
        }
        // 進めた駒が成っても、評価値追加だぜ☆（＾～＾）
        +
        promotion_value;
        self.promotion_value += delta_promotion;

        (delta_captured_piece, delta_promotion)
    }

    pub fn before_undo_move(&mut self, delta_captured_piece: isize, delta_promotion: isize) {
        // 1手戻すぜ☆（＾～＾）
        self.piece_allocation_value -= delta_captured_piece;
        self.promotion_value -= delta_promotion;
    }

    /// 取った駒は相手の駒に決まってるぜ☆（＾～＾）
    /// 読みを深めていくと、当たってる駒を　あとで取っても同じだろ、とか思って取らないのは、駒割ではなく、別の方法で対応してくれだぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// Centi pawn.
    fn caputured_piece_type_value(captured_piece_type: Option<PieceType>) -> isize {
        if let Some(captured_piece_type_val) = captured_piece_type {
            captured_piece_type_val
                .double_faced_piece_type()
                .captured_value()
        } else {
            0
        }
    }
}
