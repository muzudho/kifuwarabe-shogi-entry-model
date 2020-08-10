use crate::engine::Engine;
use crate::position::Position;
use crate::protocol::usi::UsiNewGame;

impl UsiNewGame {
    pub fn usinewgame(engine: &mut Engine) {
        engine.position = Position::default();
    }
}
