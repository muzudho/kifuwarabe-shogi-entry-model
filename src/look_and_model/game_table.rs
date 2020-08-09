use crate::cosmic::recording::{FireAddress, HandAddress, Movement, Phase};
use crate::cosmic::smart::features::{DoubleFacedPiece, PieceType};
use crate::cosmic::smart::square::{AbsoluteAddress2D, BOARD_MEMORY_AREA, RANK10U8, RANK1U8};
use crate::cosmic::toy_box::PieceNum;
use crate::cosmic::toy_box::*;
use crate::law::generate_move::Area;
use crate::law::speed_of_light::Nine299792458;
use crate::log::LogExt;
use crate::look_and_model::piece::{Piece, PIECE_WHITE_SPACE};
use crate::look_and_model::PHYSICAL_PIECE_TYPE_LEN;
use casual_logger::{Log, Table};
use num_traits::FromPrimitive;

/// ゲーム卓表示1
pub struct GameTableLook1 {}
impl GameTableLook1 {
    /// 表示
    pub fn to_string(table: &GameTable) -> String {
        // 局面表示
        format!(
            "\
[GameTable]

         9    8    7    6    5    4    3    2    1
        +----+----+----+----+----+----+----+----+----+
▲       |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}| a1   ▽
        +----+----+----+----+----+----+----+----+----+
R x{81:2}   |{9}|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}| b2   r x{88:2}
        +----+----+----+----+----+----+----+----+----+
B x{82:2}   |{18}|{19}|{20}|{21}|{22}|{23}|{24}|{25}|{26}| c3   b x{89:2}
        +----+----+----+----+----+----+----+----+----+
G x{83:2}   |{27}|{28}|{29}|{30}|{31}|{32}|{33}|{34}|{35}| d4   g x{90:2}
        +----+----+----+----+----+----+----+----+----+
S x{84:2}   |{36}|{37}|{38}|{39}|{40}|{41}|{42}|{43}|{44}| e5   s x{91:2}
        +----+----+----+----+----+----+----+----+----+
N x{85:2}   |{45}|{46}|{47}|{48}|{49}|{50}|{51}|{52}|{53}| f6   n x{92:2}
        +----+----+----+----+----+----+----+----+----+
L x{86:2}   |{54}|{55}|{56}|{57}|{58}|{59}|{60}|{61}|{62}| g7   l x{93:2}
        +----+----+----+----+----+----+----+----+----+
P x{87:2}   |{63}|{64}|{65}|{66}|{67}|{68}|{69}|{70}|{71}| h8   p x{94:2}
        +----+----+----+----+----+----+----+----+----+
        |{72}|{73}|{74}|{75}|{76}|{77}|{78}|{79}|{80}| i9
        +----+----+----+----+----+----+----+----+----+\
",
            Self::to_string3(table, 9, 1),
            Self::to_string3(table, 8, 1),
            Self::to_string3(table, 7, 1),
            Self::to_string3(table, 6, 1),
            Self::to_string3(table, 5, 1),
            Self::to_string3(table, 4, 1),
            Self::to_string3(table, 3, 1),
            Self::to_string3(table, 2, 1),
            Self::to_string3(table, 1, 1),
            Self::to_string3(table, 9, 2),
            Self::to_string3(table, 8, 2),
            Self::to_string3(table, 7, 2),
            Self::to_string3(table, 6, 2),
            Self::to_string3(table, 5, 2),
            Self::to_string3(table, 4, 2),
            Self::to_string3(table, 3, 2),
            Self::to_string3(table, 2, 2),
            Self::to_string3(table, 1, 2),
            Self::to_string3(table, 9, 3),
            Self::to_string3(table, 8, 3),
            Self::to_string3(table, 7, 3),
            Self::to_string3(table, 6, 3),
            Self::to_string3(table, 5, 3),
            Self::to_string3(table, 4, 3),
            Self::to_string3(table, 3, 3),
            Self::to_string3(table, 2, 3),
            Self::to_string3(table, 1, 3),
            Self::to_string3(table, 9, 4),
            Self::to_string3(table, 8, 4),
            Self::to_string3(table, 7, 4),
            Self::to_string3(table, 6, 4),
            Self::to_string3(table, 5, 4),
            Self::to_string3(table, 4, 4),
            Self::to_string3(table, 3, 4),
            Self::to_string3(table, 2, 4),
            Self::to_string3(table, 1, 4),
            Self::to_string3(table, 9, 5),
            Self::to_string3(table, 8, 5),
            Self::to_string3(table, 7, 5),
            Self::to_string3(table, 6, 5),
            Self::to_string3(table, 5, 5),
            Self::to_string3(table, 4, 5),
            Self::to_string3(table, 3, 5),
            Self::to_string3(table, 2, 5),
            Self::to_string3(table, 1, 5),
            Self::to_string3(table, 9, 6),
            Self::to_string3(table, 8, 6),
            Self::to_string3(table, 7, 6),
            Self::to_string3(table, 6, 6),
            Self::to_string3(table, 5, 6),
            Self::to_string3(table, 4, 6),
            Self::to_string3(table, 3, 6),
            Self::to_string3(table, 2, 6),
            Self::to_string3(table, 1, 6),
            Self::to_string3(table, 9, 7),
            Self::to_string3(table, 8, 7),
            Self::to_string3(table, 7, 7),
            Self::to_string3(table, 6, 7),
            Self::to_string3(table, 5, 7),
            Self::to_string3(table, 4, 7),
            Self::to_string3(table, 3, 7),
            Self::to_string3(table, 2, 7),
            Self::to_string3(table, 1, 7),
            Self::to_string3(table, 9, 8),
            Self::to_string3(table, 8, 8),
            Self::to_string3(table, 7, 8),
            Self::to_string3(table, 6, 8),
            Self::to_string3(table, 5, 8),
            Self::to_string3(table, 4, 8),
            Self::to_string3(table, 3, 8),
            Self::to_string3(table, 2, 8),
            Self::to_string3(table, 1, 8),
            Self::to_string3(table, 9, 9),
            Self::to_string3(table, 8, 9),
            Self::to_string3(table, 7, 9),
            Self::to_string3(table, 6, 9),
            Self::to_string3(table, 5, 9),
            Self::to_string3(table, 4, 9),
            Self::to_string3(table, 3, 9),
            Self::to_string3(table, 2, 9),
            Self::to_string3(table, 1, 9),
            table.count_hand(DoubleFacedPiece::Rook1),
            table.count_hand(DoubleFacedPiece::Bishop1),
            table.count_hand(DoubleFacedPiece::Gold1),
            table.count_hand(DoubleFacedPiece::Silver1),
            table.count_hand(DoubleFacedPiece::Knight1),
            table.count_hand(DoubleFacedPiece::Lance1),
            table.count_hand(DoubleFacedPiece::Pawn1),
            table.count_hand(DoubleFacedPiece::Rook2),
            table.count_hand(DoubleFacedPiece::Bishop2),
            table.count_hand(DoubleFacedPiece::Gold2),
            table.count_hand(DoubleFacedPiece::Silver2),
            table.count_hand(DoubleFacedPiece::Knight2),
            table.count_hand(DoubleFacedPiece::Lance2),
            table.count_hand(DoubleFacedPiece::Pawn2),
        )
    }
    fn to_string3(table: &GameTable, file: u8, rank: u8) -> String {
        if let Some(piece_info_val) =
            table.piece_info_at1(&FireAddress::Board(AbsoluteAddress2D::new(file, rank)))
        {
            format!("{}", piece_info_val.text1)
        } else {
            PIECE_WHITE_SPACE.to_string()
        }
    }
}

