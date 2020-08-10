use crate::{
    command_line_seek::CommandLineSeek,
    engine::Engine,
    protocol::{usi::input_usi::*, uxi::Do},
};

impl Do {
    pub fn do_(engine: &mut Engine, p: &mut CommandLineSeek) {
        // コマンド読取。棋譜に追加され、手目も増える
        if read_sasite(&mut engine.position, p) {
            // 手目を戻す
            engine.position.history.add_moves(-1);
            // 入っている指し手の通り指すぜ☆（＾～＾）
            let ply = engine.position.history.length_from_the_middle();
            let move_ = engine.position.history.movements[ply as usize];
            engine.position.redo_move(&engine.config, &move_);
        }
    }
}
