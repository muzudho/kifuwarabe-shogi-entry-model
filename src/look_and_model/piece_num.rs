use crate::look_and_model::PieceNum;
use std::*;

/// きふわらべ「USIでは使わないから、独自の表記をして構わないぜ☆」
impl fmt::Display for PieceNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // ▲ が半角サイズ、▽ が見た目が全角の半角サイズなのは、Windows Terminal の担当者 いい加減だぜ☆（＾～＾）
        use crate::look_and_model::PieceNum::*;
        write!(
            f,
            "{}",
            match *self {
                King1 => "01K ",
                King2 => "02K ",
                Gold3 => "03G ",
                Gold4 => "04G ",
                Gold5 => "05G ",
                Gold6 => "06G ",
                Silver7 => "07S ",
                Silver8 => "08S ",
                Silver9 => "09S ",
                Silver10 => "10S ",
                Knight11 => "11N ",
                Knight12 => "12N ",
                Knight13 => "13N ",
                Knight14 => "14N ",
                Lance15 => "15L ",
                Lance16 => "16L ",
                Lance17 => "17L ",
                Lance18 => "18L ",
                Bishop19 => "19B ",
                Bishop20 => "20B ",
                Rook21 => "21R ",
                Rook22 => "22R ",
                Pawn23 => "23P ",
                Pawn24 => "24P ",
                Pawn25 => "25P ",
                Pawn26 => "26P ",
                Pawn27 => "27P ",
                Pawn28 => "28P ",
                Pawn29 => "29P ",
                Pawn30 => "30P ",
                Pawn31 => "31P ",
                Pawn32 => "32P ",
                Pawn33 => "33P ",
                Pawn34 => "34P ",
                Pawn35 => "35P ",
                Pawn36 => "36P ",
                Pawn37 => "37P ",
                Pawn38 => "38P ",
                Pawn39 => "39P ",
                Pawn40 => "40P ",
            }
        )
    }
}
