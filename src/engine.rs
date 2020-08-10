use crate::command_line_seek::CommandLineSeek;
use crate::log::LogExt;
use crate::look_and_model::Title;
use crate::position::Position;
use crate::spaceship::crew::{Chiyuri, Kifuwarabe};
use crate::usi_protocol::{Go, IsReady, Position as UsiPosition, SetOption};
use casual_logger::{Log, Table};

/// アプリケーション開始時に決め終えておくものだぜ☆（＾～＾）
pub struct Engine {
    /// 対局
    pub position: Position,
    /// 大会ルールの最大手数☆（＾～＾）
    pub option_max_ply: usize,
    /// 読みの最大深さ。
    pub option_max_depth: usize,
    /// 思考時間の最小秒☆（＾～＾）
    pub option_min_think_msec: u64,
    /// 思考時間の最大秒☆（＾～＾）
    pub option_max_think_msec: u64,
    /// 諦めない深さ☆（＾～＾）読み終わるまで、思考時間を無視するぜ☆（＾～＾）max_depth - 1 より小さくしろだぜ☆（＾～＾）
    pub option_depth_not_to_give_up: usize,
}
impl Default for Engine {
    fn default() -> Self {
        Engine {
            position: Position::default(),
            option_max_ply: 320,
            option_max_depth: 1,
            option_depth_not_to_give_up: 2,
            /// min < max にしろだぜ☆（＾～＾）
            option_min_think_msec: 7000,
            option_max_think_msec: 17000,
        }
    }
}
impl Engine {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        self.position.big_bang();
        // タイトル表示
        // １画面は２５行だが、最後の２行は開けておかないと、
        // カーソルが２行分場所を取るんだぜ☆（＾～＾）
        Title::pretty();
    }

    pub fn enter(&mut self, line: &str) -> Option<Response> {
        // Write input to log.
        // 入力をログに書きます。
        Log::notice_t(&line, Table::default().str("Description", "Input."));

        // p is the acronym for parser.
        // p は parser の頭文字。
        let mut p = CommandLineSeek::new(&line);

        if p.starts_with("go") {
            Go::go(self, &mut p);
        } else if p.starts_with("usinewgame") {
            Kifuwarabe::usinewgame(self);
        } else if p.starts_with("isready") {
            IsReady::isready();
        } else if p.starts_with("position") {
            UsiPosition::position(self, &line);
        } else if p.starts_with("quit") {
            return Some(Response::Quit);
        } else if p.starts_with("setoption name ") {
            SetOption::setoption_name(self, &mut CommandLineSeek::new(&line));
        } else if p.starts_with("sfen") {
            Log::print_notice(&format!("{}", self.position.to_xfen(true)));
        } else if p.starts_with("usi") {
            Kifuwarabe::usi();
        } else if p.starts_with("xfen") {
            Log::print_notice(&format!("{}", self.position.to_xfen(false)));
        } else {
            self.help_chiyuri(&mut p);
        }

        None
    }

    /// 独自コマンド☆（＾～＾）
    fn help_chiyuri(&mut self, p: &mut CommandLineSeek) {
        // D
        if p.starts_with("do ") {
            p.go_next_to("do ");
            Chiyuri::do_(self, p);
        // G
        } else if p.starts_with("genmove") {
            Chiyuri::genmove(&self);
        // H
        } else if p.starts_with("how-much") {
            Chiyuri::how_much(p.line());
        } else if p.starts_with("hash") {
            Chiyuri::hash(self);
        } else if p.starts_with("kifu") {
            Chiyuri::kifu(self);
        // L
        } else if p.starts_with("list40") {
            Chiyuri::list40(self);
        // P
        } else if p.starts_with("pos0") {
            Chiyuri::pos0(self);
        } else if p.starts_with("pos2") {
            Chiyuri::pos2(self);
        } else if p.starts_with("pos") {
            Chiyuri::pos(self);
        // S
        } else if p.starts_with("startpos") {
            Chiyuri::startpos(self);
        // R
        } else if p.starts_with("rand") {
            Chiyuri::rand();
        // S
        } else if p.starts_with("same") {
            Chiyuri::same(self);
        // T
        } else if p.starts_with("teigi::conv") {
            Chiyuri::teigi_conv();
        // U
        } else if p.starts_with("undo") {
            Chiyuri::undo(self);
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
