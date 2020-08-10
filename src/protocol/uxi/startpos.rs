use crate::command_line_seek::CommandLineSeek;
use crate::engine::Engine;
use crate::protocol::usi::input_usi::*;
use crate::protocol::uxi::Startpos;

impl Startpos {
    pub fn startpos(engine: &mut Engine) {
        // 平手初期局面
        set_position(engine, &mut CommandLineSeek::new(&POS_1.to_string()));
    }
}
