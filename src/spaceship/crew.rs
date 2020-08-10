use crate::command_line_seek::CommandLineSeek;
use crate::config::*;
use crate::cosmic::{
    playing::PosNums,
    recording::{Movement, Phase},
    smart::square::{AbsoluteAddress2D, FILE1U8},
};
use crate::engine::Engine;
use crate::law::{cryptographic::*, generate_move::MoveGen};
use crate::log::LogExt;
use crate::look_and_model::{
    facility::Kitchen,
    position::{PositionLook, PositionLook2a, PositionLook2b},
};
use crate::usi_protocol::input_usi::*;
use casual_logger::Log;
use rand::Rng;

/// 船長：きふわらべ
///
/// 対局で許されている命令だけをするぜ☆（＾～＾）
pub struct Kifuwarabe {}
impl Kifuwarabe {
    pub fn usi() {
        Log::print_notice(&format!("id name {}", ENGINE_NAME));
        Log::print_notice(&format!("id author {}", ENGINE_AUTHOR));
        /*
        IO::writeln("option name BookFile type string default public.bin");
        IO::writeln("option name UseBook type check default true");
        IO::writeln("option name Selectivity type spin default 2 min 0 max 4");
        IO::writeln(
            "option name Style type combo default Normal var Solid var Normal var Risky",
        );
        IO::writeln("option name ResetLearning type button");
        IO::writeln("option name LearningFile type filename default <empty>");
        */
        // アルファベット順ではなく、将棋所のダイアログボックスが見やすくなるように並べろだぜ☆（＾～＾）
        // 大会ルール関連☆（＾～＾）
        Log::print_notice("option name MaxPly type spin default 320 min 1 max 10000");
        // 読みの深さ関連☆（＾～＾）
        Log::print_notice("option name DepthNotToGiveUp type spin default 4 min 1 max 8");
        Log::print_notice("option name MaxDepth type spin default 7 min 1 max 15");
        // 思考時間関連☆（＾～＾）
        Log::print_notice("option name MinThinkMsec type spin default 5000 min 0 max 599000");
        Log::print_notice("option name MaxThinkMsec type spin default 17000 min 1000 max 600000");
        Log::print_notice("usiok");
    }
}

/// 副船長：ちゆり
///
/// 対局でやっちゃいかん命令なら任せろだぜ☆（＾～＾）
pub struct Chiyuri {}
impl Chiyuri {
    pub fn do_(engine: &mut Engine, p: &mut CommandLineSeek) {
        // コマンド読取。棋譜に追加され、手目も増える
        if read_sasite(&mut engine.position, p) {
            // 手目を戻す
            engine.position.history.add_moves(-1);
            // 入っている指し手の通り指すぜ☆（＾～＾）
            let ply = engine.position.history.length_from_the_middle();
            let move_ = engine.position.history.movements[ply as usize];
            engine.position.redo_move(&engine.config, &move_);
        }
    }
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
    pub fn hash(engine: &Engine) {
        Log::print_notice("局面ハッシュ表示");
        let s = engine.position.get_positions_hash_text();
        Log::print_notice(&s);
    }
    pub fn how_much(line: &str) {
        // Example: how-much 7g7f
        let bestmove = &line[9..];
        Log::print_notice(&format!("Debug   | bestmove=|{}|", bestmove));
    }
    pub fn kifu(engine: &Engine) {
        Log::print_notice("棋譜表示");
        let s = engine.position.get_moves_history_text();
        Log::print_notice(&s);
    }
    /// 表示するだけ☆（＾～＾）
    pub fn list40(engine: &Engine) {
        Log::print_notice("----駒リスト40表示 ここから----");
        engine
            .position
            .table
            .for_all_pieces_on_table(&mut |i, adr, piece| {
                Log::print_notice(&format!(
                    "[{}]{}{}",
                    i,
                    if let Some(adr_val) = adr {
                        format!(" {:?}", adr_val)
                    } else {
                        " --".to_string()
                    },
                    if let Some(piece_val) = piece {
                        format!(" {} {:?}", piece_val.text1, piece_val.num)
                    } else {
                        " --".to_string()
                    }
                ));
            });
        Log::print_notice("----駒リスト40表示 ここまで----");
    }
    pub fn pos(engine: &Engine) {
        // 現局面表示
        let s = PositionLook::to_string(&engine.position, PosNums::Current);
        Log::print_notice(&s);
    }
    pub fn pos2(engine: &Engine) {
        // 現局面表示
        let s = format!(
            "{}{}{}{}",
            PositionLook2a::to_string(&engine.position, PosNums::Current),
            PositionLook2b::to_string(&engine.position, PosNums::Current),
            engine.position.table.pretty2c(),
            engine.position.table.pretty2d()
        );
        Log::print_notice(&s);
    }
    pub fn pos0(engine: &Engine) {
        // 初期局面表示
        let s = PositionLook::to_string(&engine.position, PosNums::Start);
        Log::print_notice(&s);
    }
    pub fn rand() {
        // 乱数の試し
        let secret_number = rand::thread_rng().gen_range(1, 101); //1~100
        Log::print_notice(&format!("乱数={}", secret_number));
    }
    pub fn same(engine: &Engine) {
        let count = engine.position.count_same_position();
        Log::print_notice(&format!("同一局面調べ count={}", count));
    }
    pub fn startpos(engine: &mut Engine) {
        // 平手初期局面
        set_position(
            &engine.config,
            &mut engine.position,
            &mut CommandLineSeek::new(&POS_1.to_string()),
        );
    }
    pub fn teigi_conv() {
        Log::print_notice("teigi::convのテスト");

        for ms in 1..9 {
            for hash in 0..10 {
                let sq = AbsoluteAddress2D::new(FILE1U8, ms);
                let next = push_sq_to_hash(hash, Some(&sq));
                let (hash_orig, square_orig) = pop_sq_from_hash(next);
                Log::print_notice( &format!("push_ms_to_hash(0b{:4b},0b{:5b})=0b{:11b} pop_sq_from_hash(...)=(0b{:4b},0b{:5b})"
                    ,hash
                    ,ms
                    ,next
                    ,hash_orig
                    ,if let Some(square_orig_val) = square_orig{ square_orig_val.serial_number()}else{0}
                ));
            }
        }
    }
    pub fn undo(engine: &mut Engine) {
        if !engine.position.undo_move() {
            Log::print_notice(&format!(
                "info string ply={} を、これより戻せません",
                engine.position.history.length_from_the_middle()
            ));
        }
    }
}
