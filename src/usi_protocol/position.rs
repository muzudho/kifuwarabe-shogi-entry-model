use crate::command_line_seek::CommandLineSeek;
use crate::engine::Engine;
use crate::usi_protocol::input_usi::set_position;
use crate::usi_protocol::Position;

impl Position {
    pub fn position(engine: &mut Engine, line: &str) {
        // positionコマンドの読取を丸投げ
        set_position(engine, &mut CommandLineSeek::new(line));
    }
}
