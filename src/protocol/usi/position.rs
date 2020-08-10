use crate::command_line_seek::CommandLineSeek;
use crate::engine::Engine;
use crate::protocol::usi::input_usi::set_position;
use crate::protocol::usi::Position;

impl Position {
    pub fn position(engine: &mut Engine, line: &str) {
        // positionコマンドの読取を丸投げ
        set_position(engine, &mut CommandLineSeek::new(line));
    }
}
