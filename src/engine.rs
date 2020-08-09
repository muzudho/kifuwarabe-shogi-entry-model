use crate::command_line_seek::CommandLineSeek;
use crate::cosmic::universe::Universe;
use crate::log::LogExt;
use crate::spaceship::crew::{Chiyuri, Kifuwarabe};
use casual_logger::{Log, Table};

pub struct Engine {}
impl Default for Engine {
    fn default() -> Self {
        Engine {}
    }
}
impl Engine {
    pub fn enter(&mut self, universe: &mut Universe, line: &str) -> Option<Response> {
        // Write input to log.
        // 入力をログに書きます。
        Log::notice_t(&line, Table::default().str("Description", "Input."));

        // p is the acronym for parser.
        // p は parser の頭文字。
        let mut p = CommandLineSeek::new(&line);

        if p.len() == 0 {
            // 任せろだぜ☆（＾～＾）
            Chiyuri::len0(universe);
        } else if p.starts_with("go") {
            Kifuwarabe::go(universe, &mut p);
        } else if p.starts_with("usinewgame") {
            Kifuwarabe::usinewgame(universe);
        } else if p.starts_with("isready") {
            Kifuwarabe::isready();
        } else if p.starts_with("position") {
            Kifuwarabe::position(universe, &line);
        } else if p.starts_with("quit") {
            return Some(Response::Quit);
        } else if p.starts_with("setoption name ") {
            Kifuwarabe::setoption_name(universe, &mut CommandLineSeek::new(&line));
        } else if p.starts_with("sfen") {
            Log::print_notice(&format!("{}", universe.position.to_xfen(true)));
        } else if p.starts_with("usi") {
            Kifuwarabe::usi();
        } else if p.starts_with("xfen") {
            Log::print_notice(&format!("{}", universe.position.to_xfen(false)));
        } else {
            Engine::help_chiyuri(universe, &mut p);
        }

        None
    }

    /// 独自コマンド☆（＾～＾）
    fn help_chiyuri(universe: &mut Universe, p: &mut CommandLineSeek) {
        // D
        if p.starts_with("do ") {
            p.go_next_to("do ");
            Chiyuri::do_(universe, p);
        // G
        } else if p.starts_with("genmove") {
            Chiyuri::genmove(&universe);
        // H
        } else if p.starts_with("how-much") {
            Chiyuri::how_much(p.line());
        } else if p.starts_with("hash") {
            Chiyuri::hash(universe);
        } else if p.starts_with("kifu") {
            Chiyuri::kifu(universe);
        // L
        } else if p.starts_with("list40") {
            Chiyuri::list40(universe);
        // P
        } else if p.starts_with("pos0") {
            Chiyuri::pos0(universe);
        } else if p.starts_with("pos2") {
            Chiyuri::pos2(universe);
        } else if p.starts_with("pos") {
            Chiyuri::pos(universe);
        // S
        } else if p.starts_with("startpos") {
            Chiyuri::startpos(universe);
        // R
        } else if p.starts_with("rand") {
            Chiyuri::rand();
        // S
        } else if p.starts_with("same") {
            Chiyuri::same(universe);
        // T
        } else if p.starts_with("teigi::conv") {
            Chiyuri::teigi_conv();
        // U
        } else if p.starts_with("undo") {
            Chiyuri::undo(universe);
        }
    }
}

/// Engine response.
/// エンジンの応答。
pub enum Response {
    /// Quit.
    /// 終了。
    Quit,
}
