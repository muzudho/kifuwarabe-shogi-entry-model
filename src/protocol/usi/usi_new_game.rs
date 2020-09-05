use crate::{look_and_model::Position, protocol::usi::UsiNewGame, Engine};

impl UsiNewGame {
    pub fn usinewgame(engine: &mut Engine) {
        engine.position = Position::default();
    }
}
