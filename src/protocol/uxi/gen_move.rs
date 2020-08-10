use crate::cosmic::recording::{Movement, Phase};
use crate::engine::Engine;
use crate::law::generate_move::MoveGen;
use crate::log::LogExt;
use crate::look_and_model::facility::Kitchen;
use crate::protocol::uxi::GenMove;
use casual_logger::Log;

impl GenMove {
    pub fn genmove(engine: &Engine) {
        // Generation move.
        // FIXME 合法手とは限らない
        let mut ways = Vec::<Movement>::new();
        MoveGen::make_move(
            &engine.position,
            match engine.position.history.get_turn() {
                Phase::First => &engine.position.movegen_phase.first_movegen,
                Phase::Second => &engine.position.movegen_phase.second_movegen,
            },
            &mut |way| {
                ways.push(way);
            },
        );
        Log::print_notice("----指し手生成(合法手とは限らない) ここから----");
        Kitchen::print_ways(&engine.position.table, &ways);
        Log::print_notice("----指し手生成(合法手とは限らない) ここまで----");
    }
}
