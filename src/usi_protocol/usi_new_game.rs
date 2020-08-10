use crate::engine::Engine;
use crate::position::Position;
use crate::usi_protocol::UsiNewGame;

impl UsiNewGame {
    pub fn usinewgame(engine: &mut Engine) {
        engine.position = Position::default();
    }
}
