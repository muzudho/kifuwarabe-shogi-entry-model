use crate::{
    law::generate_move::{FirstOperation, SecondOperation},
    look_and_model::searching::MovegenPhase,
};

impl Default for MovegenPhase {
    fn default() -> Self {
        MovegenPhase {
            /// 指し手生成
            first_movegen: Box::new(FirstOperation::default()),
            second_movegen: Box::new(SecondOperation::default()),
        }
    }
}
