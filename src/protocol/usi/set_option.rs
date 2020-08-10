use crate::{
    command_line_seek::CommandLineSeek, engine::Engine, log::LogExt, protocol::usi::SetOption,
};
use casual_logger::{Log, Table};

impl SetOption {
    pub fn setoption_name(engine: &mut Engine, p: &mut CommandLineSeek) {
        // Example: setoption name USI_Ponder value true
        p.go_next_to("setoption name ");
        if let Some(name_len) = p.line()[p.current()..].find(' ') {
            let name = p.line()[p.current()..(p.current() + name_len)].to_string();
            p.go_next_to(&name);
            p.go_next_to(" value ");
            let value = &p.line()[(p.current())..];
            Log::debug_t(
                "SetOption",
                Table::default().str("Name", &name).str("Value", value),
            );
            match name.as_str() {
                "MaxPly" => {
                    engine.option_max_ply = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => panic!(Log::print_fatal(&format!(
                            "(Err.108) Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "DepthNotToGiveUp" => {
                    engine.option_depth_not_to_give_up = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => panic!(Log::print_fatal(&format!(
                            "(Err.117) Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "MaxDepth" => {
                    engine.option_max_depth = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => panic!(Log::print_fatal(&format!(
                            "(Err.126) Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "MinThinkMsec" => {
                    engine.option_min_think_msec = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => panic!(Log::print_fatal(&format!(
                            "(Err.135) Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "MaxThinkMsec" => {
                    engine.option_max_think_msec = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => panic!(Log::print_fatal(&format!(
                            "(Err.144) Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                _ => {}
            }
        };
    }
}
