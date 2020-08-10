use crate::look_and_model::recording::Phase;
use std::fmt;

/*
impl Phase {
    pub fn turn(self) -> Phase {
        use self::Phase::*;
        match self {
            First => Second,
            Second => First,
        }
    }
}
*/
/// 後手（上手）を盤の下側に持ってきて表示するのを基本とするぜ☆（＾～＾）
impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // Windows Terminal では ▲、▽が半角サイズで表示されるので、それに合わせている☆（＾～＾） Microsoft 製品に最適化していいのか知らないが……☆（＾～＾）
        use self::Phase::*;
        match *self {
            First => write!(f, " ▲"),
            Second => write!(f, " ▽"),
        }
    }
}
