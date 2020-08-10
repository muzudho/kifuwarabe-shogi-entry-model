use crate::{engine::Engine, protocol::usi::UsiNewGame, Position};

impl UsiNewGame {
    pub fn usinewgame(engine: &mut Engine) {
        engine.position = Position::default();
    }
}