/// Board display type 2.
/// 盤表示２。
pub struct GameTableLook2a {}
impl GameTableLook2a {
    /// 表示
    pub fn to_string(table: &GameTable) -> String {
        // 局面表示
        // フォーマットの引数は 98個まで。
        format!(
            "{}{}{}",
            "[GameTable2a]

",
            format!(
                "  12   11   10    9    8    7    6    5    4    3    2    1    0
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{60} {55}|{50}|{45} {40} {35} {30}|{25} {20} {15} {10}|{5 }|{0 }| z
+    +    +    +----+----+----+----+----+----+----+----+----+----+
|{61} {56}|{51}|{46}|{41}|{36}|{31}|{26}|{21}|{16}|{11}|{6 }|{1 }| a
+----+    +----+----+----+----+----+----+----+----+----+----+    +
     |{57}|{52}|{47}|{42}|{37}|{32}|{27}|{22}|{17}|{12}|{7 }|{2 }| b
     +    +    +----+----+----+----+----+----+----+----+----+----+
     |{58}|{53}|{48}|{43}|{38}|{33}|{28}|{23}|{18}|{13}|{8 }|{3 }| c
     +    +----+----+----+----+----+----+----+----+----+----+    +
     |{59} {54}|{49}|{44}|{39}|{34}|{29}|{24}|{19}|{14}|{9 }|{4 }| d
",
                Self::to_string3(table, 0),
                Self::to_string3(table, 1),
                Self::to_string3(table, 2),
                Self::to_string3(table, 3),
                Self::to_string3(table, 4),
                Self::to_string3(table, 10),
                Self::to_string3(table, 11),
                Self::to_string3(table, 12),
                Self::to_string3(table, 13),
                Self::to_string3(table, 14),
                Self::to_string3(table, 20),
                Self::to_string3(table, 21),
                Self::to_string3(table, 22),
                Self::to_string3(table, 23),
                Self::to_string3(table, 24),
                Self::to_string3(table, 30),
                Self::to_string3(table, 31),
                Self::to_string3(table, 32),
                Self::to_string3(table, 33),
                Self::to_string3(table, 34),
                Self::to_string3(table, 40),
                Self::to_string3(table, 41),
                Self::to_string3(table, 42),
                Self::to_string3(table, 43),
                Self::to_string3(table, 44),
                Self::to_string3(table, 50),
                Self::to_string3(table, 51),
                Self::to_string3(table, 52),
                Self::to_string3(table, 53),
                Self::to_string3(table, 54),
                Self::to_string3(table, 60),
                Self::to_string3(table, 61),
                Self::to_string3(table, 62),
                Self::to_string3(table, 63),
                Self::to_string3(table, 64),
                Self::to_string3(table, 70),
                Self::to_string3(table, 71),
                Self::to_string3(table, 72),
                Self::to_string3(table, 73),
                Self::to_string3(table, 74),
                Self::to_string3(table, 80),
                Self::to_string3(table, 81),
                Self::to_string3(table, 82),
                Self::to_string3(table, 83),
                Self::to_string3(table, 84),
                Self::to_string3(table, 90),
                Self::to_string3(table, 91),
                Self::to_string3(table, 92),
                Self::to_string3(table, 93),
                Self::to_string3(table, 94),
                Self::to_string3(table, 100),
                Self::to_string3(table, 101),
                Self::to_string3(table, 102),
                Self::to_string3(table, 103),
                Self::to_string3(table, 104),
                Self::to_string3(table, 110),
                Self::to_string3(table, 111),
                Self::to_string3(table, 112),
                Self::to_string3(table, 113),
                Self::to_string3(table, 114),
                Self::to_string3(table, 120),
                Self::to_string3(table, 121),
            ),
            format!(
                "     +    +    +----+----+----+----+----+----+----+----+----+    +
     |{55} {50}|{45}|{40}|{35}|{30}|{25}|{20}|{15}|{10}|{5 }|{0 }| e
     +    +    +----+----+----+----+----+----+----+----+----+    +
     |{56} {51}|{46}|{41}|{36}|{31}|{26}|{21}|{16}|{11}|{6 }|{1 }| f
     +    +    +----+----+----+----+----+----+----+----+----+----+
     |{57} {52}|{47}|{42}|{37}|{32}|{27}|{22}|{17}|{12}|{7 }|{2 }| g
     +    +    +----+----+----+----+----+----+----+----+----+    +
     |{58} {53}|{48}|{43}|{38}|{33}|{28}|{23}|{18}|{13}|{8 }|{3 }| h
     +    +    +----+----+----+----+----+----+----+----+----+    +
     |{59} {54}|{49}|{44}|{39}|{34}|{29}|{24}|{19}|{14}|{9 }|{4 }| i
     +----+----+----+----+----+----+----+----+----+----+----+----+
",
                Self::to_string3(table, 5),
                Self::to_string3(table, 6),
                Self::to_string3(table, 7),
                Self::to_string3(table, 8),
                Self::to_string3(table, 9),
                Self::to_string3(table, 15),
                Self::to_string3(table, 16),
                Self::to_string3(table, 17),
                Self::to_string3(table, 18),
                Self::to_string3(table, 19),
                Self::to_string3(table, 25),
                Self::to_string3(table, 26),
                Self::to_string3(table, 27),
                Self::to_string3(table, 28),
                Self::to_string3(table, 29),
                Self::to_string3(table, 35),
                Self::to_string3(table, 36),
                Self::to_string3(table, 37),
                Self::to_string3(table, 38),
                Self::to_string3(table, 39),
                Self::to_string3(table, 45),
                Self::to_string3(table, 46),
                Self::to_string3(table, 47),
                Self::to_string3(table, 48),
                Self::to_string3(table, 49),
                Self::to_string3(table, 55),
                Self::to_string3(table, 56),
                Self::to_string3(table, 57),
                Self::to_string3(table, 58),
                Self::to_string3(table, 59),
                Self::to_string3(table, 65),
                Self::to_string3(table, 66),
                Self::to_string3(table, 67),
                Self::to_string3(table, 68),
                Self::to_string3(table, 69),
                Self::to_string3(table, 75),
                Self::to_string3(table, 76),
                Self::to_string3(table, 77),
                Self::to_string3(table, 78),
                Self::to_string3(table, 79),
                Self::to_string3(table, 85),
                Self::to_string3(table, 86),
                Self::to_string3(table, 87),
                Self::to_string3(table, 88),
                Self::to_string3(table, 89),
                Self::to_string3(table, 95),
                Self::to_string3(table, 96),
                Self::to_string3(table, 97),
                Self::to_string3(table, 98),
                Self::to_string3(table, 99),
                Self::to_string3(table, 105),
                Self::to_string3(table, 106),
                Self::to_string3(table, 107),
                Self::to_string3(table, 108),
                Self::to_string3(table, 109),
                Self::to_string3(table, 115),
                Self::to_string3(table, 116),
                Self::to_string3(table, 117),
                Self::to_string3(table, 118),
                Self::to_string3(table, 119),
            )
        )
    }
    fn to_string3(table: &GameTable, serial: u8) -> String {
        if let Some(sq) = AbsoluteAddress2D::from_absolute_address(serial) {
            if let Some(piece_info_val) = table.piece_info_num_at(&FireAddress::Board(sq)) {
                format!("{}", piece_info_val.text1).to_string()
            } else {
                "    ".to_string()
            }
        } else {
            // 0 は None.
            "    ".to_string()
        }
    }
}
/// Address display type 2 of all pieces.
/// 全ての駒の番地。
pub struct GameTableLook2b {}
impl GameTableLook2b {
    /// 表示
    pub fn to_string(table: &GameTable) -> String {
        // 局面表示
        // フォーマットの引数は 98個まで。
        format!(
            "{}{}{}{}",
            format!(
                " 01K  02K  03G  04G  05G  06G  07S  08S  09S  10S  11N  12N  13N  14N  15L  16L  17L  18L  19B  20B
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
",
                Self::to_string3(table, PieceNum::King1),
                Self::to_string3(table, PieceNum::King2),
                Self::to_string3(table, PieceNum::Gold3),
                Self::to_string3(table, PieceNum::Gold4),
                Self::to_string3(table, PieceNum::Gold5),
                Self::to_string3(table, PieceNum::Gold6),
                Self::to_string3(table, PieceNum::Silver7),
                Self::to_string3(table, PieceNum::Silver8),
                Self::to_string3(table, PieceNum::Silver9),
                Self::to_string3(table, PieceNum::Silver10),
                Self::to_string3(table, PieceNum::Knight11),
                Self::to_string3(table, PieceNum::Knight12),
                Self::to_string3(table, PieceNum::Knight13),
                Self::to_string3(table, PieceNum::Knight14),
                Self::to_string3(table, PieceNum::Lance15),
                Self::to_string3(table, PieceNum::Lance16),
                Self::to_string3(table, PieceNum::Lance17),
                Self::to_string3(table, PieceNum::Lance18),
                Self::to_string3(table, PieceNum::Bishop19),
                Self::to_string3(table, PieceNum::Bishop20),
            ),
            format!(
                "\
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
",
                Self::to_string4(table, PieceNum::King1),
                Self::to_string4(table, PieceNum::King2),
                Self::to_string4(table, PieceNum::Gold3),
                Self::to_string4(table, PieceNum::Gold4),
                Self::to_string4(table, PieceNum::Gold5),
                Self::to_string4(table, PieceNum::Gold6),
                Self::to_string4(table, PieceNum::Silver7),
                Self::to_string4(table, PieceNum::Silver8),
                Self::to_string4(table, PieceNum::Silver9),
                Self::to_string4(table, PieceNum::Silver10),
                Self::to_string4(table, PieceNum::Knight11),
                Self::to_string4(table, PieceNum::Knight12),
                Self::to_string4(table, PieceNum::Knight13),
                Self::to_string4(table, PieceNum::Knight14),
                Self::to_string4(table, PieceNum::Lance15),
                Self::to_string4(table, PieceNum::Lance16),
                Self::to_string4(table, PieceNum::Lance17),
                Self::to_string4(table, PieceNum::Lance18),
                Self::to_string4(table, PieceNum::Bishop19),
                Self::to_string4(table, PieceNum::Bishop20),
            ),
            format!(
                " 21R  22R  23P  24P  25P  26P  27P  28P  29P  30P  31P  32P  33P  34P  35P  36P  37P  38P  39P  40P
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
",
                Self::to_string3(table, PieceNum::Rook21),
                Self::to_string3(table, PieceNum::Rook22),
                Self::to_string3(table, PieceNum::Pawn23),
                Self::to_string3(table, PieceNum::Pawn24),
                Self::to_string3(table, PieceNum::Pawn25),
                Self::to_string3(table, PieceNum::Pawn26),
                Self::to_string3(table, PieceNum::Pawn27),
                Self::to_string3(table, PieceNum::Pawn28),
                Self::to_string3(table, PieceNum::Pawn29),
                Self::to_string3(table, PieceNum::Pawn30),
                Self::to_string3(table, PieceNum::Pawn31),
                Self::to_string3(table, PieceNum::Pawn32),
                Self::to_string3(table, PieceNum::Pawn33),
                Self::to_string3(table, PieceNum::Pawn34),
                Self::to_string3(table, PieceNum::Pawn35),
                Self::to_string3(table, PieceNum::Pawn36),
                Self::to_string3(table, PieceNum::Pawn37),
                Self::to_string3(table, PieceNum::Pawn38),
                Self::to_string3(table, PieceNum::Pawn39),
                Self::to_string3(table, PieceNum::Pawn40),
            ),
            format!(
                "\
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
",
                Self::to_string4(table, PieceNum::Rook21),
                Self::to_string4(table, PieceNum::Rook22),
                Self::to_string4(table, PieceNum::Pawn23),
                Self::to_string4(table, PieceNum::Pawn24),
                Self::to_string4(table, PieceNum::Pawn25),
                Self::to_string4(table, PieceNum::Pawn26),
                Self::to_string4(table, PieceNum::Pawn27),
                Self::to_string4(table, PieceNum::Pawn28),
                Self::to_string4(table, PieceNum::Pawn29),
                Self::to_string4(table, PieceNum::Pawn30),
                Self::to_string4(table, PieceNum::Pawn31),
                Self::to_string4(table, PieceNum::Pawn32),
                Self::to_string4(table, PieceNum::Pawn33),
                Self::to_string4(table, PieceNum::Pawn34),
                Self::to_string4(table, PieceNum::Pawn35),
                Self::to_string4(table, PieceNum::Pawn36),
                Self::to_string4(table, PieceNum::Pawn37),
                Self::to_string4(table, PieceNum::Pawn38),
                Self::to_string4(table, PieceNum::Pawn39),
                Self::to_string4(table, PieceNum::Pawn40),
            ),
        )
    }
    fn to_string3(table: &GameTable, piece_num: PieceNum) -> String {
        if let Some(piece_info_val) = table.piece_info_address_at(piece_num) {
            format!("{: >4}", piece_info_val.text1).to_string()
        } else {
            "    ".to_string()
        }
    }
    fn to_string4(table: &GameTable, piece_num: PieceNum) -> String {
        if let Some(piece_info_val) = table.piece_info_piece_at(piece_num) {
            format!("{: >4}", piece_info_val.text1).to_string()
        } else {
            "    ".to_string()
        }
    }
}
/// Address display type 2 of hand piece peak.
/// 全ての持ち駒の次の番地。
pub struct GameTableLook2c {}
impl GameTableLook2c {
    /// 表示
    pub fn to_string(table: &GameTable) -> String {
        // 局面表示
        // フォーマットの引数は 98個まで。
        format!(
            " K2   K1   G2   G1   S2   S1   N2   N1   L2   L1   B2   B1   R2   R1   P2   P1
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
",
            Self::to_string2(table, DoubleFacedPiece::King2),
            Self::to_string2(table, DoubleFacedPiece::King1),
            Self::to_string2(table, DoubleFacedPiece::Gold2),
            Self::to_string2(table, DoubleFacedPiece::Gold1),
            Self::to_string2(table, DoubleFacedPiece::Silver2),
            Self::to_string2(table, DoubleFacedPiece::Silver1),
            Self::to_string2(table, DoubleFacedPiece::Knight2),
            Self::to_string2(table, DoubleFacedPiece::Knight1),
            Self::to_string2(table, DoubleFacedPiece::Lance2),
            Self::to_string2(table, DoubleFacedPiece::Lance1),
            Self::to_string2(table, DoubleFacedPiece::Bishop2),
            Self::to_string2(table, DoubleFacedPiece::Bishop1),
            Self::to_string2(table, DoubleFacedPiece::Rook2),
            Self::to_string2(table, DoubleFacedPiece::Rook1),
            Self::to_string2(table, DoubleFacedPiece::Pawn2),
            Self::to_string2(table, DoubleFacedPiece::Pawn1),
        )
    }
    fn to_string2(table: &GameTable, piece: DoubleFacedPiece) -> String {
        format!("{: >4}", table.hand_cur(piece)).to_string()
    }
}

