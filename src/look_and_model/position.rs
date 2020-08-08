use crate::cosmic::playing::PosNums;
use crate::cosmic::recording::{FireAddress, HandAddress, Phase};
use crate::cosmic::smart::{features::DoubleFacedPieceType, square::AbsoluteAddress2D};
use crate::look_and_model::piece::PIECE_WHITE_SPACE;
use crate::position::{GameTable, Position};

/// 局面
pub struct PositionLook {}
impl PositionLook {
    /// 表示
    pub fn to_string(pos: &Position, pos_nums: PosNums) -> String {
        let table = pos.get_table(pos_nums);
        let ply = pos.history.ply;
        let phase = pos.history.get_turn();
        let same_pos_count = pos.count_same_position();

        // 局面表示
        format!(
            "\
[{95} ply. {96} phase. {97} repeats.]

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
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Rook,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Bishop,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Gold,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Silver,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Knight,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Lance,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Pawn,
                    AbsoluteAddress2D::default()
                ))
            ),
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Rook,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Bishop,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Gold,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Silver,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Knight,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Lance,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Pawn,
                    AbsoluteAddress2D::default()
                ))
            ),
            ply,
            phase,
            same_pos_count
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

/// ゲーム卓表示1
pub struct GameTableLook {}
impl GameTableLook {
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
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Rook,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Bishop,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Gold,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Silver,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Knight,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Lance,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Pawn,
                    AbsoluteAddress2D::default()
                ))
            ),
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Rook,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Bishop,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Gold,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Silver,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Knight,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Lance,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Pawn,
                    AbsoluteAddress2D::default()
                ))
            ),
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

