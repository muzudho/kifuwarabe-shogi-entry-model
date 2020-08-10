use crate::{
    cosmic::{
        recording::HandAddress,
        smart::square::{BOARD_MEMORY_AREA, RANK10U8, RANK1U8},
    },
    law::{generate_move::Area, speed_of_light::Nine299792458},
    log::LogExt,
    look_and_model::{
        recording::{FireAddress, Movement, Phase},
        AbsoluteAddress2D, DoubleFacedPiece, GameTable, Piece, PieceInfo, PieceNum, PieceType,
        DOUBLE_FACED_PIECE_TYPE_LEN, PIECE_NUM_LEN, PIECE_WHITE_SPACE,
    },
};
use casual_logger::{Log, Table};
use num_traits::FromPrimitive;

impl GameTable {
    /// ゲーム卓表示1
    pub fn pretty1(&self) -> String {
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
            self.to_string1(9, 1),
            self.to_string1(8, 1),
            self.to_string1(7, 1),
            self.to_string1(6, 1),
            self.to_string1(5, 1),
            self.to_string1(4, 1),
            self.to_string1(3, 1),
            self.to_string1(2, 1),
            self.to_string1(1, 1),
            self.to_string1(9, 2),
            self.to_string1(8, 2),
            self.to_string1(7, 2),
            self.to_string1(6, 2),
            self.to_string1(5, 2),
            self.to_string1(4, 2),
            self.to_string1(3, 2),
            self.to_string1(2, 2),
            self.to_string1(1, 2),
            self.to_string1(9, 3),
            self.to_string1(8, 3),
            self.to_string1(7, 3),
            self.to_string1(6, 3),
            self.to_string1(5, 3),
            self.to_string1(4, 3),
            self.to_string1(3, 3),
            self.to_string1(2, 3),
            self.to_string1(1, 3),
            self.to_string1(9, 4),
            self.to_string1(8, 4),
            self.to_string1(7, 4),
            self.to_string1(6, 4),
            self.to_string1(5, 4),
            self.to_string1(4, 4),
            self.to_string1(3, 4),
            self.to_string1(2, 4),
            self.to_string1(1, 4),
            self.to_string1(9, 5),
            self.to_string1(8, 5),
            self.to_string1(7, 5),
            self.to_string1(6, 5),
            self.to_string1(5, 5),
            self.to_string1(4, 5),
            self.to_string1(3, 5),
            self.to_string1(2, 5),
            self.to_string1(1, 5),
            self.to_string1(9, 6),
            self.to_string1(8, 6),
            self.to_string1(7, 6),
            self.to_string1(6, 6),
            self.to_string1(5, 6),
            self.to_string1(4, 6),
            self.to_string1(3, 6),
            self.to_string1(2, 6),
            self.to_string1(1, 6),
            self.to_string1(9, 7),
            self.to_string1(8, 7),
            self.to_string1(7, 7),
            self.to_string1(6, 7),
            self.to_string1(5, 7),
            self.to_string1(4, 7),
            self.to_string1(3, 7),
            self.to_string1(2, 7),
            self.to_string1(1, 7),
            self.to_string1(9, 8),
            self.to_string1(8, 8),
            self.to_string1(7, 8),
            self.to_string1(6, 8),
            self.to_string1(5, 8),
            self.to_string1(4, 8),
            self.to_string1(3, 8),
            self.to_string1(2, 8),
            self.to_string1(1, 8),
            self.to_string1(9, 9),
            self.to_string1(8, 9),
            self.to_string1(7, 9),
            self.to_string1(6, 9),
            self.to_string1(5, 9),
            self.to_string1(4, 9),
            self.to_string1(3, 9),
            self.to_string1(2, 9),
            self.to_string1(1, 9),
            self.count_hand(DoubleFacedPiece::Rook1),
            self.count_hand(DoubleFacedPiece::Bishop1),
            self.count_hand(DoubleFacedPiece::Gold1),
            self.count_hand(DoubleFacedPiece::Silver1),
            self.count_hand(DoubleFacedPiece::Knight1),
            self.count_hand(DoubleFacedPiece::Lance1),
            self.count_hand(DoubleFacedPiece::Pawn1),
            self.count_hand(DoubleFacedPiece::Rook2),
            self.count_hand(DoubleFacedPiece::Bishop2),
            self.count_hand(DoubleFacedPiece::Gold2),
            self.count_hand(DoubleFacedPiece::Silver2),
            self.count_hand(DoubleFacedPiece::Knight2),
            self.count_hand(DoubleFacedPiece::Lance2),
            self.count_hand(DoubleFacedPiece::Pawn2),
        )
    }
    fn to_string1(&self, file: u8, rank: u8) -> String {
        if let Some(piece_info_val) =
            self.piece_info_at1(&FireAddress::Board(AbsoluteAddress2D::new(file, rank)))
        {
            format!("{}", piece_info_val.text1)
        } else {
            PIECE_WHITE_SPACE.to_string()
        }
    }

    /// Board display type 2.
    /// 盤表示２。
    pub fn pretty2a(&self) -> String {
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
                self.to_string2a(0),
                self.to_string2a(1),
                self.to_string2a(2),
                self.to_string2a(3),
                self.to_string2a(4),
                self.to_string2a(10),
                self.to_string2a(11),
                self.to_string2a(12),
                self.to_string2a(13),
                self.to_string2a(14),
                self.to_string2a(20),
                self.to_string2a(21),
                self.to_string2a(22),
                self.to_string2a(23),
                self.to_string2a(24),
                self.to_string2a(30),
                self.to_string2a(31),
                self.to_string2a(32),
                self.to_string2a(33),
                self.to_string2a(34),
                self.to_string2a(40),
                self.to_string2a(41),
                self.to_string2a(42),
                self.to_string2a(43),
                self.to_string2a(44),
                self.to_string2a(50),
                self.to_string2a(51),
                self.to_string2a(52),
                self.to_string2a(53),
                self.to_string2a(54),
                self.to_string2a(60),
                self.to_string2a(61),
                self.to_string2a(62),
                self.to_string2a(63),
                self.to_string2a(64),
                self.to_string2a(70),
                self.to_string2a(71),
                self.to_string2a(72),
                self.to_string2a(73),
                self.to_string2a(74),
                self.to_string2a(80),
                self.to_string2a(81),
                self.to_string2a(82),
                self.to_string2a(83),
                self.to_string2a(84),
                self.to_string2a(90),
                self.to_string2a(91),
                self.to_string2a(92),
                self.to_string2a(93),
                self.to_string2a(94),
                self.to_string2a(100),
                self.to_string2a(101),
                self.to_string2a(102),
                self.to_string2a(103),
                self.to_string2a(104),
                self.to_string2a(110),
                self.to_string2a(111),
                self.to_string2a(112),
                self.to_string2a(113),
                self.to_string2a(114),
                self.to_string2a(120),
                self.to_string2a(121),
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
                self.to_string2a(5),
                self.to_string2a(6),
                self.to_string2a(7),
                self.to_string2a(8),
                self.to_string2a(9),
                self.to_string2a(15),
                self.to_string2a(16),
                self.to_string2a(17),
                self.to_string2a(18),
                self.to_string2a(19),
                self.to_string2a(25),
                self.to_string2a(26),
                self.to_string2a(27),
                self.to_string2a(28),
                self.to_string2a(29),
                self.to_string2a(35),
                self.to_string2a(36),
                self.to_string2a(37),
                self.to_string2a(38),
                self.to_string2a(39),
                self.to_string2a(45),
                self.to_string2a(46),
                self.to_string2a(47),
                self.to_string2a(48),
                self.to_string2a(49),
                self.to_string2a(55),
                self.to_string2a(56),
                self.to_string2a(57),
                self.to_string2a(58),
                self.to_string2a(59),
                self.to_string2a(65),
                self.to_string2a(66),
                self.to_string2a(67),
                self.to_string2a(68),
                self.to_string2a(69),
                self.to_string2a(75),
                self.to_string2a(76),
                self.to_string2a(77),
                self.to_string2a(78),
                self.to_string2a(79),
                self.to_string2a(85),
                self.to_string2a(86),
                self.to_string2a(87),
                self.to_string2a(88),
                self.to_string2a(89),
                self.to_string2a(95),
                self.to_string2a(96),
                self.to_string2a(97),
                self.to_string2a(98),
                self.to_string2a(99),
                self.to_string2a(105),
                self.to_string2a(106),
                self.to_string2a(107),
                self.to_string2a(108),
                self.to_string2a(109),
                self.to_string2a(115),
                self.to_string2a(116),
                self.to_string2a(117),
                self.to_string2a(118),
                self.to_string2a(119),
            )
        )
    }
    fn to_string2a(&self, serial: u8) -> String {
        if let Some(sq) = AbsoluteAddress2D::from_absolute_address(serial) {
            if let Some(piece_info_val) = self.piece_info_num_at(&FireAddress::Board(sq)) {
                format!("{}", piece_info_val.text1).to_string()
            } else {
                "    ".to_string()
            }
        } else {
            // 0 は None.
            "    ".to_string()
        }
    }
    /// Address display type 2 of all pieces.
    /// 全ての駒の番地。
    pub fn pretty2b(&self) -> String {
        // 局面表示
        // フォーマットの引数は 98個まで。
        format!(
            "{}{}{}{}",
            format!(
                " 01K  02K  03G  04G  05G  06G  07S  08S  09S  10S  11N  12N  13N  14N  15L  16L  17L  18L  19B  20B
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
",
                self.to_string2b1( PieceNum::King1),
                self.to_string2b1( PieceNum::King2),
                self.to_string2b1( PieceNum::Gold3),
                self.to_string2b1( PieceNum::Gold4),
                self.to_string2b1( PieceNum::Gold5),
                self.to_string2b1( PieceNum::Gold6),
                self.to_string2b1( PieceNum::Silver7),
                self.to_string2b1( PieceNum::Silver8),
                self.to_string2b1( PieceNum::Silver9),
                self.to_string2b1( PieceNum::Silver10),
                self.to_string2b1( PieceNum::Knight11),
                self.to_string2b1( PieceNum::Knight12),
                self.to_string2b1( PieceNum::Knight13),
                self.to_string2b1( PieceNum::Knight14),
                self.to_string2b1( PieceNum::Lance15),
                self.to_string2b1( PieceNum::Lance16),
                self.to_string2b1( PieceNum::Lance17),
                self.to_string2b1( PieceNum::Lance18),
                self.to_string2b1( PieceNum::Bishop19),
                self.to_string2b1( PieceNum::Bishop20),
            ),
            format!(
                "\
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
",
                self.to_string2b2( PieceNum::King1),
                self.to_string2b2( PieceNum::King2),
                self.to_string2b2( PieceNum::Gold3),
                self.to_string2b2( PieceNum::Gold4),
                self.to_string2b2( PieceNum::Gold5),
                self.to_string2b2( PieceNum::Gold6),
                self.to_string2b2( PieceNum::Silver7),
                self.to_string2b2( PieceNum::Silver8),
                self.to_string2b2( PieceNum::Silver9),
                self.to_string2b2( PieceNum::Silver10),
                self.to_string2b2( PieceNum::Knight11),
                self.to_string2b2( PieceNum::Knight12),
                self.to_string2b2( PieceNum::Knight13),
                self.to_string2b2( PieceNum::Knight14),
                self.to_string2b2( PieceNum::Lance15),
                self.to_string2b2( PieceNum::Lance16),
                self.to_string2b2( PieceNum::Lance17),
                self.to_string2b2( PieceNum::Lance18),
                self.to_string2b2( PieceNum::Bishop19),
                self.to_string2b2( PieceNum::Bishop20),
            ),
            format!(
                " 21R  22R  23P  24P  25P  26P  27P  28P  29P  30P  31P  32P  33P  34P  35P  36P  37P  38P  39P  40P
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
",
                self.to_string2b1( PieceNum::Rook21),
                self.to_string2b1( PieceNum::Rook22),
                self.to_string2b1( PieceNum::Pawn23),
                self.to_string2b1( PieceNum::Pawn24),
                self.to_string2b1( PieceNum::Pawn25),
                self.to_string2b1( PieceNum::Pawn26),
                self.to_string2b1( PieceNum::Pawn27),
                self.to_string2b1( PieceNum::Pawn28),
                self.to_string2b1( PieceNum::Pawn29),
                self.to_string2b1( PieceNum::Pawn30),
                self.to_string2b1( PieceNum::Pawn31),
                self.to_string2b1( PieceNum::Pawn32),
                self.to_string2b1( PieceNum::Pawn33),
                self.to_string2b1( PieceNum::Pawn34),
                self.to_string2b1( PieceNum::Pawn35),
                self.to_string2b1( PieceNum::Pawn36),
                self.to_string2b1( PieceNum::Pawn37),
                self.to_string2b1( PieceNum::Pawn38),
                self.to_string2b1( PieceNum::Pawn39),
                self.to_string2b1( PieceNum::Pawn40),
            ),
            format!(
                "\
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
",
                self.to_string2b2( PieceNum::Rook21),
                self.to_string2b2( PieceNum::Rook22),
                self.to_string2b2( PieceNum::Pawn23),
                self.to_string2b2( PieceNum::Pawn24),
                self.to_string2b2( PieceNum::Pawn25),
                self.to_string2b2( PieceNum::Pawn26),
                self.to_string2b2( PieceNum::Pawn27),
                self.to_string2b2( PieceNum::Pawn28),
                self.to_string2b2( PieceNum::Pawn29),
                self.to_string2b2( PieceNum::Pawn30),
                self.to_string2b2( PieceNum::Pawn31),
                self.to_string2b2( PieceNum::Pawn32),
                self.to_string2b2( PieceNum::Pawn33),
                self.to_string2b2( PieceNum::Pawn34),
                self.to_string2b2( PieceNum::Pawn35),
                self.to_string2b2( PieceNum::Pawn36),
                self.to_string2b2( PieceNum::Pawn37),
                self.to_string2b2( PieceNum::Pawn38),
                self.to_string2b2( PieceNum::Pawn39),
                self.to_string2b2( PieceNum::Pawn40),
            ),
        )
    }
    fn to_string2b1(&self, piece_num: PieceNum) -> String {
        if let Some(piece_info_val) = self.piece_info_address_at(piece_num) {
            format!("{: >4}", piece_info_val.text1).to_string()
        } else {
            "    ".to_string()
        }
    }
    fn to_string2b2(&self, piece_num: PieceNum) -> String {
        if let Some(piece_info_val) = self.piece_info_piece_at(piece_num) {
            format!("{: >4}", piece_info_val.text1).to_string()
        } else {
            "    ".to_string()
        }
    }

    /// Address display type 2 of hand piece peak.
    /// 全ての持ち駒の次の番地。
    pub fn pretty2c(&self) -> String {
        // 局面表示
        // フォーマットの引数は 98個まで。
        format!(
            " K2   K1   G2   G1   S2   S1   N2   N1   L2   L1   B2   B1   R2   R1   P2   P1
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
",
            self.to_string2c(DoubleFacedPiece::King2),
            self.to_string2c(DoubleFacedPiece::King1),
            self.to_string2c(DoubleFacedPiece::Gold2),
            self.to_string2c(DoubleFacedPiece::Gold1),
            self.to_string2c(DoubleFacedPiece::Silver2),
            self.to_string2c(DoubleFacedPiece::Silver1),
            self.to_string2c(DoubleFacedPiece::Knight2),
            self.to_string2c(DoubleFacedPiece::Knight1),
            self.to_string2c(DoubleFacedPiece::Lance2),
            self.to_string2c(DoubleFacedPiece::Lance1),
            self.to_string2c(DoubleFacedPiece::Bishop2),
            self.to_string2c(DoubleFacedPiece::Bishop1),
            self.to_string2c(DoubleFacedPiece::Rook2),
            self.to_string2c(DoubleFacedPiece::Rook1),
            self.to_string2c(DoubleFacedPiece::Pawn2),
            self.to_string2c(DoubleFacedPiece::Pawn1),
        )
    }
    fn to_string2c(&self, piece: DoubleFacedPiece) -> String {
        format!("{: >4}", self.hand_next(piece)).to_string()
    }

    /// Address display type 2d of hand piece count.
    /// 全ての持ち駒の枚数。
    pub fn pretty2d(&self) -> String {
        format!(
            " K2   K1   G2   G1   S2   S1   N2   N1   L2   L1   B2   B1   R2   R1   P2   P1
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
",
            self.to_string2d(DoubleFacedPiece::King2),
            self.to_string2d(DoubleFacedPiece::King1),
            self.to_string2d(DoubleFacedPiece::Gold2),
            self.to_string2d(DoubleFacedPiece::Gold1),
            self.to_string2d(DoubleFacedPiece::Silver2),
            self.to_string2d(DoubleFacedPiece::Silver1),
            self.to_string2d(DoubleFacedPiece::Knight2),
            self.to_string2d(DoubleFacedPiece::Knight1),
            self.to_string2d(DoubleFacedPiece::Lance2),
            self.to_string2d(DoubleFacedPiece::Lance1),
            self.to_string2d(DoubleFacedPiece::Bishop2),
            self.to_string2d(DoubleFacedPiece::Bishop1),
            self.to_string2d(DoubleFacedPiece::Rook2),
            self.to_string2d(DoubleFacedPiece::Rook1),
            self.to_string2d(DoubleFacedPiece::Pawn2),
            self.to_string2d(DoubleFacedPiece::Pawn1),
        )
    }
    fn to_string2d(&self, piece: DoubleFacedPiece) -> String {
        format!("{: >4}", self.count_hand(piece)).to_string()
    }
}

