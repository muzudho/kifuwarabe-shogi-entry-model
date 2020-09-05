use crate::{
    command_line_seek::CommandLineSeek,
    log::LogExt,
    look_and_model::{Position, Title},
    protocol::{
        usi::{Go, IsReady, Position as UsiPosition, SetOption, Usi, UsiNewGame},
        uxi::{
            Do, GenMove, Hash, HowMuch, Kifu, List40, Pos, Pos0, Pos2, Rand, Same, Startpos,
            TeigiConv, Undo,
        },
    },
    Config, Engine,
};
use casual_logger::{Log, Table};

impl Default for Engine {
    fn default() -> Self {
        Engine {
            position: Position::default(),
            config: Config::default(),
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
        // ゲームのハッシュの種をリセット
        self.config.hash_seed.big_bang();
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
            UsiNewGame::usinewgame(self);
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
            Usi::usi();
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
            Do::do_(self, p);
        // G
        } else if p.starts_with("genmove") {
            GenMove::genmove(&self);
        // H
        } else if p.starts_with("how-much") {
            HowMuch::how_much(p.line());
        } else if p.starts_with("hash") {
            Hash::hash(self);
        } else if p.starts_with("kifu") {
            Kifu::kifu(self);
        // L
        } else if p.starts_with("list40") {
            List40::list40(self);
        // P
        } else if p.starts_with("pos0") {
            Pos0::pos0(self);
        } else if p.starts_with("pos2") {
            Pos2::pos2(self);
        } else if p.starts_with("pos") {
            Pos::pos(self);
        // S
        } else if p.starts_with("startpos") {
            Startpos::startpos(self);
        // R
        } else if p.starts_with("rand") {
            Rand::rand();
        // S
        } else if p.starts_with("same") {
            Same::same(self);
        // T
        } else if p.starts_with("teigi::conv") {
            TeigiConv::teigi_conv();
        // U
        } else if p.starts_with("undo") {
            Undo::undo(self);
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