/// ゲーム卓表示2
pub struct GameTableLook2 {}
impl GameTableLook2 {
    /// 表示
    pub fn to_string(table: &GameTable) -> String {
        // 局面表示
        format!(
            "{}{}{}{}{}{}{}{}{}{}",
            format!(
                "\
[GameTable2]

 12   11   10   9    8    7    6    5    4    3    2    1    0
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|{10}|{11}|{12}|
",
                Self::to_string3(table, 12, 0),
                Self::to_string3(table, 11, 0),
                Self::to_string3(table, 10, 0),
                Self::to_string3(table, 9, 0),
                Self::to_string3(table, 8, 0),
                Self::to_string3(table, 7, 0),
                Self::to_string3(table, 6, 0),
                Self::to_string3(table, 5, 0),
                Self::to_string3(table, 4, 0),
                Self::to_string3(table, 3, 0),
                Self::to_string3(table, 2, 0),
                Self::to_string3(table, 1, 0),
                Self::to_string3(table, 0, 0),
            ),
            format!(
                "\
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|{10}|{11}|{12}|
",
                Self::to_string3(table, 12, 1),
                Self::to_string3(table, 11, 1),
                Self::to_string3(table, 10, 1),
                Self::to_string3(table, 9, 1),
                Self::to_string3(table, 8, 1),
                Self::to_string3(table, 7, 1),
                Self::to_string3(table, 6, 1),
                Self::to_string3(table, 5, 1),
                Self::to_string3(table, 4, 1),
                Self::to_string3(table, 3, 1),
                Self::to_string3(table, 2, 1),
                Self::to_string3(table, 1, 1),
                Self::to_string3(table, 0, 1),
            ),
            format!(
                "\
+----+----+----+----+----+----+----+----+----+----+----+----+----+
     |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|{10}|{11}|
",
                Self::to_string3(table, 11, 2),
                Self::to_string3(table, 10, 2),
                Self::to_string3(table, 9, 2),
                Self::to_string3(table, 8, 2),
                Self::to_string3(table, 7, 2),
                Self::to_string3(table, 6, 2),
                Self::to_string3(table, 5, 2),
                Self::to_string3(table, 4, 2),
                Self::to_string3(table, 3, 2),
                Self::to_string3(table, 2, 2),
                Self::to_string3(table, 1, 2),
                Self::to_string3(table, 0, 2),
            ),
            format!(
                "\
+----+----+----+----+----+----+----+----+----+----+----+----+
     |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|{10}|{11}|
",
                Self::to_string3(table, 11, 3),
                Self::to_string3(table, 10, 3),
                Self::to_string3(table, 9, 3),
                Self::to_string3(table, 8, 3),
                Self::to_string3(table, 7, 3),
                Self::to_string3(table, 6, 3),
                Self::to_string3(table, 5, 3),
                Self::to_string3(table, 4, 3),
                Self::to_string3(table, 3, 3),
                Self::to_string3(table, 2, 3),
                Self::to_string3(table, 1, 3),
                Self::to_string3(table, 0, 3),
            ),
            format!(
                "\
+----+----+----+----+----+----+----+----+----+----+----+----+
     |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|{10}|{11}|
",
                Self::to_string3(table, 11, 4),
                Self::to_string3(table, 10, 4),
                Self::to_string3(table, 9, 4),
                Self::to_string3(table, 8, 4),
                Self::to_string3(table, 7, 4),
                Self::to_string3(table, 6, 4),
                Self::to_string3(table, 5, 4),
                Self::to_string3(table, 4, 4),
                Self::to_string3(table, 3, 4),
                Self::to_string3(table, 2, 4),
                Self::to_string3(table, 1, 4),
                Self::to_string3(table, 0, 4),
            ),
            format!(
                "\
+----+----+----+----+----+----+----+----+----+----+----+----+
     |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|{10}|{11}|
",
                Self::to_string3(table, 11, 5),
                Self::to_string3(table, 10, 5),
                Self::to_string3(table, 9, 5),
                Self::to_string3(table, 8, 5),
                Self::to_string3(table, 7, 5),
                Self::to_string3(table, 6, 5),
                Self::to_string3(table, 5, 5),
                Self::to_string3(table, 4, 5),
                Self::to_string3(table, 3, 5),
                Self::to_string3(table, 2, 5),
                Self::to_string3(table, 1, 5),
                Self::to_string3(table, 0, 5),
            ),
            format!(
                "\
+----+----+----+----+----+----+----+----+----+----+----+----+
     |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|{10}|{11}|
",
                Self::to_string3(table, 11, 6),
                Self::to_string3(table, 10, 6),
                Self::to_string3(table, 9, 6),
                Self::to_string3(table, 8, 6),
                Self::to_string3(table, 7, 6),
                Self::to_string3(table, 6, 6),
                Self::to_string3(table, 5, 6),
                Self::to_string3(table, 4, 6),
                Self::to_string3(table, 3, 6),
                Self::to_string3(table, 2, 6),
                Self::to_string3(table, 1, 6),
                Self::to_string3(table, 0, 6),
            ),
            format!(
                "\
+----+----+----+----+----+----+----+----+----+----+----+----+
     |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|{10}|{11}|
",
                Self::to_string3(table, 11, 7),
                Self::to_string3(table, 10, 7),
                Self::to_string3(table, 9, 7),
                Self::to_string3(table, 8, 7),
                Self::to_string3(table, 7, 7),
                Self::to_string3(table, 6, 7),
                Self::to_string3(table, 5, 7),
                Self::to_string3(table, 4, 7),
                Self::to_string3(table, 3, 7),
                Self::to_string3(table, 2, 7),
                Self::to_string3(table, 1, 7),
                Self::to_string3(table, 0, 7),
            ),
            format!(
                "\
+----+----+----+----+----+----+----+----+----+----+----+----+
     |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|{10}|{11}|
",
                Self::to_string3(table, 11, 8),
                Self::to_string3(table, 10, 8),
                Self::to_string3(table, 9, 8),
                Self::to_string3(table, 8, 8),
                Self::to_string3(table, 7, 8),
                Self::to_string3(table, 6, 8),
                Self::to_string3(table, 5, 8),
                Self::to_string3(table, 4, 8),
                Self::to_string3(table, 3, 8),
                Self::to_string3(table, 2, 8),
                Self::to_string3(table, 1, 8),
                Self::to_string3(table, 0, 8),
            ),
            format!(
                "\
+----+----+----+----+----+----+----+----+----+----+----+----+
     |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|{9}|{10}|{11}|
",
                Self::to_string3(table, 11, 9),
                Self::to_string3(table, 10, 9),
                Self::to_string3(table, 9, 9),
                Self::to_string3(table, 8, 9),
                Self::to_string3(table, 7, 9),
                Self::to_string3(table, 6, 9),
                Self::to_string3(table, 5, 9),
                Self::to_string3(table, 4, 9),
                Self::to_string3(table, 3, 9),
                Self::to_string3(table, 2, 9),
                Self::to_string3(table, 1, 9),
                Self::to_string3(table, 0, 9),
            ),
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