impl Default for GameTable {
    fn default() -> Self {
        GameTable {
            // 盤上
            board: [None; BOARD_MEMORY_AREA],
            /// 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
            address_list: [FireAddress::default(); PIECE_NUM_LEN],
            /// 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
            piece_list: [Piece::King1; PIECE_NUM_LEN],
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
            hand_next: [
                DoubleFacedPiece::King1.hand_start(),
                DoubleFacedPiece::Rook1.hand_start(),
                DoubleFacedPiece::Bishop1.hand_start(),
                DoubleFacedPiece::Gold1.hand_start(),
                DoubleFacedPiece::Silver1.hand_start(),
                DoubleFacedPiece::Knight1.hand_start(),
                DoubleFacedPiece::Lance1.hand_start(),
                DoubleFacedPiece::Pawn1.hand_start(),
                DoubleFacedPiece::King2.hand_start(),
                DoubleFacedPiece::Rook2.hand_start(),
                DoubleFacedPiece::Bishop2.hand_start(),
                DoubleFacedPiece::Gold2.hand_start(),
                DoubleFacedPiece::Silver2.hand_start(),
                DoubleFacedPiece::Knight2.hand_start(),
                DoubleFacedPiece::Lance2.hand_start(),
                DoubleFacedPiece::Pawn2.hand_start(),
            ],
        }
    }
}
impl GameTable {
    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, table: &GameTable) {
        self.board = table.board.clone();
        self.address_list = table.address_list.clone();
        self.piece_list = table.piece_list.clone();
        self.double_faced_piece_type_index = table.double_faced_piece_type_index.clone();
        self.area = table.area.clone();
        self.hand_next = table.hand_next;
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
    pub fn rotate_piece_board_to_hand_on_do(&mut self, move_: &Movement) {
        // Captured piece number.
        // 衝突された駒の背番号。
        if let Some(cap) = self.pop_piece(&move_.destination) {
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
                                .str("GameTable1", &self.pretty1())
                                .str("GameTable2a", &self.pretty2a())
                                .str("GameTable2b", &self.pretty2b())
                                .str("GameTable2c", &self.pretty2c())
                                .str("GameTable2d", &self.pretty2d())
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
                        if let Some(piece_num) = self.pop_piece(&fire1) {
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
    pub fn pop_piece(&mut self, fire: &FireAddress) -> Option<PieceNum> {
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
                // let drop = DoubleFacedPiece::from_phase_and_type(turn, drop.piece.type_());
                let piece_num = self.pop_hand(drop.piece);
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
            FireAddress::Hand(drop) => self.last_hand_num(drop.piece),
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

        const FIRST_SECOND: [[DoubleFacedPiece; DOUBLE_FACED_PIECE_TYPE_LEN - 1]; 2] = [
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

    /// 駒の先後を ひっくり返してから入れてください。
    pub fn push_hand(&mut self, drop: DoubleFacedPiece, num: PieceNum) {
        let next = self.hand_next(drop);
        let start = drop.hand_start();
        if next != start {
            let direction = drop.hand_direction();
            let last = next - direction;
            if let Some(_num) = self.board[last as usize] {
            } else {
                panic!(Log::print_fatal_t(
                    "(Err.993) 駒台に連続せずに駒を置こうとしました。",
                    Table::default()
                        .str("GameTable1", &self.pretty1())
                        .str("GameTable2a", &self.pretty2a())
                        .str("GameTable2b", &self.pretty2b())
                        .str("GameTable2c", &self.pretty2c())
                        .str("GameTable2d", &self.pretty2d())
                        .str("Drop", &format!("{:?}", drop))
                        .isize("Next", next)
                        .isize("Last", last)
                        .isize("Start", start)
                        .isize("Direction", direction)
                ));
            }
        }
        // 盤上の駒も、持ち駒も区別なく、ゲーム卓に駒を置くぜ☆（＾～＾）
        self.board[next as usize] = Some(num);
        // 位置を増減するぜ☆（＾～＾）
        self.increase_hand_next(drop);
    }
    pub fn pop_hand(&mut self, drop: DoubleFacedPiece) -> PieceNum {
        let count = self.count_hand(drop);
        if count < 1 {
            panic!(Log::print_fatal_t(
                "(Err.1045) 駒台にない駒を、駒台から取ろうとしました。",
                Table::default()
                    .str("GameTable1", &self.pretty1())
                    .str("GameTable2a", &self.pretty2a())
                    .str("GameTable2b", &self.pretty2b())
                    .str("GameTable2c", &self.pretty2c())
                    .str("GameTable2d", &self.pretty2d())
                    .str("Drop", &format!("{:?}", drop))
                    .usize("Count", count)
            ));
        }
        // 位置を増減するぜ☆（＾～＾）
        self.decrease_hand_next(drop);
        let next = self.hand_next(drop);
        // 駒台の駒をはがすぜ☆（＾～＾）
        let num = if let Some(num) = self.board[next as usize] {
            num
        } else {
            panic!(Log::print_fatal_t(
                "(Err.1151) Not founc piece.",
                Table::default()
                    .str("GameTable1", &self.pretty1())
                    .str("GameTable2a", &self.pretty2a())
                    .str("GameTable2b", &self.pretty2b())
                    .str("GameTable2c", &self.pretty2c())
                    .str("GameTable2d", &self.pretty2d())
                    .str("Drop", &format!("{:?}", drop))
                    .isize("Next", next)
            ));
        };
        self.board[next as usize] = None;
        num
    }
    pub fn count_hand(&self, drop: DoubleFacedPiece) -> usize {
        if drop.hand_direction() < 0 {
            // 先手
            let count = (drop.hand_start() - self.hand_next(drop)) / drop.hand_direction().abs();
            if drop.type_().hand_max_elements() < count as usize {
                panic!(Log::print_fatal_t(
                    "(Err.1068) 先手の持ち駒が上限枚数を超えています。",
                    Table::default()
                        .usize("HandMaxElements", drop.type_().hand_max_elements())
                        .isize("HandDirection", drop.hand_direction())
                        .isize("HandStart", drop.hand_start())
                        .isize("HandNext", self.hand_next(drop))
                        .isize("Count", count)
                        .str("Drop", &format!("{:?}", drop)) // pretty1() は循環参照。
                        .str("GameTable2a", &self.pretty2a()) //.str("GameTable2b", &GameTableLook2b::to_string(&self)) //.str("GameTable2c", &GameTableLook2c::to_string(&self))
                ));
            } else if count < 0 {
                panic!(Log::print_fatal_t(
                    "(Err.1079) 先手の持ち駒が０枚を下回っています。",
                    Table::default()
                        .usize("HandMaxElements", drop.type_().hand_max_elements())
                        .isize("HandDirection", drop.hand_direction())
                        .isize("HandStart", drop.hand_start())
                        .isize("HandNext", self.hand_next(drop))
                        .isize("Count", count)
                        .str("Drop", &format!("{:?}", drop)) // pretty1() は循環参照。
                        .str("GameTable2a", &self.pretty2a()) //.str("GameTable2b", &GameTableLook2b::to_string(&self))
                                                              //.str("GameTable2c", &GameTableLook2c::to_string(&self))
                ));
            }
            count as usize
        } else {
            let count = (self.hand_next(drop) - drop.hand_start()) / drop.hand_direction().abs();
            if drop.type_().hand_max_elements() < count as usize {
                panic!(Log::print_fatal_t(
                    "(Err.1094) 後手の持ち駒が上限枚数を超えています。",
                    Table::default()
                        .usize("HandMaxElements", drop.type_().hand_max_elements())
                        .isize("HandDirection", drop.hand_direction())
                        .isize("HandStart", drop.hand_start())
                        .isize("HandNext", self.hand_next(drop))
                        .isize("Count", count)
                        .str("Drop", &format!("{:?}", drop)) // pretty1() は循環参照。
                        .str("GameTable2a", &self.pretty2a()) //.str("GameTable2b", &GameTableLook2b::to_string(&self)) //.str("GameTable2c", &GameTableLook2c::to_string(&self))
                ));
            } else if count < 0 {
                panic!(Log::print_fatal_t(
                    "(Err.1105) 先手の持ち駒が０枚を下回っています。",
                    Table::default()
                        .usize("HandMaxElements", drop.type_().hand_max_elements())
                        .isize("HandDirection", drop.hand_direction())
                        .isize("HandStart", drop.hand_start())
                        .isize("HandNext", self.hand_next(drop))
                        .isize("Count", count)
                        .str("Drop", &format!("{:?}", drop)) // pretty1() は循環参照。
                        .str("GameTable2a", &self.pretty2a()) //.str("GameTable2b", &GameTableLook2b::to_string(&self))
                                                              //.str("GameTable2c", &GameTableLook2c::to_string(&self))
                ));
            }
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
        let direction = drop.hand_direction();
        if direction < 0 {
            // 先手
            if drop.hand_start() <= self.hand_next(drop) {
                None
            } else {
                self.board[(self.hand_next(drop) - direction) as usize]
            }
        } else {
            if self.hand_next(drop) <= drop.hand_start() {
                None
            } else {
                self.board[(self.hand_next(drop) - direction) as usize]
            }
        }
    }

    /// 指し手生成で使うぜ☆（＾～＾）有無を調べるぜ☆（＾～＾）
    pub fn is_empty_hand(&self, drop: DoubleFacedPiece) -> bool {
        if drop.hand_direction() < 0 {
            // 先手
            drop.hand_start() <= self.hand_next(drop)
        } else {
            self.hand_next(drop) <= drop.hand_start()
        }
    }

    fn hand_next(&self, double_faced_piece: DoubleFacedPiece) -> isize {
        self.hand_next[double_faced_piece as usize]
    }
    fn increase_hand_next(&mut self, drop: DoubleFacedPiece) {
        self.hand_next[drop as usize] += drop.hand_direction();
    }
    fn decrease_hand_next(&mut self, drop: DoubleFacedPiece) {
        if drop.hand_direction() < 0 {
            // 先手
            if drop.hand_start() <= self.hand_next[drop as usize] {
                panic!(Log::print_fatal_t(
                    "(Err.1169) 先手の無い持ち駒をさらに減らそうとしました。",
                    Table::default()
                        .usize("HandMaxElements", drop.type_().hand_max_elements())
                        .isize("HandDirection", drop.hand_direction())
                        .isize("HandStart", drop.hand_start())
                        .isize("HandNext", self.hand_next(drop))
                        .str("Drop", &format!("{:?}", drop)) // pretty1() は循環参照。
                        .str("GameTable2a", &self.pretty2a()) //.str("GameTable2b", &GameTableLook2b::to_string(&self))
                                                              //.str("GameTable2c", &GameTableLook2c::to_string(&self))
                ));
            }
        } else {
            if self.hand_next[drop as usize] <= drop.hand_start() {
                panic!(Log::print_fatal_t(
                    "(Err.1183) 後手の無い持ち駒をさらに減らそうとしました。",
                    Table::default()
                        .usize("HandMaxElements", drop.type_().hand_max_elements())
                        .isize("HandDirection", drop.hand_direction())
                        .isize("HandStart", drop.hand_start())
                        .isize("HandNext", self.hand_next(drop))
                        .str("Drop", &format!("{:?}", drop)) // pretty1() は循環参照。
                        .str("GameTable2a", &self.pretty2a()) //.str("GameTable2b", &GameTableLook2b::to_string(&self))
                                                              //.str("GameTable2c", &GameTableLook2c::to_string(&self))
                ));
            }
        }
        self.hand_next[drop as usize] -= drop.hand_direction();
    }
}
