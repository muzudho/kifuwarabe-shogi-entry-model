use crate::{engine::Engine, look_and_model::Position, protocol::usi::UsiNewGame};

impl UsiNewGame {
    pub fn usinewgame(engine: &mut Engine) {
        engine.position = Position::default();
    }
}
