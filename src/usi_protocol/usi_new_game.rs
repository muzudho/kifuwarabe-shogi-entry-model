use crate::engine::Engine;
use crate::usi_protocol::UsiNewGame;

impl UsiNewGame {
    pub fn usinewgame(engine: &mut Engine) {
        engine.position.usi_new_game();
        // TODO これだとダメ☆（＾～＾） engine.position = Position::default();
    }
}
