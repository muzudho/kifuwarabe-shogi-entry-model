use crate::{
    command_line_seek::CommandLineSeek,
    engine::Engine,
    protocol::{
        usi::input_usi::{set_position, POS_1},
        uxi::Startpos,
    },
};

impl Startpos {
    pub fn startpos(engine: &mut Engine) {
        // 平手初期局面
        set_position(engine, &mut CommandLineSeek::new(&POS_1.to_string()));
    }
}
