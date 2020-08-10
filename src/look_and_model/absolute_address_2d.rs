use crate::cosmic::square::{RelAdr2D, FILE0U8, FILE13U8, RANK0U8, RANK10U8};
use crate::law::cryptographic::num_to_lower_case;
use crate::look_and_model::AbsoluteAddress2D;
use std::fmt;

impl Default for AbsoluteAddress2D {
    /// ゴミの値を作るぜ☆（＾～＾）
    fn default() -> Self {
        AbsoluteAddress2D { serial: 11 }
    }
}
impl AbsoluteAddress2D {
    pub fn new(file: u8, rank: u8) -> Self {
        debug_assert!(FILE0U8 <= file && file < FILE13U8, format!("file={}", file));
        debug_assert!(
            RANK0U8 <= rank && rank < RANK10U8 as u8,
            format!("rank={}", rank)
        );
        AbsoluteAddress2D {
            serial: 10 * file as u8 + rank as u8,
        }
    }

    pub fn from_absolute_address(serial: u8) -> Option<AbsoluteAddress2D> {
        let file = serial / 10; // 12 列まであるぜ☆（＾～＾） // (serial / 10) % 10
        let rank = serial % 10;
        if serial == 0 {
            None
        } else {
            debug_assert!(FILE0U8 <= file && file < FILE13U8, format!("file={}", file));
            debug_assert!(RANK0U8 <= rank && rank < RANK10U8, format!("rank={}", rank));
            Some(AbsoluteAddress2D::new(file, rank))
        }
    }

    /// 列番号。いわゆる筋。右から 1, 2, 3 ...
    pub fn file(&self) -> u8 {
        self.serial / 10 // 12 列まであるぜ☆（＾～＾） // (self.serial / 10) % 10
    }

    /// 行番号。いわゆる段。上から 1, 2, 3 ...
    pub fn rank(&self) -> u8 {
        self.serial % 10
    }

    pub fn to_file_rank(&self) -> (u8, u8) {
        (self.file(), self.rank())
    }

    /*
    pub fn rotate_180(&self) -> Self {
        let file = FILE10U8 - self.file();
        let rank = RANK10U8 - self.rank();
        debug_assert!(FILE0U8 < file && file < FILE10U8, format!("file={}", file));
        debug_assert!(RANK0U8 < rank && rank < RANK10U8, format!("rank={}", rank));
        AbsoluteAddress2D::new(file, rank)
    }
    */

    /// 壁の中にいる☆（＾～＾）
    pub fn wall(&self) -> bool {
        self.file() % 10 == 0 || self.rank() % 10 == 0
    }

    /// 連番
    pub fn serial_number(&self) -> u8 {
        self.serial
    }

    pub fn offset(&mut self, r: &RelAdr2D) -> &mut Self {
        // TODO rankの符号はどうだったか……☆（＾～＾） 絶対番地の使い方をしてれば問題ないだろ☆（＾～＾）
        // TODO sum は負数になることもあり、そのときは明らかにイリーガルだぜ☆（＾～＾）
        self.serial = (self.serial_number() as isize + r.get_address()) as u8;
        self
    }
}
/// USI向け。
impl fmt::Display for AbsoluteAddress2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.file(),
            num_to_lower_case(self.rank() as usize),
        )
    }
}
impl fmt::Debug for AbsoluteAddress2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}x {}y {}adr)",
            self.file(),
            self.rank(),
            self.serial_number()
        )
    }
}