/// 卓☆（＾～＾）
/// でかいのでコピーもクローンも不可☆（＾～＾）！
/// 10の位を筋、1の位を段とする。
/// 0筋、0段は未使用
pub struct GameTable {
    /// 盤に、駒が紐づくぜ☆（＾～＾）
    board: [Option<PieceNum>; BOARD_MEMORY_AREA as usize],
    hand_king1_cur: isize,
    hand_rook1_cur: isize,
    hand_bishop1_cur: isize,
    hand_gold1_cur: isize,
    hand_silver1_cur: isize,
    hand_knight1_cur: isize,
    hand_lance1_cur: isize,
    hand_pawn1_cur: isize,
    hand_king2_cur: isize,
    hand_rook2_cur: isize,
    hand_bishop2_cur: isize,
    hand_gold2_cur: isize,
    hand_silver2_cur: isize,
    hand_knight2_cur: isize,
    hand_lance2_cur: isize,
    hand_pawn2_cur: isize,
    /// 背番号付きの駒に、番地が紐づいているぜ☆（＾～＾）
    address_list: [FireAddress; NAMED_PIECES_LEN],
    /// 駒の背番号に、駒が紐づくぜ☆（＾～＾）
    piece_list: [Piece; NAMED_PIECES_LEN],
    /// 駒の背番号を付けるのに使うぜ☆（＾～＾）
    double_faced_piece_type_index: [usize; PHYSICAL_PIECE_TYPE_LEN],
    /// 指し手生成に利用☆（＾～＾）
    pub area: Area,
}
impl Default for GameTable {
    fn default() -> Self {
        GameTable {
            // 盤上
            board: [None; BOARD_MEMORY_AREA],
            /// 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
            address_list: [FireAddress::default(); NAMED_PIECES_LEN],
            /// 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
            piece_list: [Piece::King1; NAMED_PIECES_LEN],
            double_faced_piece_type_index: [
                PieceNum::King1 as usize,
                PieceNum::Rook21 as usize,
                PieceNum::Bishop19 as usize,
                PieceNum::Gold3 as usize,
                PieceNum::Silver7 as usize,
                PieceNum::Knight11 as usize,
                PieceNum::Lance15 as usize,
                PieceNum::Pawn23 as usize,
            ],
            area: Area::default(),
            hand_king1_cur: GameTable::hand_start(DoubleFacedPiece::King1),
            hand_rook1_cur: GameTable::hand_start(DoubleFacedPiece::Rook1),
            hand_bishop1_cur: GameTable::hand_start(DoubleFacedPiece::Bishop1),
            hand_gold1_cur: GameTable::hand_start(DoubleFacedPiece::Gold1),
            hand_silver1_cur: GameTable::hand_start(DoubleFacedPiece::Silver1),
            hand_knight1_cur: GameTable::hand_start(DoubleFacedPiece::Knight1),
            hand_lance1_cur: GameTable::hand_start(DoubleFacedPiece::Lance1),
            hand_pawn1_cur: GameTable::hand_start(DoubleFacedPiece::Pawn1),
            hand_king2_cur: GameTable::hand_start(DoubleFacedPiece::King2),
            hand_rook2_cur: GameTable::hand_start(DoubleFacedPiece::Rook2),
            hand_bishop2_cur: GameTable::hand_start(DoubleFacedPiece::Bishop2),
            hand_gold2_cur: GameTable::hand_start(DoubleFacedPiece::Gold2),
            hand_silver2_cur: GameTable::hand_start(DoubleFacedPiece::Silver2),
            hand_knight2_cur: GameTable::hand_start(DoubleFacedPiece::Knight2),
            hand_lance2_cur: GameTable::hand_start(DoubleFacedPiece::Lance2),
            hand_pawn2_cur: GameTable::hand_start(DoubleFacedPiece::Pawn2),
        }
    }
}
impl GameTable {
    pub fn clear(&mut self) {
        self.board = [None; BOARD_MEMORY_AREA];
        // 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
        self.address_list = [FireAddress::default(); NAMED_PIECES_LEN];
        // 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
        self.piece_list = [Piece::King1; NAMED_PIECES_LEN];
        self.double_faced_piece_type_index = [
            PieceNum::King1 as usize,
            PieceNum::Rook21 as usize,
            PieceNum::Bishop19 as usize,
            PieceNum::Gold3 as usize,
            PieceNum::Silver7 as usize,
            PieceNum::Knight11 as usize,
            PieceNum::Lance15 as usize,
            PieceNum::Pawn23 as usize,
        ];
    }

    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, table: &GameTable) {
        self.board = table.board.clone();
        self.address_list = table.address_list.clone();
        self.piece_list = table.piece_list.clone();
        self.double_faced_piece_type_index = table.double_faced_piece_type_index.clone();
    }

    pub fn get_piece_string(&self, num: PieceNum) -> String {
        format!("{}", self.piece_list[num as usize])
    }
    pub fn get_phase(&self, num: PieceNum) -> Phase {
        self.piece_list[num as usize].phase()
    }
    pub fn get_type(&self, num: PieceNum) -> PieceType {
        self.piece_list[num as usize].type_()
    }
    pub fn get_double_faced_piece(&self, num: PieceNum) -> DoubleFacedPiece {
        self.piece_list[num as usize].double_faced_piece()
    }
    /*
    pub fn get_double_faced_piece_type(&self, num: PieceNum) -> DoubleFacedPieceType {
        self.piece_list[num as usize].double_faced_piece().type_()
    }
    */
    fn new_piece_num(&mut self, piece: Piece, num: PieceNum) -> PieceNum {
        self.piece_list[num as usize] = piece;
        num
    }
    pub fn turn_phase(&mut self, num: PieceNum) {
        self.piece_list[num as usize] = self.piece_list[num as usize].captured();
    }
    // 成り駒にします。
    pub fn promote(&mut self, num: PieceNum) {
        self.piece_list[num as usize] = self.piece_list[num as usize].promoted();
    }
    // 成っていない駒にします。
    pub fn demote(&mut self, num: PieceNum) {
        self.piece_list[num as usize] = self.piece_list[num as usize].demoted();
    }

    /// ドゥ時の動き。
    /// 駒の先後を反転させるぜ☆（＾～＾）
    // あれば　盤の相手の駒を先後反転して、自分の駒台に置きます。
    pub fn rotate_piece_board_to_hand_on_do(&mut self, turn: Phase, move_: &Movement) {
        // Captured piece number.
        // 衝突された駒の背番号。
        if let Some(cap) = self.pop_piece(turn, &move_.destination) {
            // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
            // 先後ひっくり返す。
            self.turn_phase(cap);
            self.push_piece(
                &FireAddress::Hand(HandAddress::new(
                    self.get_double_faced_piece(cap),
                    AbsoluteAddress2D::default(),
                )),
                Some(cap),
            );
        }
    }

    /// アンドゥ時の動き。
    /// あれば、指し手で取った駒の先後をひっくり返せば、自分の駒台にある駒を取り出せるので取り出して、盤の上に指し手の取った駒のまま駒を置きます。
    pub fn rotate_piece_hand_to_board_on_undo(&mut self, turn: Phase, move_: &Movement) {
        if let Some(move2_val) = move_.captured {
            // 行き先の駒は、まだ自分の駒台にある。
            match move2_val.destination {
                FireAddress::Board(_) => {
                    panic!(Log::print_fatal("(Err.653) DropなのにBoard?"));
                }
                FireAddress::Hand(drop) => {
                    let drop = DoubleFacedPiece::from_phase_and_type(turn, drop.piece.type_());
                    let dst_piece_t = if let Some(dst_piece_t) = self.last_hand_type(drop) {
                        dst_piece_t
                    } else {
                        // 行き先に駒が無かった。
                        panic!(Log::print_fatal_t(
                            "(Err.441) Invalid piece_type on undo.",
                            Table::default()
                                .str("Turn", &format!("{:?}", turn))
                                .str("Move", &format!("{:?}", move_))
                                .str("Move2Dst", &format!("{}", &move2_val.destination))
                                .str("GameTable1", &GameTableLook1::to_string(self))
                                .str("GameTable2a", &GameTableLook2a::to_string(&self))
                                .str("GameTable2b", &GameTableLook2b::to_string(&self))
                                .str("GameTable2c", &GameTableLook2c::to_string(&self))
                        ));
                    };
                    // 取った方の駒台の先後に合わせるぜ☆（＾～＾）
                    // 取った方の持ち駒を減らす
                    let piece_num = {
                        // TODO テスト中☆（＾～＾）
                        let double_faced_piece = DoubleFacedPiece::from_phase_and_type(
                            turn,
                            dst_piece_t.double_faced_piece_type(),
                        );
                        let fire1 = FireAddress::Hand(HandAddress::new(
                            double_faced_piece,
                            AbsoluteAddress2D::default(),
                        ));
                        if let Some(piece_num) = self.pop_piece(turn, &fire1) {
                            piece_num
                        } else {
                            panic!(Log::print_fatal("(Err.461) Invalid piece_num."));
                        }
                    };
                    // 駒の先後をひっくり返す。
                    self.turn_phase(piece_num);
                    if dst_piece_t.promoted() {
                        // 成り駒にします。
                        self.promote(piece_num);
                    } else {
                        // 成っていない駒にします。
                        self.demote(piece_num);
                    }
                    // 取られた方に、駒を返すぜ☆（＾～＾）置くのは指し手の移動先☆（＾～＾）
                    // self.turn_phase(piece_num); // これかくとバグる
                    self.push_piece(&move_.destination, Some(piece_num));
                }
            }
        }
    }
    /// 駒を置く。
    pub fn push_piece(&mut self, fire: &FireAddress, piece_num: Option<PieceNum>) {
        match fire {
            FireAddress::Board(sq) => {
                if let Some(piece_num_val) = piece_num {
                    // マスに駒を置きます。
                    self.board[sq.serial_number() as usize] = piece_num;
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] = FireAddress::Board(*sq);
                } else {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                }
            }
            FireAddress::Hand(_drop_type) => {
                if let Some(piece_num_val) = piece_num {
                    // 持ち駒を１つ増やします。
                    self.push_hand(self.get_double_faced_piece(piece_num_val), piece_num_val);
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] = *fire;
                }
            }
        }
    }
    /// 駒を取りのぞく。
    pub fn pop_piece(&mut self, turn: Phase, fire: &FireAddress) -> Option<PieceNum> {
        match fire {
            FireAddress::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                    // TODO 背番号の番地を、ゴミ値で塗りつぶすが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                    self.address_list[piece_num_val as usize] = FireAddress::default();
                }
                piece_num
            }
            FireAddress::Hand(drop) => {
                // 場所で指定します。
                // 台から取りのぞきます。
                let drop = DoubleFacedPiece::from_phase_and_type(turn, drop.piece.type_());
                let piece_num = self.pop_hand(drop);
                // TODO 背番号の番地に、ゴミ値を入れて消去するが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                // TODO 持ってない駒でも、マイナス１オーバーフローしたところに別の駒があれば、打てるのはよくないぜ☆（＾～＾）
                self.address_list[piece_num as usize] = FireAddress::default();
                Some(piece_num)
            }
        }
    }

    /// 散らばっている駒に、背番号を付けて、駒台に置くぜ☆（＾～＾）
    pub fn init_hand(&mut self, turn: Phase, piece_type: PieceType) {
        // 駒に背番号を付けるぜ☆（＾～＾）
        let piece_num = self.numbering_piece(turn, piece_type);
        // 駒台に置くぜ☆（＾～＾）
        let drop = FireAddress::Hand(HandAddress::new(
            self.get_double_faced_piece(piece_num),
            AbsoluteAddress2D::default(),
        ));
        self.push_piece(&drop, Some(piece_num));
    }

    /// 駒の新しい背番号を生成します。
    pub fn numbering_piece(&mut self, turn: Phase, piece_type: PieceType) -> PieceNum {
        let piece = Piece::from_phase_and_piece_type(turn, piece_type);
        match piece {
            // 玉だけ、先後は決まってるから従えだぜ☆（＾～＾）
            Piece::King1 => self.new_piece_num(piece, PieceNum::King1),
            Piece::King2 => self.new_piece_num(piece, PieceNum::King2),
            _ => {
                let drop_type = piece.double_faced_piece().type_() as usize;
                // 玉以外の背番号は、先後に関わりなく SFENに書いてあった順で☆（＾～＾）
                let piece_num = if let Some(piece_num) =
                    PieceNum::from_usize(self.double_faced_piece_type_index[drop_type])
                {
                    piece_num
                } else {
                    panic!(Log::print_fatal("(Err.556) Invalid piece_num."));
                };
                // カウントアップ☆（＾～＾）
                self.double_faced_piece_type_index[drop_type] += 1;
                self.new_piece_num(piece, piece_num)
            }
        }
    }

    /// 歩が置いてあるか確認
    pub fn exists_pawn_on_file(&self, turn: Phase, file: u8) -> bool {
        for rank in RANK1U8..RANK10U8 {
            if let Some(piece_num) =
                self.piece_num_at(&FireAddress::Board(AbsoluteAddress2D::new(file, rank)))
            {
                if self.get_phase(piece_num) == turn && self.get_type(piece_num) == PieceType::Pawn
                {
                    return true;
                }
            }
        }
        false
    }
    /// ハッシュを作るときに利用。盤上専用。
    pub fn get_piece_board_hash_index(&self, addr: &FireAddress) -> Option<usize> {
        match addr {
            FireAddress::Board(sq) => {
                if let Some(piece_num) = self.board[sq.serial_number() as usize] {
                    Some(self.piece_list[piece_num as usize] as usize)
                } else {
                    None
                }
            }
            FireAddress::Hand(_drop_type) => panic!(Log::print_fatal(&format!(
                "(Err.345) 駒台は非対応☆（＾～＾）！",
            ))),
        }
    }
    /// TODO Piece をカプセル化したい。外に出したくないぜ☆（＾～＾）
    /// 升で指定して駒を取得。
    /// 駒台には対応してない。 -> 何に使っている？
    pub fn piece_num_at(&self, fire: &FireAddress) -> Option<PieceNum> {
        match fire {
            FireAddress::Board(sq) => self.board[sq.serial_number() as usize],
            FireAddress::Hand(drop) => {
                // let drop = DoubleFacedPiece::from_phase_and_type(turn, drop.piece.type_());
                self.last_hand_num(drop.piece)
            }
        }
    }
    pub fn piece_num_on_board_at(&self, sq: &AbsoluteAddress2D) -> Option<PieceNum> {
        self.board[sq.serial_number() as usize]
    }
    /// 通常盤表示用。
    pub fn piece_info_at1(&self, addr: &FireAddress) -> Option<PieceInfo> {
        match addr {
            FireAddress::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    Some(PieceInfo::new(
                        &format!("{:?}", self.piece_list[piece_num_val as usize]),
                        piece_num_val,
                    ))
                } else {
                    None
                }
            }
            _ => panic!(Log::print_fatal(&format!(
                "(Err.321) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 盤2表示用。
    pub fn piece_info_num_at(&self, addr: &FireAddress) -> Option<PieceInfo> {
        match addr {
            FireAddress::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    Some(PieceInfo::new(&format!("{}", piece_num_val), piece_num_val))
                } else {
                    None
                }
            }
            _ => panic!(Log::print_fatal(&format!(
                "(Err.321) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 盤2表示用。
    pub fn piece_info_address_at(&self, piece_num: PieceNum) -> Option<PieceInfo> {
        Some(PieceInfo::new(
            &format!("{}", self.address_list[piece_num as usize]),
            piece_num,
        ))
    }
    /// 盤2表示用。
    pub fn piece_info_piece_at(&self, piece_num: PieceNum) -> Option<PieceInfo> {
        Some(PieceInfo::new(
            &format!("{:?}", self.piece_list[piece_num as usize]),
            piece_num,
        ))
    }
    /// 指し手生成で使うぜ☆（＾～＾）有無を調べるぜ☆（＾～＾）
    pub fn last_hand_type(&self, drop: DoubleFacedPiece) -> Option<PieceType> {
        if let Some(piece_num) = self.last_hand_num(drop) {
            Some(self.get_type(piece_num))
        } else {
            None
        }
    }

    /// 表示に使うだけ☆（＾～＾）
    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_table<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, Option<&AbsoluteAddress2D>, Option<PieceInfo>),
    {
        for (i, fire) in self.address_list.iter().enumerate() {
            match fire {
                FireAddress::Board(sq) => {
                    // 盤上の駒☆（＾～＾）
                    let piece_info = if let Some(piece_info) = self.piece_info_at1(&fire) {
                        piece_info
                    } else {
                        panic!(Log::print_fatal("(Err.1007) Invalid piece_info."));
                    };
                    piece_get(i, Some(&sq), Some(piece_info));
                }
                FireAddress::Hand(_drop) => {
                    // TODO 持ち駒☆（＾～＾）
                    piece_get(i, None, None);
                }
            }
        }
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    /// TODO 自分、相手で分けて持っておけば２倍ぐらい短縮できないか☆（＾～＾）？
    /// TODO できれば、「自分の盤上の駒」「自分の持ち駒」「相手の盤上の駒」「相手の持ち駒」の４チャンネルで分けておけないか☆（＾～＾）？
    pub fn for_some_pieces_on_list40<F>(&self, turn: Phase, piece_get: &mut F)
    where
        F: FnMut(&FireAddress),
    {
        for piece_num in Nine299792458::piece_numbers().iter() {
            // 盤上の駒だけを調べようぜ☆（＾～＾）
            let fire = self.address_list[*piece_num as usize];
            match fire {
                FireAddress::Board(_sq) => {
                    if self.get_phase(*piece_num) == turn {
                        piece_get(&fire);
                    }
                }
                FireAddress::Hand(_drop) => {
                    // 持ち駒はここで調べるのは無駄な気がするよな☆（＾～＾）持ち駒に歩が１８個とか☆（＾～＾）
                }
            }
        }

        const FIRST_SECOND: [[DoubleFacedPiece; PHYSICAL_PIECE_TYPE_LEN - 1]; 2] = [
            [
                // King なし
                DoubleFacedPiece::Rook1,
                DoubleFacedPiece::Bishop1,
                DoubleFacedPiece::Gold1,
                DoubleFacedPiece::Silver1,
                DoubleFacedPiece::Knight1,
                DoubleFacedPiece::Lance1,
                DoubleFacedPiece::Pawn1,
            ],
            [
                // King なし
                DoubleFacedPiece::Rook2,
                DoubleFacedPiece::Bishop2,
                DoubleFacedPiece::Gold2,
                DoubleFacedPiece::Silver2,
                DoubleFacedPiece::Knight2,
                DoubleFacedPiece::Lance2,
                DoubleFacedPiece::Pawn2,
            ],
        ];
        for drop in &FIRST_SECOND[turn as usize] {
            if !self.is_empty_hand(*drop) {
                piece_get(&FireAddress::Hand(HandAddress::new(
                    *drop,
                    AbsoluteAddress2D::default(),
                ))); // TODO この fire は使い回せないのかだぜ☆（＾～＾）？
            }
        }
    }

    /// 開始地点。
    fn hand_start(double_faced_piece: DoubleFacedPiece) -> isize {
        match double_faced_piece {
            DoubleFacedPiece::King1 => 2,
            DoubleFacedPiece::Rook1 => 103,
            DoubleFacedPiece::Bishop1 => 101,
            DoubleFacedPiece::Gold1 => 6,
            DoubleFacedPiece::Silver1 => 10,
            DoubleFacedPiece::Knight1 => 50,
            DoubleFacedPiece::Lance1 => 90,
            DoubleFacedPiece::Pawn1 => 121,
            DoubleFacedPiece::King2 => 1,
            DoubleFacedPiece::Rook2 => 102,
            DoubleFacedPiece::Bishop2 => 100,
            DoubleFacedPiece::Gold2 => 3,
            DoubleFacedPiece::Silver2 => 7,
            DoubleFacedPiece::Knight2 => 20,
            DoubleFacedPiece::Lance2 => 60,
            DoubleFacedPiece::Pawn2 => 104,
        }
    }
    /// 向き。
    fn hand_direction(double_faced_piece: DoubleFacedPiece) -> isize {
        match double_faced_piece {
            DoubleFacedPiece::King1 => -1,
            DoubleFacedPiece::Rook1 => -1,
            DoubleFacedPiece::Bishop1 => -1,
            DoubleFacedPiece::Gold1 => -1,
            DoubleFacedPiece::Silver1 => -1,
            DoubleFacedPiece::Knight1 => -10,
            DoubleFacedPiece::Lance1 => -10,
            DoubleFacedPiece::Pawn1 => -1,
            DoubleFacedPiece::King2 => 1,
            DoubleFacedPiece::Rook2 => 1,
            DoubleFacedPiece::Bishop2 => 1,
            DoubleFacedPiece::Gold2 => 1,
            DoubleFacedPiece::Silver2 => 1,
            DoubleFacedPiece::Knight2 => 10,
            DoubleFacedPiece::Lance2 => 10,
            DoubleFacedPiece::Pawn2 => 1,
        }
    }

    /// 駒の先後を ひっくり返してから入れてください。
    pub fn push_hand(&mut self, drop: DoubleFacedPiece, num: PieceNum) {
        // 盤上の駒も、持ち駒も区別なく、ゲーム卓に駒を置くぜ☆（＾～＾）
        self.board[self.hand_cur(drop) as usize] = Some(num);
        // 位置を増減するぜ☆（＾～＾）
        self.add_hand_cur(drop, GameTable::hand_direction(drop));
    }
    pub fn pop_hand(&mut self, drop: DoubleFacedPiece) -> PieceNum {
        /* TODO
        if self.count_hand(drop) < 1 {
            panic!(Log::print_fatal_t(
                "(Err.1045) 駒台にない駒を、駒台から取ろうとしました。",
                Table::default()
                    .str("DoubleFacedPiece", &format!("{:?}", drop))
                    .str("GameTable1", &GameTableLook1::to_string(self))
                    .str("GameTable2a", &GameTableLook2a::to_string(&self))
                    .str("GameTable2b", &GameTableLook2b::to_string(&self))
                    .str("GameTable2c", &GameTableLook2c::to_string(&self))
                    .str("Drop", &format!("{:?}", drop))
            ));
        }
        */
        // 位置を増減するぜ☆（＾～＾）
        self.add_hand_cur(drop, -GameTable::hand_direction(drop));
        // 駒台の駒をはがすぜ☆（＾～＾）
        let num = if let Some(num) = self.board[self.hand_cur(drop) as usize] {
            num
        } else {
            panic!(Log::print_fatal_t(
                "(Err.1151) Invalid num.",
                Table::default()
                    .str("DoubleFacedPiece", &format!("{:?}", drop))
                    .str("GameTable1", &GameTableLook1::to_string(self))
                    .str("GameTable2a", &GameTableLook2a::to_string(&self))
                    .str("GameTable2b", &GameTableLook2b::to_string(&self))
                    .str("GameTable2c", &GameTableLook2c::to_string(&self))
                    .str("Drop", &format!("{:?}", drop))
            ));
        };
        self.board[self.hand_cur(drop) as usize] = None;
        num
    }
    pub fn count_hand(&self, drop: DoubleFacedPiece) -> usize {
        if GameTable::hand_direction(drop) < 0 {
            // 先手
            let count = GameTable::hand_start(drop) - self.hand_cur(drop);
            /* TODO
            if drop.type_().hand_max_elements() < count as usize {
                panic!(Log::print_fatal_t(
                    "(Err.1068) 先手の持ち駒が上限枚数を超えています。",
                    Table::default()
                        .usize("HandMaxElements", drop.type_().hand_max_elements())
                        .isize("HandDirection", GameTable::hand_direction(drop))
                        .isize("HandStart", GameTable::hand_start(drop))
                        .isize("HandCur", self.hand_cur(drop))
                        .isize("Count", count)
                        .str("Drop", &format!("{:?}", drop))
                        .str("GameTable1", &GameTableLook1::to_string(self))
                        .str("GameTable2a", &GameTableLook2a::to_string(&self))
                        .str("GameTable2b", &GameTableLook2b::to_string(&self))
                        .str("GameTable2c", &GameTableLook2c::to_string(&self))
                ));
            } else if count < 0 {
                panic!(Log::print_fatal_t(
                    "(Err.1079) 先手の持ち駒が０枚を下回っています。",
                    Table::default()
                        .usize("HandMaxElements", drop.type_().hand_max_elements())
                        .isize("HandDirection", GameTable::hand_direction(drop))
                        .isize("HandStart", GameTable::hand_start(drop))
                        .isize("HandCur", self.hand_cur(drop))
                        .isize("Count", count)
                        .str("Drop", &format!("{:?}", drop))
                        .str("GameTable1", &GameTableLook1::to_string(self))
                        .str("GameTable2a", &GameTableLook2a::to_string(&self))
                        .str("GameTable2b", &GameTableLook2b::to_string(&self))
                        .str("GameTable2c", &GameTableLook2c::to_string(&self))
                ));
            }
            */
            count as usize
        } else {
            let count = self.hand_cur(drop) - GameTable::hand_start(drop);
            /* TODO
            if drop.type_().hand_max_elements() < count as usize {
                panic!(Log::print_fatal_t(
                    "(Err.1094) 後手の持ち駒が上限枚数を超えています。",
                    Table::default()
                        .usize("HandMaxElements", drop.type_().hand_max_elements())
                        .isize("HandDirection", GameTable::hand_direction(drop))
                        .isize("HandStart", GameTable::hand_start(drop))
                        .isize("HandCur", self.hand_cur(drop))
                        .isize("Count", count)
                        .str("Drop", &format!("{:?}", drop))
                        .str("GameTable1", &GameTableLook1::to_string(self))
                        .str("GameTable2a", &GameTableLook2a::to_string(&self))
                        .str("GameTable2b", &GameTableLook2b::to_string(&self))
                        .str("GameTable2c", &GameTableLook2c::to_string(&self))
                ));
            } else if count < 0 {
                panic!(Log::print_fatal_t(
                    "(Err.1105) 先手の持ち駒が０枚を下回っています。",
                    Table::default()
                        .usize("HandMaxElements", drop.type_().hand_max_elements())
                        .isize("HandDirection", GameTable::hand_direction(drop))
                        .isize("HandStart", GameTable::hand_start(drop))
                        .isize("HandCur", self.hand_cur(drop))
                        .isize("Count", count)
                        .str("Drop", &format!("{:?}", drop))
                        .str("GameTable1", &GameTableLook1::to_string(self))
                        .str("GameTable2a", &GameTableLook2a::to_string(&self))
                        .str("GameTable2b", &GameTableLook2b::to_string(&self))
                        .str("GameTable2c", &GameTableLook2c::to_string(&self))
                ));
            }
            */
            count as usize
        }
    }

    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand(&self, drop: DoubleFacedPiece) -> Option<(PieceType, FireAddress)> {
        if let Some(piece_num) = self.last_hand_num(drop) {
            let piece = self.piece_list[piece_num as usize];
            Some((
                piece.type_(),
                FireAddress::Hand(HandAddress::new(
                    piece.double_faced_piece(),
                    AbsoluteAddress2D::default(),
                )),
            ))
        } else {
            None
        }
    }
    pub fn last_hand_num(&self, drop: DoubleFacedPiece) -> Option<PieceNum> {
        let direction = GameTable::hand_direction(drop);
        if direction < 0 {
            // 先手
            if self.hand_cur(drop) < GameTable::hand_start(drop) {
                self.board[(self.hand_cur(drop) - direction) as usize]
            } else {
                None
            }
        } else {
            if GameTable::hand_start(drop) < self.hand_cur(drop) {
                self.board[(self.hand_cur(drop) - direction) as usize]
            } else {
                None
            }
        }
    }

    /// 指し手生成で使うぜ☆（＾～＾）有無を調べるぜ☆（＾～＾）
    pub fn is_empty_hand(&self, drop: DoubleFacedPiece) -> bool {
        //let drop = DoubleFacedPiece::from_phase_and_type(turn, drop.piece.type_());
        if GameTable::hand_direction(drop) < 0 {
            // 先手
            if self.hand_cur(drop) < GameTable::hand_start(drop) {
                false
            } else {
                true
            }
        } else {
            if GameTable::hand_start(drop) < self.hand_cur(drop) {
                false
            } else {
                true
            }
        }
    }

    fn hand_cur(&self, double_faced_piece: DoubleFacedPiece) -> isize {
        match double_faced_piece {
            DoubleFacedPiece::King1 => self.hand_king1_cur,
            DoubleFacedPiece::Rook1 => self.hand_rook1_cur,
            DoubleFacedPiece::Bishop1 => self.hand_bishop1_cur,
            DoubleFacedPiece::Gold1 => self.hand_gold1_cur,
            DoubleFacedPiece::Silver1 => self.hand_silver1_cur,
            DoubleFacedPiece::Knight1 => self.hand_knight1_cur,
            DoubleFacedPiece::Lance1 => self.hand_lance1_cur,
            DoubleFacedPiece::Pawn1 => self.hand_pawn1_cur,
            DoubleFacedPiece::King2 => self.hand_king2_cur,
            DoubleFacedPiece::Rook2 => self.hand_rook2_cur,
            DoubleFacedPiece::Bishop2 => self.hand_bishop2_cur,
            DoubleFacedPiece::Gold2 => self.hand_gold2_cur,
            DoubleFacedPiece::Silver2 => self.hand_silver2_cur,
            DoubleFacedPiece::Knight2 => self.hand_knight2_cur,
            DoubleFacedPiece::Lance2 => self.hand_lance2_cur,
            DoubleFacedPiece::Pawn2 => self.hand_pawn2_cur,
        }
    }
    fn add_hand_cur(&mut self, double_faced_piece: DoubleFacedPiece, direction: isize) {
        match double_faced_piece {
            DoubleFacedPiece::King1 => self.hand_king1_cur += direction,
            DoubleFacedPiece::Rook1 => self.hand_rook1_cur += direction,
            DoubleFacedPiece::Bishop1 => self.hand_bishop1_cur += direction,
            DoubleFacedPiece::Gold1 => self.hand_gold1_cur += direction,
            DoubleFacedPiece::Silver1 => self.hand_silver1_cur += direction,
            DoubleFacedPiece::Knight1 => self.hand_knight1_cur += direction,
            DoubleFacedPiece::Lance1 => self.hand_lance1_cur += direction,
            DoubleFacedPiece::Pawn1 => self.hand_pawn1_cur += direction,
            DoubleFacedPiece::King2 => self.hand_king2_cur += direction,
            DoubleFacedPiece::Rook2 => self.hand_rook2_cur += direction,
            DoubleFacedPiece::Bishop2 => self.hand_bishop2_cur += direction,
            DoubleFacedPiece::Gold2 => self.hand_gold2_cur += direction,
            DoubleFacedPiece::Silver2 => self.hand_silver2_cur += direction,
            DoubleFacedPiece::Knight2 => self.hand_knight2_cur += direction,
            DoubleFacedPiece::Lance2 => self.hand_lance2_cur += direction,
            DoubleFacedPiece::Pawn2 => self.hand_pawn2_cur += direction,
        }
    }
}
