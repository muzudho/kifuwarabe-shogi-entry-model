use crate::{
    command_line_seek::CommandLineSeek,
    engine::Engine,
    protocol::{usi::input_usi::set_position, usi::Position},
};

impl Position {
    pub fn position(engine: &mut Engine, line: &str) {
        // positionコマンドの読取を丸投げ
        set_position(engine, &mut CommandLineSeek::new(line));
    }
}
