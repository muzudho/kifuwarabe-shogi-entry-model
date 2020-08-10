use crate::{
    command_line_seek::CommandLineSeek,
    cosmic::recording::Phase,
    engine::Engine,
    log::LogExt,
    look_and_model::{PvString, Search},
    protocol::usi::Go,
};
use casual_logger::Log;
use rand::Rng;
use regex::Regex;
use std::fmt;

impl fmt::Display for Go {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "go btime {} wtime {} binc {} winc {}",
            self.btime, self.wtime, self.binc, self.winc
        )
    }
}

impl Go {
    /// bestmoveコマンドを送るぜ☆（＾～＾） 思考するのもこの中だぜ☆（＾～＾）
    pub fn go(engine: &mut Engine, p: &mut CommandLineSeek) {
        // go btime 40000 wtime 50000 binc 10000 winc 10000
        let go1 = Go::parse(p);
        Log::debug(&format!("Debug   | go=|{}|", go1));
        let mut tree = Search::new(engine.option_depth_not_to_give_up);

        // 残り時間と、追加時間☆（＾～＾）
        fn margined_msec(msec: u64) -> u64 {
            if 2000 < msec {
                msec - 2000
            } else {
                0
            }
        }
        let (msec, _minc) = match engine.position.history.get_turn() {
            // 2秒余裕を見ておけば、探索を中断できるだろ……☆（＾～＾）負の数になったらエラーな☆（＾～＾）
            Phase::First => (margined_msec(go1.btime), go1.binc),
            Phase::Second => (margined_msec(go1.wtime), go1.winc),
        };
        tree.think_msec = if engine.option_max_think_msec < msec {
            // 残り時間が、最大思考時間より長ければ充分だぜ☆（＾～＾）
            rand::thread_rng().gen_range(engine.option_min_think_msec, engine.option_max_think_msec)
                as u128
        } else if engine.option_min_think_msec < msec {
            // 残り時間が、最小思考時間より長いが、最長思考時間まで考えてられないな☆（＾～＾）
            rand::thread_rng().gen_range(engine.option_min_think_msec, msec) as u128
        } else if 3000 < msec {
            // 持ち時間が、最小思考時間未満、3秒より多いになったら☆（＾～＾）
            // 第一引数が負の数にならないように注意☆（＾～＾）
            rand::thread_rng().gen_range(0, msec - 2000) as u128
        } else {
            // ヤケクソの 1msec 指しだぜ☆（＾～＾）
            1
        };

        let ts = tree.iteration_deeping(engine);
        // その手を選んだ理由☆（＾～＾）
        Log::print_info(&Search::info_str(
            None,
            Some((tree.nodes, tree.nps())),
            Some(ts.bestmove.value),
            ts.bestmove.movement,
            &Some(PvString::String(ts.bestmove.reason.to_string())),
        ));
        // 例: bestmove 7g7f
        // 例: bestmove resign
        Log::print_notice(&format!(
            "bestmove {}",
            if let Some(bestmove) = ts.bestmove.movement {
                format!("{}", bestmove)
            } else {
                "resign".to_string()
            }
        ));
    }

    /// Example
    /// -------
    /// go btime 40000 wtime 50000 binc 10000 winc 10000
    pub fn parse(p: &mut CommandLineSeek) -> Go {
        let re = match Regex::new(r"^go btime (\d+) wtime (\d+) binc (\d+) winc (\d+)$") {
            Result::Ok(val) => val,
            Result::Err(e) => panic!(Log::print_fatal(&format!("(Err.29) Invalid regex=|{}|", e))),
        };
        if let Some(cap) = re.captures(p.line()) {
            Go {
                btime: match cap[1].parse() {
                    Result::Ok(val) => val,
                    Result::Err(e) => {
                        panic!(Log::print_fatal(&format!("(Err.35) Invalid cap1=|{}|", e)))
                    }
                },
                wtime: match cap[2].parse() {
                    Result::Ok(val) => val,
                    Result::Err(e) => {
                        panic!(Log::print_fatal(&format!("(Err.41) Invalid cap2=|{}|", e)))
                    }
                },
                binc: match cap[3].parse() {
                    Result::Ok(val) => val,
                    Result::Err(e) => {
                        panic!(Log::print_fatal(&format!("(Err.47) Invalid cap3=|{}|", e)))
                    }
                },
                winc: match cap[4].parse() {
                    Result::Ok(val) => val,
                    Result::Err(e) => {
                        panic!(Log::print_fatal(&format!("(Err.53) Invalid cap4=|{}|", e)))
                    }
                },
            }
        } else {
            // デバッグ時に `go` のみ打鍵した場合など。小さな数にします。
            Go {
                btime: 500,
                wtime: 500,
                binc: 0,
                winc: 0,
            }
        }
    }
}
