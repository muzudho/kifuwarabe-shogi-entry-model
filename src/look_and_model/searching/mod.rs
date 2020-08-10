use crate::law::generate_move::PhaseOperation;

pub mod movegen_phase;

/// 指し手生成で、先手、後手で処理が変わるやつを吸収するぜ☆（＾～＾）
pub struct MovegenPhase {
    pub first_movegen: Box<dyn PhaseOperation>,
    pub second_movegen: Box<dyn PhaseOperation>,
}
